# Recovery Governor — subject API surface @ IPQ-1

**Subject:** `4d6cb05` (product material `d90d92d`)  
**Crate:** `recovery-governor` · **Store:** `CaseStore`

## Public types

| Type | Role |
| --- | --- |
| `CaseStore` | Session root; owns embedded `effectguard::SessionStore` |
| `CaseRecord` | Case id, parent, children, mode, operation list, leaf |
| `ExecutionMode` | `ADVISORY` (default) \| `GOVERNED` |
| `AdvisoryView` / `AdvisoryPack` | SafeNext projection + `recovery-governor.pack/v1` |
| `RevalidationVerdict` | `Fresh` \| `Stale` |
| `RecoveryGovernorError` | Product errors (incl. `EffectGuard` passthrough) |

## Methods (qualification-relevant)

| Method | Mutates Fabric truth? | Qual class |
| --- | --- | --- |
| `open_case` | No | C |
| `bind_operation` | No (EG register) | C, B |
| `ancestry` / `case_status` | No | C |
| `set_execution_mode` | No | C |
| `advise_safe_next` | No | A, D |
| `revalidate_advisory` | No | F |
| `governed_begin_attempt` | No | B |
| `governed_dispatch` | **Fabric lifecycle via EG** | B |
| `request_restart` | Arms via EG only | B |

## Not present @ subject (bounded)

`GovernorSession`, `evaluate_playbook`, `try_arm_redispatch`, `SubstrateSnapshot::ingest` — see CF-002; Class H/D deferred items in charter BL-02.
