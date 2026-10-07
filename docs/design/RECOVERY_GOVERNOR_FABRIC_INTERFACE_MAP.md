# Recovery Governor — Fabric interface map

**Fabric pin:** `135bbd932553613fa3dd758d73143401068a4cb8` (`origin/fabric/integration-m3-135bbd9`)  
**Verification:** read-only `git show` / workspace fetch 2026-10-07  
**Legend:** `AVAILABLE_NOW` · `THIN_ADAPTER` · `MISSING` · `FUTURE`

## Summary

| Class | Count | Notes |
| --- | --- | --- |
| AVAILABLE_NOW | 6 | Rust types + lifecycle outcome fields |
| THIN_ADAPTER | 5 | Requires bridge to EG session or sandbox run |
| MISSING | 4 | Design docs vs code gaps |
| FUTURE | 5 | ACG graph + REAK progression |

## Runtime / library (`fabric-m3-consequence`)

| Capability | Status | Fabric location | RG usage |
| --- | --- | --- | --- |
| `EffectTruth` enum | **AVAILABLE_NOW** | `crates/fabric-m3-consequence/src/truth.rs` | Snapshot ingest |
| `SafeNextDecision` (`Proceed`/`Hold`) | **AVAILABLE_NOW** | `truth.rs` `safe_next_for` | Read-only gate |
| `adjudicate_truth` | **AVAILABLE_NOW** | `truth.rs` | RG must not reimplement |
| `reconciliation_clearance` | **AVAILABLE_NOW** | `truth.rs` | Pre-arm check |
| `RunOptions.allow_redispatch` | **AVAILABLE_NOW** | `lifecycle.rs` | Arm redispatch |
| `LifecycleError::BlindRetryBlocked` | **AVAILABLE_NOW** | `lifecycle.rs` | Error passthrough |
| `LifecycleOutcome` fields (`effect_truth`, `safe_next`, readback labels) | **AVAILABLE_NOW** | `lifecycle.rs` | Primary snapshot source |
| `SandboxLifecycleRun` / `ExternalLifecycleRun` | **THIN_ADAPTER** | `lifecycle.rs` | RG bridge invokes; not embedded in core |
| `ProviderPort` / `ExternalProviderAdapter` | **THIN_ADAPTER** | `adapter.rs`, `provider.rs` | WAIT_READBACK polls via Fabric only |
| SafeNext `RECOVER` posture | **MISSING** | `docs/m3/M3_TRUTH_STATES.md` lists `RECOVER`; `truth.rs` has no third enum variant | RG playbooks must treat HOLD only until Fabric adds `RECOVER` |
| `CONTRADICTION` in `EffectTruth` | **MISSING** | Design doc; Rust enum lacks `Contradiction` variant @ pin | RG must not synthesize contradiction handling beyond Fabric labels |
| Dedicated `request_restart` API on Fabric | **MISSING** | Restart arm is `RunOptions` flag on lifecycle execute | **THIN_ADAPTER** via EffectGuard `request_restart` |
| Recovery graph nodes (`RETRY_WITH_SAME_IDEMPOTENCY_KEY`, etc.) | **FUTURE** | `docs/m3/M3_SAFENEXT_POLICY.md` placeholders | Playbook kinds mirror intent only |

## Design documentation (@ pin)

| Doc | Status | RG usage |
| --- | --- | --- |
| `docs/m3/M3_SAFENEXT_POLICY.md` | **AVAILABLE_NOW** | Policy alignment for HOLD triggers |
| `docs/m3/M3_TRUTH_STATES.md` | **AVAILABLE_NOW** | Vocabulary; note RECOVER gap |
| `docs/m3/M3_PROVIDER_TRUTH_MODEL.md` | **AVAILABLE_NOW** | Non-claims for ACK-as-truth |
| `docs/m3/M3_CLAIM_INVENTORY.md` | **AVAILABLE_NOW** | LE-F / M3-X firewall |

## ACG object model (R1-A registry)

| Object type | Status | Registry | RG usage |
| --- | --- | --- | --- |
| `SafeNext` | **FUTURE** (graph emit) | `reak/consequence-fabric/safenext/1` | Lifecycle pushes stage object today; RG does not author |
| `RecoveryCandidate` | **FUTURE** | `recoverycandidate/1` | Playbook proposal mapping TBD |
| `RecoveryAuthorization` | **FUTURE** | `recoveryauthorization/1` | Human/policy signature TBD |
| `RecoveryDispatch` | **FUTURE** | `recoverydispatch/1` | Coupled to Fabric dispatch stage |
| `TruthState` | **THIN_ADAPTER** | M1/M3 lifecycle stages | Read via outcome labels |

Workspace mirror: `consequence-fabric-r1/crates/acg-object-model/frozen/ACG_OBJECT_TYPE_REGISTRY.json` matches domain separators (not integration runtime).

## EffectGuard (`effectfence` @ `b7c5368`)

| Capability | Status | Notes |
| --- | --- | --- |
| `SessionStore::request_restart` | **THIN_ADAPTER** | Qualified product API |
| `safe_next()` / `observe()` | **THIN_ADAPTER** | Snapshot ingest |
| `effectfence.pack/v1` | **AVAILABLE_NOW** | Parse-only for RG |
| Offline verify | **AVAILABLE_NOW** | RG must not duplicate |

## Programme / qualification inputs

| Input | Status | Notes |
| --- | --- | --- |
| M3 Lane E qual @ `fc62c375` | **AVAILABLE_NOW** | Manifest `m3.qualification` |
| Named integration ref | **AVAILABLE_NOW** | `doctrine/fabric/integration-m3-135bbd9` |
| `doctrine/main` @ `fc1c604` | **MISSING** (divergence) | Do not assume main = integration |
| LE-F-002 export | **MISSING** (blocked) | No passport ingest |

## REAK kernel (`reak-recovery`, `reak-progression`)

| Capability | Status | Notes |
| --- | --- | --- |
| In-process recovery module | **FUTURE** | REAK pin separate; RG is Fabric successor product lane |
| Phase 9 progression gate | **FUTURE** | Not production |

## Gap register (honest)

| Gap ID | Description | RG design mitigation |
| --- | --- | --- |
| GAP-01 | `RECOVER` SafeNext in docs not in Rust enum | Treat as HOLD; playbook `NOOP_HOLD` |
| GAP-02 | No first-class Recovery* lifecycle API | Playbook + THIN_ADAPTER to `allow_redispatch` |
| GAP-03 | Contradiction state not in Rust enum | Do not claim contradiction handling beyond Fabric strings |
| GAP-04 | No durable RG store in Fabric | `RG_DURABLE_PLAYBOOK_STORE_V1=NOT_IN_SCOPE` |
| GAP-05 | Remote `main` ≠ integration `135bbd9` | Pin named ref only |

## Adapter boundary (recommended)

```text
recovery-governor-bridge::
  ingest_from_effectguard(session) -> SubstrateSnapshot
  ingest_from_lifecycle(outcome) -> SubstrateSnapshot
  arm_via_effectguard(session) -> Result<(), BlindRetryBlocked>
  poll_readback_via_fabric(adapter, effect_id) -> ReadbackOutcome
```

Core crate depends only on serde + playbook schema — **no** `fabric-m3-consequence` HTTP paths.

## Related

- [RECOVERY_GOVERNOR_API_CONTRACT.md](./RECOVERY_GOVERNOR_API_CONTRACT.md)
- [RECOVERY_GOVERNOR_SEMANTIC_OWNERSHIP.md](./RECOVERY_GOVERNOR_SEMANTIC_OWNERSHIP.md)
