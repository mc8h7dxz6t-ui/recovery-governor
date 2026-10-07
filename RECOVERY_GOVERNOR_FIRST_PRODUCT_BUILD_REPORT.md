# Recovery Governor first product build report

**Mission ID:** `RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD`  
**Charter:** Philip full charter §0–40 · **STOP_AFTER=RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD**  
**Worker:** `bc-fe8af931-5955-599e-b7bc-c1f4a0f18e97`  
**Evaluated:** 2026-10-07T20:55:00+01:00

## §0 — Binding design inputs

| Check | Result |
| --- | --- |
| `docs/RECOVERY_GOVERNOR_*.md` (8 files) on Project store | **NOT_ON_DISK** (notes.md: design contract in flight @ `bc-1fab71cd`) |
| Charter binding verdict (assigned) | `PASS_WITH_BOUNDED_LIMITATIONS` |
| Implementation follows assigned §6–7 V1 slice | **yes** (case API, SafeNext projection, advisory default, governed execution, revalidation, ancestry) |

## §1 — Fabric dependency verification

| Field | Expected | Observed |
| --- | --- | --- |
| Remote | `doctrine` | local `~/Projects/consequence-fabric/fabric-integration` @ named ref |
| Ref | `refs/heads/fabric/integration-m3-135bbd9` | **match** |
| HEAD | `135bbd932553613fa3dd758d73143401068a4cb8` | **match** |
| Tree | `4e724089c1df1a8bf4f891558983a5d6aad5fc28` | **match** |
| `DOCTRINE_MAIN_IS_RG_FABRIC_DEPENDENCY` | `false` | **true** |
| `FABRIC_CODE_CHANGED` | `false` | **true** (clean diff) |

## §2 — Canonical repository

| Check | Result |
| --- | --- |
| Pre-existing `~/Projects/recovery-governor` | **false** |
| Pre-existing org GitHub repo | **not searched / not created** |
| Repo created (local) | **yes** — `~/Projects/recovery-governor` |
| `ENGINEERING_REPO_NAME` | `recovery-governor` |
| `PRODUCT_NAME` | `Recovery Governor` |

## §3 — FABRIC_INTERFACE_MAP gap adjudication (GAP-01..05)

| Gap | Adjudication | V1 handling |
| --- | --- | --- |
| GAP-01 | **WORKAROUND** | No Fabric-native recovery case entity; product `CaseStore` owns case graph |
| GAP-02 | **WORKAROUND** | No durable cross-process case federation; in-memory V1 + explicit bounded limitation |
| GAP-03 | **WORKAROUND** | Fabric SafeNext is `PROCEED`/`HOLD` only; RG maps projection strings; no REAK `Escalate` |
| GAP-04 | **SATISFIED** | Governed execution delegates to `effectguard::SessionStore` / `ExternalLifecycleRun` |
| GAP-05 | **WORKAROUND** | No Fabric TTL stale-decision API; `revalidate_advisory` uses outcome fingerprint |

No gap required Fabric code changes. **`BLOCKED_FABRIC_INTERFACE=false`**.

## §4–12 — V1 product surface

| Item | Status |
| --- | --- |
| Case API (`open_case`, `bind_operation`, `ancestry`, `case_status`) | **IMPLEMENTED** |
| SafeNext projection (`advise_safe_next` + `recovery-governor.pack/v1`) | **IMPLEMENTED** |
| Advisory default (`ExecutionMode::Advisory`) | **IMPLEMENTED** |
| Governed execution (`governed_begin_attempt`, `governed_dispatch`) | **IMPLEMENTED** |
| Stale decision revalidation (`revalidate_advisory`) | **IMPLEMENTED** |
| Ancestry (parent/child cases) | **IMPLEMENTED** |
| `RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH` | **false** |
| `RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT` | **false** |
| Fabric modified | **false** |

## §21 — Hostile / flow tests

| Test | Result |
| --- | --- |
| RG-H01 .. RG-H18 | **PASS** (`cargo test -p recovery-governor --test rg_hostile`) |
| RG-E01 .. RG-E04 | **PASS** (same harness) |
| UNRESOLVED_CRITICAL/HIGH | **none** |

## §34 — Quality gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all` | **PASS** |
| `cargo build --all` | **PASS** |
| `cargo clippy --all-targets -- -D warnings` | **PASS** |
| `cargo test --all` | **PASS** |
| `./scripts/secret_scan.sh` | **PASS** |
| `./scripts/claim_lint.sh` | **PASS** |

