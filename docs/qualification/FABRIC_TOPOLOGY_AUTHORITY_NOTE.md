# Fabric topology authority note (§9)

**Audience:** IPQ-1 execution and evidence reviewers  
**One-page authority model for Recovery Governor dependencies**

## What is authoritative for RG

| Layer | Authority | Pin |
| --- | --- | --- |
| Fabric integration | **Named ref** on remote `doctrine` | `refs/heads/fabric/integration-m3-135bbd9` @ `135bbd932553613fa3dd758d73143401068a4cb8` (tree `4e724089…`) |
| Fabric `main` on `doctrine` | **Not** RG substrate | Observed divergent (`fc1c604`); `DOCTRINE_MAIN_IS_RG_FABRIC_DEPENDENCY=false` |
| M3 Lane E qualification | **Input only** | `fc62c375` merged into `135bbd9`; does not imply RG qualified |
| EffectGuard product | **Adjacent qualified product** | `b7c5368` / tree `1ae7c50a…` @ `effectfence` tag `effectguard-v1-qualified` |
| EffectGuard qualification | **Does not transfer** | `EFFECTGUARD_QUALIFIED=true` ≠ `RECOVERY_GOVERNOR_QUALIFIED` |

## Bridge topology @ IPQ-1 subject

```text
recovery-governor (CaseStore)
    └── effectguard::SessionStore   [path workspace dep → pin @ execution]
            └── fabric-m3-consequence @ 135bbd9
```

RG **must not** treat workspace path resolution as implicit qualification of the bridge commit. Execution records `EFFECTGUARD_BRIDGE_SUBJECT_HEAD=b7c5368` with `git rev-parse` on the resolved `effectguard` crate checkout.

## Global vs local publication

- Fabric integration ref is **globally authoritative** per programme manifest (`fabric_integration_remotely_published=true` on named ref).
- Recovery Governor remote `mc8h7dxz6t-ui/recovery-governor` is **product repo** authority only; it does not republish Fabric.

## LE-F visibility

```text
LE_F_001_STATUS=OPEN_VISIBLE
LE_F_002_STATUS=OPEN_VISIBLE
RG_QUALIFICATION_CLOSES_LE_F=false
```

RG qual cannot activate trust-boundary export or M3-X passport claims.
