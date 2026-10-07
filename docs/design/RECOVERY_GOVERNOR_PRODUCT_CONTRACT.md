# Recovery Governor — product contract (design)

**Tranche:** `RECOVERY_GOVERNOR_PRODUCT_CONTRACT`  
**Mode:** Design only — no implementation, no qualification, no EffectGuard V1 or Fabric semantic/code changes  
**Evaluated:** 2026-10-07T19:35:00Z  
**Worker:** bc-1fab71cd-d20d-5f25-964a-b65f94c6e19f  
**Authority:** [PROGRAMME_PIN_MANIFEST.json](./consequence-fabric/PROGRAMME_PIN_MANIFEST.json) · Fabric ref `doctrine/fabric/integration-m3-135bbd9` @ `135bbd9`

## §0 — Mission

Define **Recovery Governor** as an independent Fabric-adjacent **product lane** that orchestrates **governed recovery and progression steps** after partial failure, HOLD, or operator pause — without becoming a second truth engine, without overriding SafeNext, and without duplicating EffectGuard’s evidence-verify surface.

Successor to EffectGuard V1 qualified freeze (`NEXT_PRODUCT_DESIGN_ACTION=RECOVERY_GOVERNOR_PRODUCT_CONTRACT`).

## §1 — Product question (frozen)

> **After Fabric has adjudicated truth and SafeNext, can an operator or automation safely select, authorize, and execute the *next recovery action* (poll, escalate, arm redispatch, pause for human) without the product layer smuggling clearance, collapsing ambiguity, or inventing progression policy that belongs to Fabric or the REAK kernel?**

```text
RECOVERY_GOVERNOR_IS_SEPARATE_SEMANTIC_ENGINE=false
RECOVERY_GOVERNOR_USES_FABRIC_SEMANTICS=true
```

## §2 — Frozen vocabulary

| Term | Meaning |
| --- | --- |
| **Substrate snapshot** | Immutable read of Fabric lifecycle outcome fields (`effect_truth`, `safe_next`, reconciliation signals, attempt ids) |
| **Playbook** | Product-owned **policy graph** of recovery *steps* (labels, timers, escalation), not truth rules |
| **Recovery recommendation** | Product output describing *eligible* next steps given snapshot + playbook — **non-authoritative** until Fabric clearance |
| **Clearance arm** | Product call that sets `allow_redispatch` / restart arms **only** when Fabric reconciliation rules already permit |
| **Progression** | REAK/Fabric spine stage advance after recovery — **not** redefined by this product |
| **SafeNext** | Fabric `SafeNextDecision` (`PROCEED` / `HOLD`) from `fabric-m3-consequence` — canonical gate |
| **ACG recovery objects** | `RecoveryCandidate`, `RecoveryAuthorization`, `RecoveryDispatch`, `SafeNext` graph types (R1-A registry) |

```text
RG_PLAYBOOK_IS_POLICY_NOT_TRUTH=true
RG_RECOMMENDATION_IS_NOT_CLEARANCE=true
RG_IS_NOT_KERNEL_PROGRESSION=true
```

## §3 — Scope firewall

| In scope (design) | Out of scope |
| --- | --- |
| Product contract, API sketch, invariants, adversarial matrix | Runtime implementation |
| Fabric interface map (honest gaps) | Fabric / EffectGuard code changes |
| Claim inventory (design-frozen, not activated) | Product qualification execution |
| Playbook catalog semantics | M4 witness implementation |
| Thin adapter to `RunOptions.allow_redispatch` + lifecycle | REAK kernel edits |
| Coexistence with EffectGuard (`effectfence`) | Change Proof lane |
| LE-F / programme debt carry-forward | Trust-boundary export (LE-F-002) |

**Naming:** Customer-facing **Recovery Governor**; engineering repo TBD at first build (`RECOVERY_GOVERNOR_CANONICAL_REPO=UNRESOLVED`).

## §4 — Authoritative Fabric inputs (verified)

| Pin | Value | Verification |
| --- | --- | --- |
| `FABRIC_DEPENDENCY_HEAD` | `135bbd932553613fa3dd758d73143401068a4cb8` | `git rev-parse origin/fabric/integration-m3-135bbd9` |
| `FABRIC_DEPENDENCY_TREE` | `4e724089c1df1a8bf4f891558983a5d6aad5fc28` | `git rev-parse …^{tree}` |
| `FABRIC_DEPENDENCY_SCOPE` | `GLOBALLY_AUTHORITATIVE_NAMED_INTEGRATION_REF` | manifest + remote branch |
| `FABRIC_M3_QUALIFIED` (input) | `true` (Lane E bounded) | manifest `m3.qualification` |
| `M3_MECHANISM_PRIMARY_PROFILE` | `P-WM-IDEMPOTENT-GET` | manifest |
| `M3_REAL_SANDBOX_PRIMARY_PROFILE` | `P-GITHUB-ISSUE-POST` | manifest |
| `EFFECTGUARD_QUALIFIED` | `true` (`v1-b7c5368`) | separate product; RG does not inherit qual |

