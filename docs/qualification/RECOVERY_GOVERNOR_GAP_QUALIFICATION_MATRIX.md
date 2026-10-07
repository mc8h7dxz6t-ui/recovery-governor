# Recovery Governor — GAP qualification matrix (GAP-01..05)

**Design map:** [RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md](../design/RECOVERY_GOVERNOR_FABRIC_INTERFACE_MAP.md)  
**Scope:** IPQ-1 design — probes defined; evidence at execution

| Gap | Design status | Qual probe (Class G) | Expected @ execution | Blocks qual? |
| --- | --- | --- | --- | --- |
| **GAP-01** | `RECOVER` doc vs Rust HOLD/PROCEED only | **RG-G01** | RG never emits PROCEED when Fabric snapshot HOLD; no third SafeNext variant | No (bounded) |
| **GAP-02** | No Recovery* lifecycle API | **RG-G02** | Redispatch only via EG `request_restart` + `RunOptions.allow_redispatch` | No |
| **GAP-03** | No Contradiction enum in Rust | **RG-G03** | RG does not synthesize contradiction labels beyond Fabric strings | No |
| **GAP-04** | No durable RG store in Fabric | **RG-G04** | In-memory only; no durability claims in evidence | No |
| **GAP-05** | `doctrine/main` ≠ integration | **RG-G05** | Pack flags + manifest cite `135bbd9` named ref only | No |

```text
BLOCKED_FABRIC_INTERFACE=false
GAP_QUAL_DESIGN_COMPLETE=true
```
