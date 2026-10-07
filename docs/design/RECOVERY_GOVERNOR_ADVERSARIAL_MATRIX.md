# Recovery Governor — adversarial matrix (design)

**Lane:** `RG-P0` (future qual) · **Cases:** `RG-H01`..`RG-H18` minimum  
**Scope:** `PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0`

`PRODUCT_ADVERSARIAL_MATRIX_COMPLETE=true` (design); execution evidence **not authorized**.

## Severity rubric

| Outcome | Policy |
| --- | --- |
| SafeNext / clearance violation | **CRITICAL** |
| Stale snapshot arm | **CRITICAL** |
| Wrong operation binding | **HIGH** |
| UX wording implying retry OK | **HIGH** |
| Playbook parse / unknown id | **MEDIUM** (fail closed) |

## Hostile cases (`RG-H01`..`RG-H18`)

| Case | Intent | Attack method | Primary invariants | Harness sketch |
| --- | --- | --- | --- | --- |
| **RG-H01** | HOLD→PROCEED via playbook | Playbook step tries to emit PROCEED recommendation on HOLD snapshot | INV-01, FW-04 | `evaluate_playbook` on HOLD fixture |
| **RG-H02** | SafeNext echo drift | Mutate internal echo field in test hook | INV-02 | Assert echo == substrate |
| **RG-H03** | UNKNOWN collapsed | Filter treats UNKNOWN as NOT_APPLIED for ARM step | INV-03 | Wiremock lost-ACK fixture |
| **RG-H04** | Uncleared arm | `try_arm_redispatch` without clearance | FW-05, INV-04 | Expect `BlindRetryBlocked` |
| **RG-H05** | Hidden dispatch | Call bridge dispatch from timer without attempt | INV-05 | Static/arch test |
| **RG-H06** | Stale snapshot arm | Ingest v1 HOLD, ingest v2 cleared, arm using v1 | INV-06, INV-07 | Sequence with `observed_at` |
| **RG-H07** | Timer reorder | `tick` before ingest | INV-08 | No arm |
| **RG-H08** | Cross-operation govern | Bind op A, ingest snapshot for B | INV-09 | `WrongOperation` |
| **RG-H09** | Lineage loss | Multi-attempt; audit missing attempt ids | INV-10 | Audit log assertions |
| **RG-H10** | Pack op swap | Tamper pack `operation_id` | INV-11 | Parse reject |
| **RG-H11** | Playbook embeds truth table | Catalog YAML with custom truth rules | INV-12, RG-D-001 | Schema lint |
| **RG-H12** | Catalog drift | Change catalog without hash update | INV-13 | Session metadata check |
| **RG-H13** | Unknown playbook | Random `playbook_id` | INV-14 | Fail closed |
| **RG-H14** | Error → retry OK | Force `FabricLifecycle` error; read UX strings | INV-15, INV-16 | String lint |
| **RG-H15** | Duplicate verify | Call `effectfence verify` inside core | INV-17 | Dependency graph test |
| **RG-H16** | Fork SessionStore | Reimplement EG register/dispatch in RG core | INV-18 | Crate boundary |
| **RG-H17** | Kernel progression claim | Playbook named `progression_engine` | INV-19 | Claim lint |
| **RG-H18** | Restart durability | Simulate process restart mid-playbook | INV-20, §10 | **EXCLUDED** — document NOT_EXECUTED |

## Supplement — error-handling (`RG-E01`..`RG-E04`)

| Case | Condition | Expected |
| --- | --- | --- |
| **RG-E01** | Fabric reconciliation mismatch | No arm; error surfaced |
| **RG-E02** | Bridge unavailable | Fail closed; no timer arm |
| **RG-E03** | Malformed catalog | `UnknownPlaybook` / parse error |
| **RG-E04** | WAIT_READBACK poll error | Remain HOLD; escalate per playbook |

## Fabric vs product fault injection

- **Fabric faults** (lost ACK, readback timeout): use Fabric/EG fixtures — RG must preserve ambiguity.
- **Product faults** (stale cache, wrong binding, playbook override): inject at `GovernorSession` boundary only.

## Future qualification mapping

| Class | Prefix | When authorized |
| --- | --- | --- |
| Deterministic hostility | RG-H*, RG-E* | RGQ-0 execution |
| Real provider | RG-R* (future) | After first build + GitHub token lane |

## Related

- [RECOVERY_GOVERNOR_INVARIANTS.md](./RECOVERY_GOVERNOR_INVARIANTS.md)
- [RECOVERY_GOVERNOR_DESIGN_GATE.md](./RECOVERY_GOVERNOR_DESIGN_GATE.md)
