# Recovery Governor claim traceability

**Build:** `RECOVERY_GOVERNOR_BUILD=QUALIFIED_BOUNDED_V1` · **Qualified subject:** `4d6cb052fd99f4e5678d25c4274615cd47681751`  
**IPQ-1:** `PASS_WITH_BOUNDED_LIMITATIONS` · **Evidence:** [qualification/rg-ipq1/](./qualification/rg-ipq1/)

## Product claims (qualified — bounded)

| Claim ID | Statement | Evidence | Status |
| --- | --- | --- | --- |
| RG-CL-001 | No mutation of Fabric canonical truth | RG-H01, claim_flags | **QUALIFIED** |
| RG-CL-002 | Semantic firewall vs Fabric / SafeNext | RG-H01..H04 | **QUALIFIED** |
| RG-CL-003 | Mode gating for governed dispatch | RG-H03, bridge | **QUALIFIED (bounded)** |
| RG-CL-004 | HOLD on ack-without-effect / blind retry | RG-H11, RG-H12 | **QUALIFIED** |
| RG-CL-005 | No live GitHub RG-R crown | Class I matrix | **QUALIFIED (bounded)** |
| RG-CL-017 | Freshness / stale advisory detection | RG-F, revalidate | **QUALIFIED (bounded)** |

## Claims NOT made

| ID | Statement |
| --- | --- |
| RG-NC-006 | Universal restart safety after crash |
| RG-NC-016 | Real provider recovery path qualified |
| RG-NC-011 | EffectGuard qual transfers to RG |
| RG-NC-012 | Fabric M3 qual transfers to RG |
| RG-NC-009 | Trust-boundary export while LE-F-002 open |

## Machine-readable state

[RECOVERY_GOVERNOR_QUALIFICATION_STATE.json](./RECOVERY_GOVERNOR_QUALIFICATION_STATE.json)

See [RECOVERY_GOVERNOR_CUSTOMER_CLAIM_MAP.md](./RECOVERY_GOVERNOR_CUSTOMER_CLAIM_MAP.md).
