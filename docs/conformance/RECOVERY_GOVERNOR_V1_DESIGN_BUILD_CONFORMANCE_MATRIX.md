# Recovery Governor V1 — design / build conformance matrix

**Subject:** `RG_V1_INITIAL_BUILD_SUBJECT` @ `4d6cb05` (tree `e0b433f1`)  
**Design package:** `rg-design-v0-e7aae88949bd74f3` · `DESIGN_FILE_RECONSTRUCTED=false`  
**Audited:** 2026-10-07T21:05:00+01:00 · Worker `bc-fe8af931-5955-599e-b7bc-c1f4a0f18e97`

## Design restoration

| Check | Result |
| --- | --- |
| Eight binding files in `docs/design/` | **PASS** (byte-identical to Project store) |
| `RECOVERY_GOVERNOR_DESIGN_PACKAGE_MANIFEST.json` | **PASS** |
| `DESIGN_PACKAGE_PROVENANCE.json` | **PASS** |

## Semantic firewall (design §7 / invariants FW-*)

| ID | Design token | Build evidence | Verdict |
| --- | --- | --- | --- |
| FW-01 | `RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH=false` | `claim_flags.rs` + RG-H01 | **PASS** |
| FW-02 | `RECOVERY_GOVERNOR_CAN_DEFINE_NEW_TRUTH_STATE=false` | no truth adjudication in `case.rs` | **PASS** |
| FW-03 | `RECOVERY_GOVERNOR_CAN_OVERRIDE_RECONCILIATION=false` | restart via EG only | **PASS** |
| FW-04 | `RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT=false` | `advise_safe_next` reads EG `safe_next` | **PASS** |
| FW-05 | `RECOVERY_GOVERNOR_CAN_AUTHORIZE_UNCLEARED_REDISPATCH=false` | EG `request_restart` / dispatch errors | **PASS** |
| FW-06 | `RECOVERY_GOVERNOR_PROVIDER_PROFILE_AUTHORITY=false` | profile from EG register | **PASS** |

## API contract (design v0)

| Surface | Design | Build @ subject | Verdict |
| --- | --- | --- | --- |
| `GovernorSession` | Core session API | `CaseStore` (case-centric) | **BOUNDED** (CF-002) |
| `SubstrateSnapshot` / `ingest` | Explicit snapshot | Implicit via EG session outcome | **BOUNDED** |
| `evaluate_playbook` / catalog | Required v0 slice | Not present | **BOUNDED** (CF-003) |
| `try_arm_redispatch` | Explicit arm | `request_restart` + governed dispatch | **PARTIAL** |
| `advise_safe_next` / advisory pack | Not named in v0 sketch | `recovery-governor.pack/v1` | **PASS** (V1 extension) |
| Case ancestry | Out of v0 minimum | `open_case` parent + `ancestry` | **BOUNDED** (CF-004) |

## Fabric interface map (GAP-01..05)

| Gap | Design mitigation | Build handling | Verdict |
| --- | --- | --- | --- |
| GAP-01 RECOVER SafeNext | HOLD-only playbooks | HOLD/PROCEED strings only | **PASS** |
| GAP-02 Recovery* API | THIN_ADAPTER | EffectGuard `SessionStore` embed | **PASS** |
| GAP-03 Contradiction enum | No synthesis | Uses EG/Fabric strings | **PASS** |
| GAP-04 Durable RG store | NOT_IN_SCOPE | In-memory `CaseStore` | **PASS** |
| GAP-05 main ≠ integration | Named ref pin | `135bbd9` in pack flags | **PASS** |

## Adversarial matrix (RG-H / RG-E)

| Range | Design intent | Build harness @ `4d6cb05` | Verdict |
| --- | --- | --- | --- |
| RG-H01..H18 | Playbook / session attacks | V1 slice smoke tests (same IDs) | **BOUNDED** (CF-001) |
| RG-E01..E04 | Error / bridge paths | Case flow tests | **BOUNDED** (CF-001) |
| RG-H18 restart | EXCLUDED | Not executed | **PASS** (aligned) |

## Quality @ subject

| Gate | Result |
| --- | --- |
| `cargo test --all` @ `4d6cb05` | **PASS** (22 hostile/flow tests) |
| Fabric HEAD/tree | **PASS** (`135bbd9` / `4e724089`) |
| Fabric / EffectGuard code changed | **false** |

## Overall

```text
RECOVERY_GOVERNOR_V1_DESIGN_BUILD_CONFORMANCE=PASS_WITH_BOUNDED_LIMITATIONS
LOAD_BEARING_MISMATCH=false
```