```text
FABRIC_M3_QUALIFICATION_IS_INPUT_NOT_RG_VERDICT=true
```

## §5 — Programme debt (visible)

```text
LE_F_001_STATUS=OPEN_VISIBLE
LE_F_002_STATUS=OPEN_VISIBLE
LE_F_002_BLOCKS_M3_TRUST_BOUNDARY_EXPORT=true
M2_FAULT_F8=NOT_EXERCISED_VISIBLE
```

Recovery Governor design **must not** imply export passport, witness crown, or LE-F closure.

## §6 — Product identity

| Field | Value |
| --- | --- |
| Programme lane | Recovery Governor |
| Product name (SoT) | **Recovery Governor** |
| Intended engineering repo | `recovery-governor` (placeholder until first build) |
| Substrate library | `fabric-m3-consequence` @ pinned integration ref |
| Adjacent qualified product | EffectGuard (`effectfence` @ `b7c5368`) — evidence/verify, not orchestration |

## §7 — Dependency model (frozen)

```text
RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH=false
RECOVERY_GOVERNOR_CAN_DEFINE_NEW_TRUTH_STATE=false
RECOVERY_GOVERNOR_CAN_OVERRIDE_RECONCILIATION=false
RECOVERY_GOVERNOR_CAN_OVERRIDE_SAFENEXT=false
RECOVERY_GOVERNOR_CAN_AUTHORIZE_UNCLEARED_REDISPATCH=false
RECOVERY_GOVERNOR_PROVIDER_PROFILE_AUTHORITY=false
```

Fabric correctness is assumed only within pinned M3 qualification scope. RG attacks **orchestration and playbook presentation** faults, not Fabric Lane E re-qual.

## §8 — Design qualification question (future lane)

> Can Recovery Governor’s playbook engine, timers, or operator UX cause an **uncleared redispatch**, **false sense of PROCEED**, or **suppressed HOLD** even when Fabric underneath is correct?

Execution lane **not** authorized in this tranche (see [RECOVERY_GOVERNOR_DESIGN_GATE.md](./RECOVERY_GOVERNOR_DESIGN_GATE.md)).

## §9 — Relationship to EffectGuard

| Concern | EffectGuard | Recovery Governor |
| --- | --- | --- |
| Offline verify / pack digest | **Owns** | Consumes packs as **read-only** snapshots (optional) |
| Session dispatch API | **Owns** product session | **Must not** fork session store; may call shared bridge or ingest outcomes |
| Explain text | **Owns** non-authoritative explanation | **Owns** playbook step labels + escalation copy |
| Retry arm | Surfaces `request_restart` | **Owns** when to *invoke* restart API after policy timers |

```text
RG_DOES_NOT_REPLACE_EFFECTGUARD_VERIFY=true
RG_DOES_NOT_DUPLICATE_EG_SESSION_SEMANTICS=true
```

## §10 — Restart / durability limitation (frozen)

```text
LANE_E_GITHUB_RESTART_AMBIGUITY=NOT_EXECUTED
RG_PROCESS_RESTART_CASE=EXCLUDED_BOUNDED_LIMITATION
RG_DURABLE_PLAYBOOK_STORE_V1=NOT_IN_SCOPE
```

**Admitted at v0 design:** in-process playbook evaluation on substrate snapshot; in-session arm of `allow_redispatch` after Fabric clearance.  
**Excluded:** OS/process restart, durable playbook store reopen, universal restart safety claims (mirrors EffectGuard IPQ-1 boundary).

## §11 — Orchestration freshness (frozen)

```text
RG_ORCHESTRATION_FRESHNESS_IS_LOAD_BEARING=true
```

Playbook evaluation must use the **latest** substrate snapshot for the bound `operation_id`. Stale snapshots cannot authorize steps.

## §12 — Playbook vs SafeNext (frozen)

```text
RG_PLAYBOOK_STEP_CANNOT_OVERRIDE_SAFENEXT=true
RG_INTERNAL_ERROR_DEFAULTS_TO_RECOVERY_ALLOWED=false
```

No playbook step may map HOLD→PROCEED or arm redispatch when Fabric denies. Internal errors must not default to “safe to recover.”

## §13 — Error-handling posture

Product errors surface as **blocked step** with reason code; never imply Fabric clearance. See adversarial matrix RG-E*.

## §14 — Design scope freeze (frozen)

```text
PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0
```

Minimum v0 product slice (first build target):

- Single-operation **GovernorSession** binding
- Playbook catalog v0 (declarative steps: `WAIT_READBACK`, `ESCALATE_HUMAN`, `ARM_REDISPATCH_IF_CLEARED`, `NOOP_HOLD`)
- Substrate ingest from Fabric lifecycle outcome JSON or EffectGuard pack fields
- One wiremock profile path + optional GitHub path via **existing** product/Fabric bridge (no new provider client in RG core)

