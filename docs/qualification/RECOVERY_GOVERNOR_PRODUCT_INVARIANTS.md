# Recovery Governor — product invariants (qualification v1)

**Scope:** `PRODUCT_QUALIFICATION_SCOPE_FROZEN=v1-4d6cb05`  
**Design catalogue:** [RECOVERY_GOVERNOR_INVARIANTS.md](../design/RECOVERY_GOVERNOR_INVARIANTS.md) (authoritative wording)

Load-bearing invariants for IPQ-1. Violation → **CRITICAL** unless noted **HIGH**.

## Exercisable @ v1 subject

| ID | Invariant | IPQ-1 class |
| --- | --- | --- |
| FW-01..FW-06 | Semantic firewall flags | A, G |
| INV-01..INV-05 | SafeNext / recommendation (projection-only subset) | A, H |
| INV-06..INV-08 | Freshness via `revalidate_advisory` | F |
| INV-09 | Wrong operation / case binding | C, H |
| INV-15..INV-16 | Error defaults | E |
| INV-17..INV-18 | EG bridge boundaries | B, H |

## Not exercisable @ v1 (documented)

| ID | Reason |
| --- | --- |
| INV-12..INV-14 | Playbook engine absent (CF-003) |
| INV-20..INV-21 | Restart durability excluded |
| INV-22 | ACG graph FUTURE |
