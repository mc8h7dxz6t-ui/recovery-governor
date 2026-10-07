# Recovery Governor — claim inventory (design frozen)

**Status:** `RG_CUSTOMER_CLAIMS_ACTIVATED=false`  
**Scope:** `PRODUCT_DESIGN_SCOPE_FROZEN=rg-design-v0`  
**Activation:** requires future RGQ qualification + charter — **not** this tranche.

## Claim classes

| Class | Meaning |
| --- | --- |
| `NOT_ACTIVATED` | Design-only; forbidden in customer surfaces |
| `NOT_CLAIMED` | Explicit programme non-claim |
| `CANDIDATE_POST_QUAL` | Wording reserved; needs evidence |
| `INHERITED_INPUT` | Fabric/EG qual as dependency only — not RG verdict |

## Inventory

| ID | Claim statement | Class | Dependency / fence |
| --- | --- | --- | --- |
| RG-C-001 | Recovery Governor is a separate semantic engine from Fabric | **NOT_CLAIMED** | `RECOVERY_GOVERNOR_IS_SEPARATE_SEMANTIC_ENGINE=false` |
| RG-C-002 | Recovery Governor uses Fabric M3 truth and SafeNext semantics | **CANDIDATE_POST_QUAL** | Pin `135bbd9` |
| RG-C-003 | Playbooks cannot override SafeNext | **CANDIDATE_POST_QUAL** | RG-H01..H04 |
| RG-C-004 | Blind retry blocked when Fabric denies | **CANDIDATE_POST_QUAL** | `BlindRetryBlocked` |
| RG-C-005 | Product qualified for production external effects | **NOT_ACTIVATED** | No RGQ yet |
| RG-C-006 | Universal restart / process-crash safe recovery | **NOT_CLAIMED** | §10 exclusion |
| RG-C-007 | Provider-neutral qualification | **NOT_CLAIMED** | programme `non_claims` |
| RG-C-008 | Exactly-once external effects | **NOT_CLAIMED** | programme `non_claims` |
| RG-C-009 | Trust-boundary passport / high-assurance export | **NOT_CLAIMED** | LE-F-002 |
| RG-C-010 | M4 witness assurance | **NOT_CLAIMED** | M4 separate |
| RG-C-011 | EffectGuard qualification implies RG qualified | **NOT_CLAIMED** | `RG_INHERITS_EFFECTGUARD_QUALIFIED=false` |
| RG-C-012 | Fabric M3 qualified implies RG qualified | **NOT_CLAIMED** | `FABRIC_M3_QUALIFICATION_IS_INPUT_NOT_RG_VERDICT=true` |
| RG-C-013 | Governed progression after uncertainty (EVP theme) | **NOT_ACTIVATED** | EVP directional only |
| RG-C-014 | Replaces Temporal / LangGraph checkpoints | **NOT_CLAIMED** | positioning forbidden |
| RG-C-015 | Emits signed RecoveryAuthorization ACG objects | **NOT_ACTIVATED** | FUTURE Fabric API |
| RG-C-016 | Real GitHub recovery path qualified | **NOT_ACTIVATED** | needs RG-R* |
| RG-C-017 | Orchestration freshness guarantees | **CANDIDATE_POST_QUAL** | INV-06..08 |
| RG-C-018 | Programme-wide product crown | **NOT_CLAIMED** | integration charter |

## Marketing lint (forbidden until activation)

- `production-ready recovery`
- `certified operational resilience`
- `DORA-compliant` (without external validation CHG)
- `SAFE_TO_RETRY` as product fact
- `qualified` without `RG_QUALIFIED=true` evidence bundle

## Programme non-claims (carry-forward)

From [PROGRAMME_PIN_MANIFEST.json](./consequence-fabric/PROGRAMME_PIN_MANIFEST.json) `non_claims` — RG inherits as **NOT_CLAIMED** for all rows.

## LE-F visibility

```text
LE_F_001_STATUS=OPEN_VISIBLE
LE_F_002_STATUS=OPEN_VISIBLE
LE_F_002_BLOCKS_M3_TRUST_BOUNDARY_EXPORT=true
```

## Related

- [RECOVERY_GOVERNOR_PRODUCT_CONTRACT.md](./RECOVERY_GOVERNOR_PRODUCT_CONTRACT.md) §16–§17
- [RECOVERY_GOVERNOR_DESIGN_GATE.md](./RECOVERY_GOVERNOR_DESIGN_GATE.md)