Out of v0: multi-tenant SaaS, cross-operation saga orchestration, ACG-signed RecoveryAuthorization graph emission.

## §15 — EVP / programme alignment (non-proof)

External validation theme #4 (recovery/progression) motivates the product wedge; [EVP_MATRIX_DEEP_DIVE_04_RECOVERY_PROGRESSION.md](./external-validation-programme/matrix-deep-dives/EVP_MATRIX_DEEP_DIVE_04_RECOVERY_PROGRESSION.md) is **directional only**, not qualification evidence.

## §16 — Claims posture (frozen)

No customer-facing qualification claims in this tranche. Candidate claims live in [RECOVERY_GOVERNOR_CLAIM_INVENTORY.md](./RECOVERY_GOVERNOR_CLAIM_INVENTORY.md) — all `NOT_ACTIVATED`.

## §17 — Non-claims (frozen)

Preserved non-claims (RG must not imply):

- Universal external exactly-once / universal external safety (programme `non_claims`)
- Fabric programme-wide qualified
- M4 third-party witness qualified
- EffectGuard qualification transfer
- REAK Phase 9 production progression
- `SAFE_TO_RETRY` as stored fact
- Temporal replacement / checkpoint sovereignty
- Trust-boundary passport (LE-F-002)

## §18 — LE-F firewall

RG qualification (future) cannot close LE-F-001/002 or activate M3-X export claims.

## §19 — Failure policy (design)

```text
UNRESOLVED_CRITICAL_FINDING => FAIL
UNRESOLVED_HIGH_FINDING => FAIL
```

Applies to future qualification; design gate uses same rubric for internal consistency review.

## §20 — Independent product lane

```text
RECOVERY_GOVERNOR_PRODUCT_LANE=RG-P0
```

Separate repo/worktree from `effectfence`; may share Fabric git pin only.

## §21 — Pin discipline

Any product-material implementation requires recorded `RG_SUBJECT_HEAD` + `FABRIC_DEPENDENCY_HEAD` before qualification. Design bound to `rg-design-v0` until superseded.

## §22 — Evidence bundle schema (future execution)

Bundle ID pattern: `recovery-governor-rgq0-evidence/<subject-head-short>/`

| Artefact | Content |
| --- | --- |
| `subject.json` | RG repo head, tree, timestamp |
| `fabric_dependency.json` | integration HEAD, scope, M3 qual refs |
| `playbook_catalog_hash.json` | pinned playbook bytes |
| `hostile/` | RG-H* logs |
| `substrate_fixtures/` | wiremock + redacted GitHub traces |
| `findings_ledger.json` | severity, status, case ref |
| `claim_adjudication.json` | inventory row outcomes |
| `limitations.md` | restart, durable store exclusions |
| `verdict.json` | closed enum |

## §23 — Design gate

Entry/exit: [RECOVERY_GOVERNOR_DESIGN_GATE.md](./RECOVERY_GOVERNOR_DESIGN_GATE.md).

## §24 — Semantic ownership

Canonical split: [RECOVERY_GOVERNOR_SEMANTIC_OWNERSHIP.md](./RECOVERY_GOVERNOR_SEMANTIC_OWNERSHIP.md).

## §25 — API contract

[RECOVERY_GOVERNOR_API_CONTRACT.md](./RECOVERY_GOVERNOR_API_CONTRACT.md).

## §26 — Invariants and hostility

- [RECOVERY_GOVERNOR_INVARIANTS.md](./RECOVERY_GOVERNOR_INVARIANTS.md)
- [RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md](./RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md)

## §27 — Next authorized action (frozen)

```text
STOP_AFTER=RECOVERY_GOVERNOR_PRODUCT_CONTRACT
NEXT_AUTHORIZED_ACTION=RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD
RG_IMPLEMENTATION_AUTHORIZED=false
RG_QUALIFICATION_AUTHORIZED=false
```

Implementation requires separate build charter; qualification requires separate RGQ lane charter.

## §28 — Fabric interface

[RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md](./RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md).

## §29 — Claim inventory

[RECOVERY_GOVERNOR_CLAIM_INVENTORY.md](./RECOVERY_GOVERNOR_CLAIM_INVENTORY.md).

## §30 — Deliverables checklist

| Artefact | Status |
| --- | --- |
| Product contract | this file |
| Semantic ownership | published |
| API contract | published |
| Invariants | published |
| Adversarial matrix RG-H01..H18 | published |
| Fabric interface map | published |
| Claim inventory | frozen design |
| Design gate | published |

## §31 — Terminal pointer

Full §31 token block: [RECOVERY_GOVERNOR_DESIGN_GATE.md](./RECOVERY_GOVERNOR_DESIGN_GATE.md) § Terminal report.

## §32 — Charter binding

User mission charter sections 0–32 are satisfied by this contract package. Frozen architecture flags appear in §1, §10–§12, §14, §16–§17, §22, §27 and are duplicated in semantic ownership + design gate for lint.