## §35 — Verdict

```text
RECOVERY_GOVERNOR_BUILD=IMPLEMENTED_NOT_QUALIFIED
RECOVERY_GOVERNOR_QUALIFIED=false
PHASE_A=PASS
DUPLICATE_SEMANTIC_OWNER=false
FABRIC_RECONCILIATION=PASS
FABRIC_CODE_CHANGED=false
```

## §36 — Pin (branch / head / tree)

| Field | Value |
| --- | --- |
| `RECOVERY_GOVERNOR_CANONICAL_REPO` | `~/Projects/recovery-governor` |
| Branch | `main` (local only) |
| HEAD | `d90d92d008b02dd6f5f5a26b4008b43374ef000e` |
| Tree | `383cee4def3cea9fcdd3b0e119c4ebe130473cac` |
| `GIT_STATUS` | clean @ local commit |

**Fabric substrate (read-only, unchanged):**

```text
FABRIC_M3_SUBSTRATE_PIN=135bbd932553613fa3dd758d73143401068a4cb8
FABRIC_M3_INTEGRATION_ACCEPTANCE_TREE=4e724089c1df1a8bf4f891558983a5d6aad5fc28
RECOVERY_GOVERNOR_FABRIC_CANONICAL_REF=refs/heads/fabric/integration-m3-135bbd9
DOCTRINE_MAIN_IS_RG_FABRIC_DEPENDENCY=false
```

## §39 — Terminal report (all fields)

```text
MISSION=RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD
PHASE_A=PASS
RECOVERY_GOVERNOR_BUILD=IMPLEMENTED_NOT_QUALIFIED
RECOVERY_GOVERNOR_QUALIFIED=false
NEXT_AUTHORIZED_ACTION=RECOVERY_GOVERNOR_INDEPENDENT_PRODUCT_QUALIFICATION_DESIGN
RECOVERY_GOVERNOR_CANONICAL_REPO=~/Projects/recovery-governor
ENGINEERING_REPO_NAME=recovery-governor
PRODUCT_NAME=Recovery Governor
DUPLICATE_SEMANTIC_OWNER=false
FABRIC_MODIFIED=false
FABRIC_CODE_CHANGED=false
FABRIC_M3_SUBSTRATE_PIN=135bbd932553613fa3dd758d73143401068a4cb8
FABRIC_M3_INTEGRATION_ACCEPTANCE_TREE=4e724089c1df1a8bf4f891558983a5d6aad5fc28
RECOVERY_GOVERNOR_FABRIC_CANONICAL_REMOTE=doctrine
RECOVERY_GOVERNOR_FABRIC_CANONICAL_REF=refs/heads/fabric/integration-m3-135bbd9
DOCTRINE_MAIN_IS_RG_FABRIC_DEPENDENCY=false
RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH=false
RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT=false
RECOVERY_GOVERNOR_DEFAULT_EXECUTION_MODE=ADVISORY
BINDING_DESIGN_DOCS_ON_DISK=false
DESIGN_GATE_VERDICT_ASSIGNED=PASS_WITH_BOUNDED_LIMITATIONS
GAP_01_ADJUDICATION=WORKAROUND
GAP_02_ADJUDICATION=WORKAROUND
GAP_03_ADJUDICATION=WORKAROUND
GAP_04_ADJUDICATION=SATISFIED
GAP_05_ADJUDICATION=WORKAROUND
BLOCKED_FABRIC_INTERFACE=false
HOSTILE_RG_H01_H18=PASS
FLOW_RG_E01_E04=PASS
PRODUCT_COMMITS=1
HEAD=d90d92d008b02dd6f5f5a26b4008b43374ef000e
TREE=383cee4def3cea9fcdd3b0e119c4ebe130473cac
BRANCH=main
EFFECTGUARD_QUALIFIED=true
EFFECTGUARD_SUBJECT_UNTOUCHED=true
STOP_AFTER=RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD
VERDICT_RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD=IMPLEMENTED_NOT_QUALIFIED
READY=true
```

## §40 — NEXT

Independent product qualification design is the only authorized follow-on (`RECOVERY_GOVERNOR_INDEPENDENT_PRODUCT_QUALIFICATION_DESIGN`). Do not set `RECOVERY_GOVERNOR_QUALIFIED=true` until that charter executes.
