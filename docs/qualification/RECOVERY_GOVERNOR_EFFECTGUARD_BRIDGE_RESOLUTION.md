# EffectGuard bridge resolution @ RG IPQ-1 subject

## Frozen RG subject

| Token | Value |
| --- | --- |
| `RG_IPQ1_SUBJECT_HEAD` | `4d6cb052fd99f4e5678d25c4274615cd47681751` |
| `RG_IPQ1_PRODUCT_MATERIAL_HEAD` | `d90d92d008b02dd6f5f5a26b4008b43374ef000e` |

## EffectGuard qualified subject (programme)

| Token | Value |
| --- | --- |
| `EFFECTGUARD_QUALIFIED_SUBJECT_HEAD` | `b7c53685208138a981d7aa58f6caf41ceaa3a35d` |
| `EFFECTGUARD_QUALIFIED_SUBJECT_TREE` | `1ae7c50a149c7859408a516d59ce45a4572daa3f` |
| Tag | `effectguard-v1-qualified` → `b7c5368` |

## Workspace dependency @ `d90d92d`

```toml
effectguard = { path = "../effectfence/crates/effectguard" }
```

**Resolution:** path dependency resolves to **whatever commit is checked out** in `~/Projects/effectfence` at build time — **not** automatically `b7c5368`.

## IPQ-1 execution requirement (frozen)

| Rule | Value |
| --- | --- |
| `EFFECTGUARD_BRIDGE_SUBJECT_HEAD` | `b7c53685208138a981d7aa58f6caf41ceaa3a35d` |
| `EFFECTGUARD_BRIDGE_SUBJECT_TREE` | `1ae7c50a149c7859408a516d59ce45a4572daa3f` |
| Evidence | `git -C ../effectfence rev-parse HEAD` recorded in bundle |
| `RG_INHERITS_EFFECTGUARD_QUALIFIED` | `false` |
| `RG_USES_EFFECTGUARD_SESSION_BRIDGE` | `true` |

Execution **must** checkout `effectfence` @ `b7c5368` (or equivalent git dependency pin) before Class B / governed dispatch tests.
