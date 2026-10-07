# Recovery Governor — claim qualification matrix (design)

**Customer claims:** activated (bounded) per IPQ-1 integration — see [RECOVERY_GOVERNOR_CUSTOMER_CLAIM_MAP.md](../RECOVERY_GOVERNOR_CUSTOMER_CLAIM_MAP.md).

| Claim ID | Wording (candidate) | Qual class | v1 status |
| --- | --- | --- | --- |
| RG-C-001 | Safe advisory SafeNext projection | A, D | QUALIFIED (bounded) |
| RG-C-002 | Governed dispatch via EffectGuard only | B | QUALIFIED (bounded) |
| RG-C-003 | No override of Fabric SafeNext | A, H | QUALIFIED (bounded) |
| RG-C-004 | Stale advisory detection | F | QUALIFIED (bounded) |

**Non-claims (preserved):** universal restart safety, Fabric programme crown, EffectGuard qual transfer, LE-F export, exactly-once, M4 witness, playbook catalog completeness.

See design [RECOVERY_GOVERNOR_CLAIM_INVENTORY.md](../design/RECOVERY_GOVERNOR_CLAIM_INVENTORY.md).
