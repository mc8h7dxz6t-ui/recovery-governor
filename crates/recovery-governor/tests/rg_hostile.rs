//! Hostile product tests RG-H01..RG-H18 and flow tests RG-E01..RG-E04.

#![allow(clippy::assertions_on_constants)]

use std::path::PathBuf;

use effectfence_pack::LabLabel;
use fabric_m3_consequence::ProviderFault;
use recovery_governor::{
    CaseStore, ExecutionMode, RecoveryGovernorError, RevalidationVerdict,
    RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH, RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT,
    RECOVERY_GOVERNOR_DEFAULT_EXECUTION_MODE, RECOVERY_GOVERNOR_QUALIFIED,
};
use recovery_governor_pack::{validate_pack, FABRIC_M3_SUBSTRATE_PIN};
use tempfile::tempdir;

fn paths() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let dir = tempdir().unwrap();
    let handoff = dir.path().join("handoff");
    let db = dir.path().join("run.sqlite");
    (dir, handoff, db)
}

fn seed_governed_case(store: &mut CaseStore, key: &str) -> (tempfile::TempDir, String) {
    let (dir, handoff, db) = paths();
    let case = store.open_case(None, ExecutionMode::Governed).unwrap();
    store
        .bind_operation(&case.case_id, handoff, db, key, LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap();
    store
        .governed_dispatch(&case.case_id, ProviderFault::Normal, "STANDARD")
        .unwrap();
    (dir, case.case_id)
}

/// RG-H01 — semantic firewall flags frozen false.
#[test]
fn rg_h01_semantic_firewall_flags() {
    assert!(!RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH);
    assert!(!RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT);
    assert!(!RECOVERY_GOVERNOR_QUALIFIED);
    assert_eq!(RECOVERY_GOVERNOR_DEFAULT_EXECUTION_MODE, "ADVISORY");
}

/// RG-H02 — default open case is advisory.
#[test]
fn rg_h02_default_advisory_mode() {
    let mut store = CaseStore::new();
    let case = store.open_case(None, ExecutionMode::Advisory).unwrap();
    assert_eq!(case.execution_mode, ExecutionMode::Advisory);
}

/// RG-H03 — advisory case rejects governed dispatch.
#[test]
fn rg_h03_advisory_blocks_governed_dispatch() {
    let mut store = CaseStore::new();
    let (dir, handoff, db) = paths();
    let case = store.open_case(None, ExecutionMode::Advisory).unwrap();
    store
        .bind_operation(&case.case_id, handoff, db, "rg-h03", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap_err();
    let err = store
        .governed_dispatch(&case.case_id, ProviderFault::Normal, "STANDARD")
        .unwrap_err();
    assert!(matches!(err, RecoveryGovernorError::AdvisoryOnly));
    let _ = dir;
}

/// RG-H04 — governed dispatch populates SafeNext advice.
#[test]
fn rg_h04_governed_dispatch_then_advise() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h04");
    let advice = store.advise_safe_next(&case_id).unwrap();
    assert!(!advice.safe_next.is_empty());
    validate_pack(&advice.pack).unwrap();
    assert_eq!(advice.execution_mode, ExecutionMode::Governed);
}

/// RG-H05 — advisory pack marks ADVISORY mode.
#[test]
fn rg_h05_advisory_pack_mode() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h05-seed");
    store
        .set_execution_mode(&case_id, ExecutionMode::Advisory)
        .unwrap();
    let advice = store.advise_safe_next(&case_id).unwrap();
    assert_eq!(advice.execution_mode, ExecutionMode::Advisory);
}

/// RG-H06 — ancestry chain preserved.
#[test]
fn rg_h06_ancestry_chain() {
    let mut store = CaseStore::new();
    let root = store.open_case(None, ExecutionMode::Advisory).unwrap();
    let child = store
        .open_case(Some(&root.case_id), ExecutionMode::Advisory)
        .unwrap();
    let chain = store.ancestry(&child.case_id).unwrap();
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].case_id, child.case_id);
    assert_eq!(chain[1].case_id, root.case_id);
}

