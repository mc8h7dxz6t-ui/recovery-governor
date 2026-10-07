# Recovery Governor — API contract (design v0)

**Scope:** `PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0`  
**Status:** Design sketch — **no stable crate published**  
**Substrate pin:** `FABRIC_DEPENDENCY_HEAD=135bbd932553613fa3dd758d73143401068a4cb8`

## Design principles

1. **Snapshot in, recommendations out** — APIs consume substrate state; they do not mutate Fabric truth.
2. **Clearance is explicit** — arms that enable redispatch are separate methods with Fabric-error passthrough.
3. **Playbooks are versioned data** — `playbook_id` + `catalog_revision` hashed in evidence (future §22 bundle).

## Core types (logical)

### `SubstrateSnapshot`

Read-only view aligned with Fabric `LifecycleOutcome` / EffectGuard session fields:

| Field | Source | Notes |
| --- | --- | --- |
| `operation_id` | Product binding | RG session key |
| `attempt_id` | Fabric / EG | Lineage |
| `provider_profile` | Fabric | e.g. `P-GITHUB-ISSUE-POST` |
| `effect_truth` | Fabric string label | Maps `EffectTruth` enum |
| `safe_next` | Fabric | `PROCEED` \| `HOLD` |
| `dispatch_ack_received` | Fabric | bool |
| `client_ack_lost` | Fabric / transport fault | Drives HOLD |
| `provider_readback` | Fabric label | `EFFECT_CONFIRMED` / `EFFECT_ABSENT` / `READBACK_TIMEOUT` |
| `reconciliation_cleared` | Derived read-only | From Fabric rules; RG must not invent |
| `substrate_pin` | Manifest | `135bbd9` short + profile ids |
| `observed_at` | Product clock | Freshness (§11) |

Ingest paths:

- `SubstrateSnapshot::from_lifecycle_outcome(&LifecycleOutcome)`
- `SubstrateSnapshot::from_effectguard_pack(&PackV1)` (read-only parse)

### `PlaybookRef`

```text
playbook_id: string          # e.g. "default-hold-escalation-v0"
catalog_revision: string     # content hash
profile_filter: string[]     # optional provider profiles
```

### `RecoveryStepKind` (v0 enum)

| Kind | Meaning | Fabric touchpoint |
| --- | --- | --- |
| `NOOP_HOLD` | Wait; surface HOLD to operator | none |
| `WAIT_READBACK` | Schedule poll | `adapter.readback` via Fabric port (THIN_ADAPTER) |
| `ESCALATE_HUMAN` | Emit escalation event | none |
| `ARM_REDISPATCH_IF_CLEARED` | Call restart arm | `allow_redispatch` + lifecycle |
| `INVOKE_EFFECTGUARD_RESTART` | Delegates to EG API | `SessionStore::request_restart` |

### `RecoveryRecommendation`

| Field | Type | Authority |
| --- | --- | --- |
| `step_kind` | `RecoveryStepKind` | Product |
| `rationale_code` | `string` | Product |
| `safe_next_echo` | `PROCEED` \| `HOLD` | **Must equal** snapshot |
| `clearance_required` | `bool` | Product documents |
| `authorized` | `bool` | **false** until clearance arm succeeds |

```text
RG_RECOMMENDATION_IS_NOT_CLEARANCE=true
```

## `GovernorSession` (v0 surface)

| Method | Preconditions | Effects | Errors |
| --- | --- | --- | --- |
| `bind_operation(operation_id)` | unique session | stores binding | `AlreadyBound` |
| `ingest(snapshot)` | bound op id match | updates latest snapshot | `StaleSnapshot`, `WrongOperation` |
| `evaluate_playbook(ref)` | snapshot present | returns `Vec<RecoveryRecommendation>` | `UnknownPlaybook` |
| `tick(now)` | playbook with timers | may enqueue WAIT_READBACK | no silent dispatch |
| `try_arm_redispatch()` | Fabric clearance + HOLD policy | sets bridge `allow_redispatch` | `BlindRetryBlocked` passthrough |
| `escalate(event)` | human step | audit log | — |
| `status()` | — | snapshot + last recommendations | — |

**Forbidden on v0 surface:** `dispatch()` without going through Fabric lifecycle or EffectGuard session bridge.

## Playbook catalog API (v0)

| Endpoint / fn | Purpose |
| --- | --- |
| `load_catalog(bytes) -> Catalog` | Parse YAML/JSON; validate schema |
| `catalog.get(playbook_id)` | Return graph |
| `catalog.hash()` | SHA-256 for evidence bundle |

### Minimal playbook graph node

```yaml
id: wait-then-escalate
when:
  safe_next: HOLD
  effect_truth: [UNKNOWN, SUPPORTED_BY_PROVIDER_ACK]
steps:
  - kind: WAIT_READBACK
    delay_ms: 5000
  - kind: ESCALATE_HUMAN
    if_still: HOLD
```

`when` clauses are **filters** on snapshot fields only — not truth adjudication.

## Bridge crate (recommended layout at build)

```text
recovery-governor-core     # playbook + session (no network)
recovery-governor-bridge   # effectguard + fabric-m3-consequence deps
recovery-governor-cli      # operator tooling (optional v0)
```

## Error taxonomy (`RecoveryGovernorError`)

| Variant | Maps from | User-visible posture |
| --- | --- | --- |
| `FabricLifecycle` | `LifecycleError` | No default “retry OK” |
| `BlindRetryBlocked` | Fabric | Hold + explain |
| `StaleSnapshot` | RG freshness | Re-ingest required |
| `SafeNextMismatch` | RG invariant | Critical fault |
| `BridgeUnavailable` | EG/Fabric missing | Fail closed |

```text
RG_INTERNAL_ERROR_DEFAULTS_TO_RECOVERY_ALLOWED=false
```

## Versioning

```text
RG_API_CONTRACT_VERSION=0.1.0-design
RG_PLAYBOOK_SCHEMA_VERSION=0.1.0
```

Breaking changes require new design scope freeze + subject re-pin.

## Related

- [RECOVERY_GOVERNOR_INVARIANTS.md](./RECOVERY_GOVERNOR_INVARIANTS.md)
- [RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md](./RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md)
