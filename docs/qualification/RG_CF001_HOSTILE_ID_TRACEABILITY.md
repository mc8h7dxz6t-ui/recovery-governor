# CF-001 — RG-H01..H18 hostile ID traceability (qual design)

**Finding:** [RECOVERY_GOVERNOR_V1_CONFORMANCE_FINDINGS.json](../conformance/RECOVERY_GOVERNOR_V1_CONFORMANCE_FINDINGS.json) CF-001  
**Design matrix:** [RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md](../design/RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md)  
**V1 harness @ subject:** `crates/recovery-governor/tests/rg_hostile.rs` (slice semantics)

## Policy

IPQ-1 **Class H** executes **design-matrix** hostile methods. V1 slice tests that reuse IDs are **aliases** until remapped or superseded by Class H evidence.

| Design ID | Design attack intent | V1 alias test (subject) | IPQ-1 execution case | CF-001 disposition |
| --- | --- | --- | --- | --- |
| RG-H01 | HOLD→PROCEED via playbook | `rg_h01_semantic_firewall_flags` | **RG-H01-DESIGN** (new) | **SUPERSEDE** alias |
| RG-H02 | SafeNext echo drift | `rg_h02_default_advisory_mode` | **RG-H02-DESIGN** | **SUPERSEDE** |
| RG-H03 | UNKNOWN collapsed | — | **RG-H03-DESIGN** | **ADD** |
| RG-H04 | Uncleared arm | `rg_h12_hold_advice_not_proceed` (partial) | **RG-H04-DESIGN** | **REMAP** |
| RG-H05 | Hidden dispatch | — | **RG-H05-DESIGN** | **ADD** |
| RG-H06 | Stale snapshot arm | `rg_h09_revalidate_fresh` / `rg_h10_*` | **RG-H06-DESIGN** | **PARTIAL** → extend |
| RG-H07 | Timer reorder | — | **RG-H07-DESIGN** | **DEFER** (no playbook tick @ v1) |
| RG-H08 | Cross-operation govern | `rg_h14_cross_case_isolation` | **RG-H08-DESIGN** | **REMAP** |
| RG-H09 | Lineage loss | `rg_e02_parent_child_flow` (partial) | **RG-H09-DESIGN** | **EXTEND** |
| RG-H10 | Pack op swap | — | **RG-H10-DESIGN** | **ADD** (pack ingest) |
| RG-H11 | Playbook embeds truth | — | **RG-H11-DESIGN** | **N/A** until playbook |
| RG-H12 | Catalog drift | — | **RG-H12-DESIGN** | **N/A** until catalog |
| RG-H13 | Unknown playbook | — | **RG-H13-DESIGN** | **N/A** until catalog |
| RG-H14 | Error→retry OK | export / dispatch errors | **RG-H14-DESIGN** | **PARTIAL** |
| RG-H15 | Duplicate verify | — | **RG-H15-DESIGN** | **ADD** (dep graph) |
| RG-H16 | Fork SessionStore | — | **RG-H16-DESIGN** | **PASS** (embed EG, no fork) |
| RG-H17 | Kernel progression claim | — | **RG-H17-DESIGN** | **CLAIM_LINT** |
| RG-H18 | Restart durability | EXCLUDED | **RG-H18-NOT_EXECUTED** | **ALIGNED** |

```text
CF_001_CLOSURE_CRITERION=CLASS_H_DESIGN_ALIGNED_EVIDENCE_PASS
V1_SLICE_ALIASES_DO_NOT_SATISFY_CLASS_H=true
```