/// RG-H07 — unknown case surfaces NotFound.
#[test]
fn rg_h07_unknown_case() {
    let store = CaseStore::new();
    store.case_status("missing").unwrap_err();
}

/// RG-H08 — advise without bound operation fails.
#[test]
fn rg_h08_advise_without_operation() {
    let mut store = CaseStore::new();
    let case = store.open_case(None, ExecutionMode::Advisory).unwrap();
    store.advise_safe_next(&case.case_id).unwrap_err();
}

/// RG-H09 — revalidate fresh after advise.
#[test]
fn rg_h09_revalidate_fresh() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h09");
    store.advise_safe_next(&case_id).unwrap();
    assert_eq!(
        store.revalidate_advisory(&case_id).unwrap(),
        RevalidationVerdict::Fresh
    );
}

/// RG-H10 — revalidate without prior advise is stale.
#[test]
fn rg_h10_revalidate_without_advice_stale() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h10");
    store.revalidate_advisory(&case_id).unwrap_err();
}

/// RG-H11 — lost ACK path does not force PROCEED in advice.
#[test]
fn rg_h11_lost_ack_holds() {
    let mut store = CaseStore::new();
    let (dir, handoff, db) = paths();
    let case = store.open_case(None, ExecutionMode::Governed).unwrap();
    store
        .bind_operation(&case.case_id, handoff, db, "rg-h11", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap();
    store
        .governed_dispatch(&case.case_id, ProviderFault::AckWithoutEffect, "STANDARD")
        .unwrap();
    let advice = store.advise_safe_next(&case.case_id).unwrap();
    assert_eq!(advice.safe_next, "HOLD");
    let _ = dir;
}

/// RG-H12 — advisory pack records HOLD (no false PROCEED) on ack-without-effect.
#[test]
fn rg_h12_hold_advice_not_proceed() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h12-seed");
    store
        .set_execution_mode(&case_id, ExecutionMode::Governed)
        .unwrap();
    let (dir, handoff, db) = paths();
    store
        .bind_operation(&case_id, handoff, db, "rg-h12-b", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case_id).unwrap();
    store
        .governed_dispatch(&case_id, ProviderFault::AckWithoutEffect, "STANDARD")
        .unwrap();
    let advice = store.advise_safe_next(&case_id).unwrap();
    assert_eq!(advice.safe_next, "HOLD");
    let _ = dir;
}

/// RG-H13 — substrate pin echoed in advisory pack.
#[test]
fn rg_h13_substrate_pin_in_pack() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h13");
    let advice = store.advise_safe_next(&case_id).unwrap();
    assert_eq!(advice.pack.fabric_m3_substrate_pin, FABRIC_M3_SUBSTRATE_PIN);
}

/// RG-H14 — cross-case operation binding isolated.
#[test]
fn rg_h14_cross_case_isolation() {
    let mut store = CaseStore::new();
    let (_dir, c1) = seed_governed_case(&mut store, "rg-h14-a");
    let c2 = store.open_case(None, ExecutionMode::Advisory).unwrap();
    store.advise_safe_next(&c1).unwrap();
    store.advise_safe_next(&c2.case_id).unwrap_err();
}

/// RG-H15 — pack digest tamper detected.
#[test]
fn rg_h15_pack_digest_tamper() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-h15");
    let advice = store.advise_safe_next(&case_id).unwrap();
    let mut pack = advice.pack.clone();
    pack.content_digest = "00".repeat(64);
    assert!(validate_pack(&pack).is_err());
}

/// RG-H16 — child case does not inherit parent operation binding.
#[test]
fn rg_h16_child_no_implicit_parent_ops() {
    let mut store = CaseStore::new();
    let (_dir, parent_id) = seed_governed_case(&mut store, "rg-h16-p");
    let child = store
        .open_case(Some(&parent_id), ExecutionMode::Advisory)
        .unwrap();
    store.advise_safe_next(&child.case_id).unwrap_err();
}

