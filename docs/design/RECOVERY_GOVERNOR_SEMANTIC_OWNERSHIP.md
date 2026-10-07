# Recovery Governor — semantic ownership

**Tranche:** `RECOVERY_GOVERNOR_PRODUCT_CONTRACT`  
**Purpose:** Exactly-one semantic owner per responsibility; prevent duplicate engines vs Fabric, EffectGuard, REAK kernel, M4 witness.

```text
RECOVERY_GOVERNOR_SEMANTIC_FIREWALL=v0
ENFORCEMENT=design_review + future claim_lint + CI
VIOLATION_CLASS=DUPLICATE_SEMANTIC_OWNER
```

## §1 — Frozen architecture flags (charter §1)

```text
RECOVERY_GOVERNOR_IS_SEPARATE_SEMANTIC_ENGINE=false
RECOVERY_GOVERNOR_USES_FABRIC_SEMANTICS=true
```

## §10 — Restart / durability (charter §10)

```text
LANE_E_GITHUB_RESTART_AMBIGUITY=NOT_EXECUTED
RG_PROCESS_RESTART_CASE=EXCLUDED_BOUNDED_LIMITATION
RG_DURABLE_PLAYBOOK_STORE_V1=NOT_IN_SCOPE
```

## §11–§12 — Orchestration contracts (charter §11–§12)

```text
RG_ORCHESTRATION_FRESHNESS_IS_LOAD_BEARING=true
RG_PLAYBOOK_STEP_CANNOT_OVERRIDE_SAFENEXT=true
RG_INTERNAL_ERROR_DEFAULTS_TO_RECOVERY_ALLOWED=false
```

## §14 — Scope freeze (charter §14)

```text
PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0
```

## §16–§17 — Claims firewall (charter §16–§17)

```text
RG_CUSTOMER_CLAIMS_ACTIVATED=false
RG_INHERITS_EFFECTGUARD_QUALIFIED=false
RG_INHERITS_FABRIC_PROGRAMME_CROWN=false
```

## §22 — Evidence custody (charter §22)

Future qualification evidence is **RG-owned** artefacts; Fabric qual bundles remain **Fabric-owned** — no commingling verdicts.

## §27 — Lane authorization (charter §27)

```text
NEXT_AUTHORIZED_ACTION=RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD
RG_IMPLEMENTATION_AUTHORIZED=false
```

## Ownership matrix

| Responsibility | Owner | RG may… |
| --- | --- | --- |
| Effect truth adjudication (`EffectTruth`) | Fabric M3 `truth.rs` | Read only |
| SafeNext decision (`SafeNextDecision`) | Fabric M3 `safe_next_for` | Read only; never recompute with divergent rules |
| Reconciliation clearance | Fabric M3 `reconciliation_clearance` | Read only |
| Lifecycle stage machine + `BlindRetryBlocked` | `fabric-m3-consequence` lifecycle | Invoke via adapter; no fork |
| Provider HTTP / profiles | Fabric M3 adapters | **No** in-core client (mirror EffectGuard FW-D-005) |
| ACG object encoding (`SafeNext`, `Recovery*`) | R1-A + Fabric graph emitters | Emit **only** if Fabric API exists; else document FUTURE |
| `RecoveryCandidate` / `RecoveryAuthorization` semantics | Fabric / REAK spine (target) | Product playbooks **propose**; Fabric **admits** |
| Playbook catalog, timers, escalation UX | **Recovery Governor** | Own |
| Recovery recommendation text | **Recovery Governor** | Own (non-authoritative) |
| Evidence pack / offline verify | **EffectGuard** | RG consumes; does not own verify vocabulary |
| Witness / export assurance | Fabric M4 design | Reference only |
| Programme qualification crowns | Integration + charter | RG cannot self-activate |
| LE-F export firewall | Fabric | RG cannot bypass |

## Deny rules (product repo MUST NOT)

| ID | Forbidden | Canonical owner |
| --- | --- | --- |
| RG-D-001 | Second `adjudicate_truth` / alternate READBACK_WINS | Fabric M3 |
| RG-D-002 | HOLD→PROCEED mapping in playbook engine | Fabric SafeNext |
| RG-D-003 | Set `allow_redispatch` without Fabric clearance path | Fabric lifecycle |
| RG-D-004 | Fork `fabric-m3-consequence` lifecycle | Fabric integration |
| RG-D-005 | In-path provider HTTP in playbook evaluator core | Fabric adapter / EffectGuard session |
| RG-D-006 | Claims `M3_REAL_EXTERNAL_PROVIDER_QUALIFIED` or Fabric Lane E crown | Fabric qualification |
| RG-D-007 | M3-X export claims while LE-F-002 open | Fabric claim inventory |
| RG-D-008 | Store `SAFE_TO_RETRY` as durable fact | Programme non-claim |
| RG-D-009 | Replace `reak-recovery` / kernel progression | REAK kernel |
| RG-D-010 | Duplicate EffectGuard `SessionStore` semantics | EffectGuard product |

## Allow rules (product repo MAY)

| ID | Allowed | Condition |
| --- | --- | --- |
| RG-A-001 | Depend on `fabric-m3-consequence` @ pinned git rev | No vendored fork |
| RG-A-002 | Parse `effectfence.pack/v1` for substrate snapshot | Read-only; no verify reimplementation |
| RG-A-003 | Declarative playbook catalog (versioned) | Does not embed truth tables |
| RG-A-004 | Call EffectGuard or bridge APIs for `request_restart` / dispatch | Clearance must come from Fabric arm |
| RG-A-005 | Hostile tests RG-H* using Fabric test helpers | Semantics owned by Fabric |
| RG-A-006 | Timer / poll **scheduling** for readback retry | Poll invokes Fabric readback API only |

## Import boundary

```text
ALLOWED_IMPORTS=fabric_m3_consequence::{lifecycle, truth, provider, ...}
OPTIONAL_IMPORTS=effectguard::SessionStore (bridge crate only)
FORBIDDEN_COPY_PASTE=truth.rs, safe_next policy tables, export_firewall
```

## Cross-product firewalls

```text
RG_DOES_NOT_REPLACE_EFFECTGUARD_VERIFY=true
RG_DOES_NOT_DUPLICATE_EG_SESSION_SEMANTICS=true
RG_PLAYBOOK_IS_POLICY_NOT_TRUTH=true
RG_RECOMMENDATION_IS_NOT_CLEARANCE=true
RG_IS_NOT_KERNEL_PROGRESSION=true
```

## Violation response

```text
ON_DUPLICATE_SEMANTIC_OWNER_DETECTED =>
  STOP_BUILD
  FILE_PROGRAMME_DEBT
  NO_SILENT_FORK
```

## Related

- [RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md](./RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md)
- [EFFECTGUARD_SEMANTIC_OWNERSHIP_FIREWALL.md](./effectguard/EFFECTGUARD_SEMANTIC_OWNERSHIP_FIREWALL.md)
- Fabric: `docs/m3/M3_SAFENEXT_POLICY.md` @ `135bbd9`
