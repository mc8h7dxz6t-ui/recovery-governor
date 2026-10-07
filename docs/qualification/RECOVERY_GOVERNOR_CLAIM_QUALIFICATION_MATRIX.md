# Recovery Governor — claim qualification matrix (design)

**All customer claims:** `NOT_ACTIVATED` until execution tranche.

| Claim ID | Wording (candidate) | Qual class | v1 status |
| --- | --- | --- | --- |
| RG-C-001 | Safe advisory SafeNext projection | A, D | NOT_ACTIVATED |
| RG-C-002 | Governed dispatch via EffectGuard only | B | NOT_ACTIVATED |
| RG-C-003 | No override of Fabric SafeNext | A, H | NOT_ACTIVATED |
| RG-C-004 | Stale advisory detection | F | NOT_ACTIVATED |

**Non-claims (preserved):** universal restart safety, Fabric programme crown, EffectGuard qual transfer, LE-F export, exactly-once, M4 witness, playbook catalog completeness.

See design [RECOVERY_GOVERNOR_CLAIM_INVENTORY.md](../design/RECOVERY_GOVERNOR_CLAIM_INVENTORY.md).
