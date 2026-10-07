# RG-IPQ-1 execution — bounded limitations

| ID | Limitation | Customer-claim safe? |
| --- | --- | --- |
| BL-RG-IPQ1-01 | Class I `RG-R01..R06` not executed | No live-provider recovery claims |
| BL-RG-IPQ1-02 | Process restart / Lane E GitHub restart ambiguity not exercised | No restart durability claims |
| BL-RG-IPQ1-03 | Class A `RG-Q` matrix partially bounded via H/E slice | No full hostile-matrix coverage claims |
| BL-RG-IPQ1-04 | CF-001 design Class H items deferred until playbook/catalog | No playbook/catalog safety claims |
| BL-RG-IPQ1-05 | EffectGuard bridge via path dependency pin at execution | Pin `b7c5368` required for reproduction |
| BL-RG-IPQ1-06 | `RG_IPQ1_FRA_ORACLE_AVAILABLE=false` — no FRA oracle evidence | Do not cite RA-1/RA-2 |

```text
LE_F_001_STATUS=OPEN_VISIBLE
LE_F_002_STATUS=OPEN_VISIBLE
LANE_E_GITHUB_RESTART_AMBIGUITY=NOT_EXECUTED
RG_PROCESS_RESTART_CASE=EXCLUDED_BOUNDED_LIMITATION
```
