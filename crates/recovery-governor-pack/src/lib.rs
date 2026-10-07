//! `recovery-governor.pack/v1` — advisory case evidence (non-authoritative over Fabric).

mod claim_flags;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub use claim_flags::*;

pub const PACK_SCHEMA: &str = "recovery-governor.pack/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionMode {
    Advisory,
    Governed,
}

impl ExecutionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ExecutionMode::Advisory => "ADVISORY",
            ExecutionMode::Governed => "GOVERNED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisoryPack {
    pub schema: String,
    pub fabric_m3_substrate_pin: String,
    pub case_id: String,
    pub parent_case_id: Option<String>,
    pub leaf_operation_id: String,
    pub execution_mode: ExecutionMode,
    pub effect_truth: String,
    pub safe_next: String,
    pub decision_fingerprint: String,
    pub substrate_pin_at_advice: String,
    pub content_digest: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PackError {
    #[error("invalid pack schema")]
    InvalidSchema,
    #[error("digest mismatch")]
    DigestMismatch,
    #[error("json: {0}")]
    Json(String),
}

pub fn decision_fingerprint(
    substrate_pin: &str,
    operation_id: &str,
    effect_truth: &str,
    safe_next: &str,
) -> String {
    let mut h = Sha256::new();
    h.update(substrate_pin.as_bytes());
    h.update(operation_id.as_bytes());
    h.update(effect_truth.as_bytes());
    h.update(safe_next.as_bytes());
    hex::encode(h.finalize())
}

#[allow(clippy::too_many_arguments)]
pub fn new_advisory_pack(
    case_id: &str,
    parent_case_id: Option<&str>,
    leaf_operation_id: &str,
    execution_mode: ExecutionMode,
    effect_truth: &str,
    safe_next: &str,
) -> AdvisoryPack {
    let fp = decision_fingerprint(
        FABRIC_M3_SUBSTRATE_PIN,
        leaf_operation_id,
        effect_truth,
        safe_next,
    );
    let mut pack = AdvisoryPack {
        schema: PACK_SCHEMA.to_string(),
        fabric_m3_substrate_pin: FABRIC_M3_SUBSTRATE_PIN.to_string(),
        case_id: case_id.to_string(),
        parent_case_id: parent_case_id.map(|s| s.to_string()),
        leaf_operation_id: leaf_operation_id.to_string(),
        execution_mode,
        effect_truth: effect_truth.to_string(),
        safe_next: safe_next.to_string(),
        decision_fingerprint: fp,
        substrate_pin_at_advice: FABRIC_M3_SUBSTRATE_PIN.to_string(),
        content_digest: String::new(),
    };
    pack.content_digest = content_digest(&pack);
    pack
}

pub fn content_digest(pack: &AdvisoryPack) -> String {
    let mut clone = pack.clone();
    clone.content_digest = String::new();
    let bytes = serde_json::to_vec(&clone).expect("advisory pack serializes");
    hex::encode(Sha256::digest(bytes))
}

pub fn validate_pack(pack: &AdvisoryPack) -> Result<(), PackError> {
    if pack.schema != PACK_SCHEMA {
        return Err(PackError::InvalidSchema);
    }
    if pack.content_digest != content_digest(pack) {
        return Err(PackError::DigestMismatch);
    }
    Ok(())
}

pub fn to_bytes(pack: &AdvisoryPack) -> Result<Vec<u8>, PackError> {
    serde_json::to_vec(pack).map_err(|e| PackError::Json(e.to_string()))
}