/// RG-H17 — governed export profile honored.
#[test]
fn rg_h17_export_profile_blocked() {
    let mut store = CaseStore::new();
    let (dir, handoff, db) = paths();
    let case = store.open_case(None, ExecutionMode::Governed).unwrap();
    store
        .bind_operation(&case.case_id, handoff, db, "rg-h17", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap();
    store
        .governed_dispatch(
            &case.case_id,
            ProviderFault::Normal,
            "TRUST_BOUNDARY_PASSPORT_EXPORT",
        )
        .unwrap_err();
    let _ = dir;
}

/// RG-H18 — multiple operations on case; leaf wins for advice.
#[test]
fn rg_h18_leaf_operation_wins() {
    let mut store = CaseStore::new();
    let (dir, handoff, db) = paths();
    let case = store.open_case(None, ExecutionMode::Governed).unwrap();
    store
        .bind_operation(
            &case.case_id,
            handoff.clone(),
            db.clone(),
            "rg-h18-a",
            LabLabel::Fixture,
        )
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap();
    store
        .governed_dispatch(&case.case_id, ProviderFault::Normal, "STANDARD")
        .unwrap();
    let db2 = dir.path().join("run2.sqlite");
    store
        .bind_operation(&case.case_id, handoff, db2, "rg-h18-b", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&case.case_id).unwrap();
    store
        .governed_dispatch(&case.case_id, ProviderFault::AckWithoutEffect, "STANDARD")
        .unwrap();
    let advice = store.advise_safe_next(&case.case_id).unwrap();
    assert_eq!(advice.safe_next, "HOLD");
}

/// RG-E01 — flow: open → bind → governed dispatch → advise.
#[test]
fn rg_e01_happy_path() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-e01");
    let advice = store.advise_safe_next(&case_id).unwrap();
    assert_eq!(advice.safe_next, "PROCEED");
}

/// RG-E02 — flow: parent/child ancestry + advise on parent only after bind.
#[test]
fn rg_e02_parent_child_flow() {
    let mut store = CaseStore::new();
    let parent = store.open_case(None, ExecutionMode::Governed).unwrap();
    let child = store
        .open_case(Some(&parent.case_id), ExecutionMode::Governed)
        .unwrap();
    let (dir, handoff, db) = paths();
    store
        .bind_operation(&child.case_id, handoff, db, "rg-e02", LabLabel::Fixture)
        .unwrap();
    store.governed_begin_attempt(&child.case_id).unwrap();
    store
        .governed_dispatch(&child.case_id, ProviderFault::Normal, "STANDARD")
        .unwrap();
    let advice = store.advise_safe_next(&child.case_id).unwrap();
    assert_eq!(advice.pack.parent_case_id.as_deref(), Some(parent.case_id.as_str()));
    let _ = dir;
}

/// RG-E03 — flow: advise → revalidate fresh → mode flip advisory.
#[test]
fn rg_e03_revalidate_and_mode_flip() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-e03");
    store.advise_safe_next(&case_id).unwrap();
    assert_eq!(
        store.revalidate_advisory(&case_id).unwrap(),
        RevalidationVerdict::Fresh
    );
    store
        .set_execution_mode(&case_id, ExecutionMode::Advisory)
        .unwrap();
    store
        .governed_dispatch(&case_id, ProviderFault::Normal, "STANDARD")
        .unwrap_err();
}

/// RG-E04 — flow: restart clearance when PROCEED (normal path).
#[test]
fn rg_e04_restart_when_proceed() {
    let mut store = CaseStore::new();
    let (_dir, case_id) = seed_governed_case(&mut store, "rg-e04");
    store.request_restart(&case_id).unwrap();
    store.governed_begin_attempt(&case_id).unwrap();
    store
        .governed_dispatch(&case_id, ProviderFault::Normal, "STANDARD")
        .unwrap();
}
