# Recovery Governor — adversarial matrix (IPQ-1 design)

**Lane:** `RG-IPQ-1` · **Scope:** `v1-4d6cb05`  
**CF-001:** [RG_CF001_HOSTILE_ID_TRACEABILITY.md](./RG_CF001_HOSTILE_ID_TRACEABILITY.md)

## Class A — RG-Q (product hostility)

| Case | Intent | Invariants |
| --- | --- | --- |
| RG-Q01 | Advisory pack SafeNext ≠ EG `safe_next` | INV-02, FW-04 |
| RG-Q02 | HOLD snapshot → advisory implies PROCEED | INV-01 |
| RG-Q03 | Stale `latest_advisory` treated fresh without revalidate | INV-06 |
| RG-Q04 | Wrong leaf operation drives advice | INV-09 |
| RG-Q05 | Cross-case operation bleed | INV-09 |
| RG-Q06..Q24 | Cache, mode flip, export, digest tamper, ancestry depth | FW-*, INV-* |

## Class B — RG-B (EffectGuard bridge)

| Case | Intent |
| --- | --- |
| RG-B01 | `governed_dispatch` passthrough lifecycle outcome |
| RG-B02 | `request_restart` denied when EG denies |
| RG-B03 | Advisory mode blocks dispatch |
| RG-B04 | No duplicate `SessionStore` register path |
| RG-B05..B12 | Profile WM, fault matrix, export firewall |

## Class C — RG-C (case / mode)

| Case | Intent |
| --- | --- |
| RG-C01 | Default `ADVISORY` |
| RG-C02 | Parent/child ancestry integrity |
| RG-C03 | Leaf operation selection |

## Class D — RG-D (advisory pack)

| Case | Intent |
| --- | --- |
| RG-D01 | Schema + digest validation |
| RG-D02 | Substrate pin in pack |

## Class E — RG-E (errors)

| Case | V1 slice test |
| --- | --- |
| RG-E01 | Happy path | `rg_e01_happy_path` |
| RG-E02 | Parent/child flow | `rg_e02_parent_child_flow` |
| RG-E03 | Revalidate + mode flip | `rg_e03_revalidate_and_mode_flip` |
| RG-E04 | Restart when PROCEED | `rg_e04_restart_when_proceed` |
| RG-E05..E08 | Fabric/EG error passthrough, no retry-OK copy |

## Class F — RG-F (freshness)

| Case | Intent |
| --- | --- |
| RG-F01 | `revalidate_advisory` Fresh after advise |
| RG-F02 | Stale when outcome changes |

## Class G — RG-G (GAP probes)

RG-G01..G05 per [RECOVERY_GOVERNOR_GAP_QUALIFICATION_MATRIX.md](./RECOVERY_GOVERNOR_GAP_QUALIFICATION_MATRIX.md).

## Class H — RG-H (design-aligned)

RG-H01..H18 per **design** [RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md](../design/RECOVERY_GOVERNOR_ADVERSARIAL_MATRIX.md). V1 slice IDs **do not** satisfy Class H.

## Class I — RG-R (real provider)

| Case | Status |
| --- | --- |
| RG-R01..R06 | **NOT_EXECUTED** v1 — requires token + charter |
