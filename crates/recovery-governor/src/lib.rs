//! Recovery Governor — case-level advisory SafeNext over Consequence Fabric M3.
//!
//! Product name: **Recovery Governor** · Engineering repo: **recovery-governor**

pub mod case;

pub use case::{
    AdvisoryView, AncestryLink, CaseRecord, CaseStore, RecoveryGovernorError, RevalidationVerdict,
};
pub use recovery_governor_pack::{
    AdvisoryPack, ExecutionMode, FABRIC_M3_INTEGRATION_ACCEPTANCE_TREE, FABRIC_M3_QUAL_SUBJECT_PIN,
    FABRIC_M3_SUBSTRATE_PIN, RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH,
    RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT, RECOVERY_GOVERNOR_DEFAULT_EXECUTION_MODE,
    RECOVERY_GOVERNOR_FABRIC_CANONICAL_REF, RECOVERY_GOVERNOR_FABRIC_CANONICAL_REMOTE,
    RECOVERY_GOVERNOR_FABRIC_DEPENDENCY_SCOPE, RECOVERY_GOVERNOR_QUALIFIED,
    RECOVERY_GOVERNOR_USES_FABRIC_SEMANTICS,
};

pub const ENGINEERING_REPO_NAME: &str = "recovery-governor";
pub const PRODUCT_NAME: &str = "Recovery Governor";
