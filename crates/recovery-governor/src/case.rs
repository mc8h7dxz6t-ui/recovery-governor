use std::collections::HashMap;
use std::path::PathBuf;

use effectfence_pack::LabLabel;
use effectguard::{EffectGuardError, SessionStore};
use fabric_m3_consequence::ProviderFault;
use recovery_governor_pack::{
    decision_fingerprint, new_advisory_pack, validate_pack, AdvisoryPack, ExecutionMode,
    FABRIC_M3_SUBSTRATE_PIN,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RecoveryGovernorError {
    #[error("case not found: {0}")]
    NotFound(String),
    #[error("operation not bound to case")]
    OperationNotBound,
    #[error("advisory only — governed execution not enabled for case")]
    AdvisoryOnly,
    #[error("stale advisory — revalidate required")]
    StaleAdvisory,
    #[error("invalid attempt state")]
    InvalidAttemptState,
    #[error("effectguard: {0}")]
    EffectGuard(#[from] EffectGuardError),
    #[error("pack: {0}")]
    Pack(#[from] recovery_governor_pack::PackError),
}

#[derive(Debug, Clone)]
pub struct CaseRecord {
    pub case_id: String,
    pub parent_case_id: Option<String>,
    pub child_case_ids: Vec<String>,
    pub execution_mode: ExecutionMode,
    pub operation_ids: Vec<String>,
    pub leaf_operation_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AncestryLink {
    pub case_id: String,
    pub parent_case_id: Option<String>,
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct AdvisoryView {
    pub case_id: String,
    pub safe_next: String,
    pub effect_truth: String,
    pub execution_mode: ExecutionMode,
    pub stale: bool,
    pub pack: AdvisoryPack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevalidationVerdict {
    Fresh,
    Stale,
}

pub struct CaseStore {
    sessions: SessionStore,
    cases: HashMap<String, CaseRecord>,
    operation_case: HashMap<String, String>,
    latest_advisory: HashMap<String, AdvisoryPack>,
}

impl Default for CaseStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CaseStore {
    pub fn new() -> Self {
        Self {
            sessions: SessionStore::new(),
            cases: HashMap::new(),
            operation_case: HashMap::new(),
            latest_advisory: HashMap::new(),
        }
    }

    /// Open a recovery case. Default mode is advisory.
    pub fn open_case(
        &mut self,
        parent_case_id: Option<&str>,
        execution_mode: ExecutionMode,
    ) -> Result<CaseRecord, RecoveryGovernorError> {
        if let Some(parent) = parent_case_id {
            self.cases
                .get(parent)
                .ok_or_else(|| RecoveryGovernorError::NotFound(parent.to_string()))?;
        }
        let case_id = Uuid::new_v4().to_string();
        let record = CaseRecord {
            case_id: case_id.clone(),
            parent_case_id: parent_case_id.map(|s| s.to_string()),
            child_case_ids: Vec::new(),
            execution_mode,
            operation_ids: Vec::new(),
            leaf_operation_id: None,
        };
        if let Some(parent) = parent_case_id {
            self.cases
                .get_mut(parent)
                .expect("parent exists")
                .child_case_ids
                .push(case_id.clone());
        }
        self.cases.insert(case_id.clone(), record);
        Ok(self.cases.get(&case_id).unwrap().clone())
    }

    /// Bind a Fabric operation to the case (ancestry leaf is the last bound operation).
    pub fn bind_operation(
        &mut self,
        case_id: &str,
        handoff_root: PathBuf,
        db_path: PathBuf,
        idempotency_key: &str,
        lab_label: LabLabel,
    ) -> Result<String, RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        let handle = self
            .sessions
            .register(handoff_root, db_path, idempotency_key, lab_label)?;
        let op_id = handle.operation_id.clone();
        self.operation_case
            .insert(op_id.clone(), case_id.to_string());
        let mut updated = case.clone();
        updated.operation_ids.push(op_id.clone());
        updated.leaf_operation_id = Some(op_id.clone());
        self.cases.insert(case_id.to_string(), updated);
        Ok(op_id)
    }

    pub fn ancestry(&self, case_id: &str) -> Result<Vec<AncestryLink>, RecoveryGovernorError> {
        self.cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        let mut links = Vec::new();
        let mut current = Some(case_id.to_string());
        let mut depth = 0u32;
        while let Some(id) = current {
            let rec = self
                .cases
                .get(&id)
                .ok_or_else(|| RecoveryGovernorError::NotFound(id.clone()))?;
            links.push(AncestryLink {
                case_id: id.clone(),
                parent_case_id: rec.parent_case_id.clone(),
                depth,
            });
            current = rec.parent_case_id.clone();
            depth += 1;
        }
        Ok(links)
    }

    pub fn case_status(&self, case_id: &str) -> Result<CaseRecord, RecoveryGovernorError> {
        self.cases
            .get(case_id)
            .cloned()
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))
    }

    /// Flip execution mode after case open (V1 bootstrap — e.g. seed Fabric outcome then return to advisory).
    pub fn set_execution_mode(
        &mut self,
        case_id: &str,
        mode: ExecutionMode,
    ) -> Result<(), RecoveryGovernorError> {
        let case = self
            .cases
            .get_mut(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        case.execution_mode = mode;
        Ok(())
    }

    /// SafeNext projection only — never mutates Fabric truth.
    pub fn advise_safe_next(
        &mut self,
        case_id: &str,
    ) -> Result<AdvisoryView, RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        let leaf = case
            .leaf_operation_id
            .as_ref()
            .ok_or(RecoveryGovernorError::OperationNotBound)?;
        let view = self.sessions.safe_next(leaf)?;
        let outcome = self
            .sessions
            .status(leaf)?
            .latest_attempt
            .and_then(|a| a.effect_truth)
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let pack = new_advisory_pack(
            case_id,
            case.parent_case_id.as_deref(),
            leaf,
            case.execution_mode,
            &outcome,
            &view.decision,
        );
        validate_pack(&pack)?;
        self.latest_advisory
            .insert(case_id.to_string(), pack.clone());
        Ok(AdvisoryView {
            case_id: case_id.to_string(),
            safe_next: view.decision,
            effect_truth: outcome,
            execution_mode: case.execution_mode,
            stale: false,
            pack,
        })
    }

    pub fn revalidate_advisory(
        &self,
        case_id: &str,
    ) -> Result<RevalidationVerdict, RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        let pack = self
            .latest_advisory
            .get(case_id)
            .ok_or(RecoveryGovernorError::StaleAdvisory)?;
        let leaf = case
            .leaf_operation_id
            .as_ref()
            .ok_or(RecoveryGovernorError::OperationNotBound)?;
        let current = self.sessions.safe_next(leaf)?;
        let effect_truth = self
            .sessions
            .status(leaf)?
            .latest_attempt
            .and_then(|a| a.effect_truth)
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let fp = decision_fingerprint(
            FABRIC_M3_SUBSTRATE_PIN,
            leaf,
            &effect_truth,
            &current.decision,
        );
        if fp != pack.decision_fingerprint
            || pack.substrate_pin_at_advice != FABRIC_M3_SUBSTRATE_PIN
        {
            Ok(RevalidationVerdict::Stale)
        } else {
            Ok(RevalidationVerdict::Fresh)
        }
    }

    pub fn governed_begin_attempt(&mut self, case_id: &str) -> Result<(), RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        if case.execution_mode != ExecutionMode::Governed {
            return Err(RecoveryGovernorError::AdvisoryOnly);
        }
        let leaf = case
            .leaf_operation_id
            .as_ref()
            .ok_or(RecoveryGovernorError::OperationNotBound)?;
        self.sessions.begin_attempt(leaf)?;
        Ok(())
    }

    pub fn governed_dispatch(
        &mut self,
        case_id: &str,
        provider_fault: ProviderFault,
        export_profile: &str,
    ) -> Result<effectguard::DispatchOutcome, RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        if case.execution_mode != ExecutionMode::Governed {
            return Err(RecoveryGovernorError::AdvisoryOnly);
        }
        let leaf = case
            .leaf_operation_id
            .as_ref()
            .ok_or(RecoveryGovernorError::OperationNotBound)?;
        Ok(self.sessions.dispatch(leaf, provider_fault, export_profile)?)
    }

    pub fn request_restart(&mut self, case_id: &str) -> Result<(), RecoveryGovernorError> {
        let case = self
            .cases
            .get(case_id)
            .ok_or_else(|| RecoveryGovernorError::NotFound(case_id.to_string()))?;
        let leaf = case
            .leaf_operation_id
            .as_ref()
            .ok_or(RecoveryGovernorError::OperationNotBound)?;
        self.sessions.request_restart(leaf)?;
        Ok(())
    }
}
