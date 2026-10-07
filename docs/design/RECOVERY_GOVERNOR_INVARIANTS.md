# Recovery Governor — product invariants (design v0)

**Scope:** `PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0`  
**Load-bearing:** violation → future qual **CRITICAL** unless noted HIGH.

## Semantic firewall (charter §7)

| ID | Invariant | Token |
| --- | --- | --- |
| FW-01 | Never mutates Fabric canonical truth | `RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH=false` |
| FW-02 | Never defines new truth states | `RECOVERY_GOVERNOR_CAN_DEFINE_NEW_TRUTH_STATE=false` |
| FW-03 | Never overrides reconciliation clearance | `RECOVERY_GOVERNOR_CAN_OVERRIDE_RECONCILIATION=false` |
| FW-04 | Never overrides SafeNext | `RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT=false` |
| FW-05 | Never arms redispatch without clearance path | `RECOVERY_GOVERNOR_CAN_AUTHORIZE_UNCLEARED_REDISPATCH=false` |
| FW-06 | Never owns provider profile authority | `RECOVERY_GOVERNOR_PROVIDER_PROFILE_AUTHORITY=false` |

## SafeNext and recommendations (charter §12)

| ID | Invariant | Token |
| --- | --- | --- |
| INV-01 | Recommendation never asserts PROCEED when snapshot HOLD | `RG_NEVER_RECOMMENDS_PROCEED_ON_HOLD` |
| INV-02 | `safe_next_echo` always equals snapshot `safe_next` | `RG_SAFE_NEXT_ECHO_MATCHES_SUBSTRATE` |
| INV-03 | Playbook cannot transform UNKNOWN→NOT_APPLIED | `RG_NEVER_COLLAPSES_UNKNOWN_TO_NOT_APPLIED` |
| INV-04 | ARM step errors when Fabric returns `BlindRetryBlocked` | `RG_RESPECTS_BLIND_RETRY_BLOCKED` |
| INV-05 | No hidden second dispatch | `RG_NEVER_HIDES_CONSEQUENTIAL_DISPATCH` |

## Freshness (charter §11)

| ID | Invariant | Token |
| --- | --- | --- |
| INV-06 | Stale snapshot never drives arm/redispatch | `RG_STALE_SNAPSHOT_NEVER_ARMS_REDISPATCH` |
| INV-07 | Orchestration freshness load-bearing | `RG_ORCHESTRATION_FRESHNESS_IS_LOAD_BEARING=true` |
| INV-08 | Tick/timer uses monotonic `observed_at` ordering | `RG_TIMER_RESPECTS_SNAPSHOT_ORDER` |

## Identity binding

| ID | Invariant | Token |
| --- | --- | --- |
| INV-09 | Wrong `operation_id` never evaluates playbook | `RG_WRONG_OPERATION_NEVER_GOVERNS` |
| INV-10 | Attempt lineage preserved in audit log | `RG_ATTEMPT_LINEAGE_PRESERVED` |
| INV-11 | Pack ingest rejects cross-operation ids | `RG_PACK_OPERATION_BINDING_ENFORCED` |

## Playbook engine

| ID | Invariant | Token |
| --- | --- | --- |
| INV-12 | Playbook is policy not truth | `RG_PLAYBOOK_IS_POLICY_NOT_TRUTH=true` |
| INV-13 | Catalog hash pinned in session metadata | `RG_PLAYBOOK_CATALOG_HASH_PINNED` |
| INV-14 | Unknown playbook id fails closed | `RG_UNKNOWN_PLAYBOOK_FAILS_CLOSED` |

## Error defaults (charter §12–§13)

| ID | Invariant | Token |
| --- | --- | --- |
| INV-15 | Internal error never defaults recovery allowed | `RG_INTERNAL_ERROR_DEFAULTS_TO_RECOVERY_ALLOWED=false` |
| INV-16 | Fabric error never mapped to “safe to retry” | `RG_FABRIC_ERROR_NEVER_BECOMES_SAFE_TO_RETRY` |

## Cross-product

| ID | Invariant | Token |
| --- | --- | --- |
| INV-17 | Does not reimplement EffectGuard verify | `RG_DOES_NOT_REPLACE_EFFECTGUARD_VERIFY=true` |
| INV-18 | Does not duplicate EG session semantics | `RG_DOES_NOT_DUPLICATE_EG_SESSION_SEMANTICS=true` |
| INV-19 | Not kernel progression engine | `RG_IS_NOT_KERNEL_PROGRESSION=true` |

## Restart boundary (charter §10)

| ID | Invariant | Notes |
| --- | --- | --- |
| INV-20 | Process restart durability | **EXCLUDED** — design only; no claim |
| INV-21 | Lane E GitHub restart ambiguity | **NOT_EXECUTED** — mirror programme debt |

## ACG graph (future)

| ID | Invariant | Notes |
| --- | --- | --- |
| INV-22 | RecoveryAuthorization graph | Emit only when Fabric exposes writer — else FUTURE |

## Traceability

Hostile cases: [RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md](./RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md).
