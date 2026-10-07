# Recovery Governor customer claim map

**Build:** `QUALIFIED_BOUNDED_V1` · **IPQ-1:** `PASS_WITH_BOUNDED_LIMITATIONS` · **Evaluated:** 2026-10-07  
**Qualified subject:** `4d6cb052fd99f4e5678d25c4274615cd47681751`  
**Fabric scope:** `GLOBALLY_AUTHORITATIVE_NAMED_INTEGRATION_REF` · `doctrine` / `fabric/integration-m3-135bbd9` @ `135bbd9…`

## What we may say (bounded qualified)

| Customer-facing statement | Claim ID | Allowed? | Evidence |
| --- | --- | --- | --- |
| Recovery Governor projects advisory SafeNext packs without mutating Fabric canonical truth (exercised scope). | RG-CL-001, RG-CL-002 | **Yes (bounded)** | RG-H01..H04, compile-time flags |
| Governed dispatch is gated on EffectGuard / Fabric clearance paths exercised in IPQ-1. | RG-CL-003 | **Yes (bounded)** | RG-H03, bridge_results |
| Blind retry / ack-without-effect paths hold operator in HOLD (exercised hostility). | RG-CL-004 | **Yes (bounded)** | RG-H11, RG-H12 |
| Orchestration freshness is load-bearing for advisory revalidation (bounded). | RG-CL-017 | **Yes (bounded)** | RG-F slice + revalidate_advisory tests |

## What we must NOT say

| Statement | ID | Ceiling |
| --- | --- | --- |
| Universal process restart / crash-safe recovery | RG-NC-006 | `RG_PROCESS_RESTART_CASE=EXCLUDED_BOUNDED_LIMITATION` |
| Live GitHub RG-R recovery path qualified | RG-NC-016 | Class I `NOT_EXECUTED` |
| EffectGuard or Fabric qualification implies RG qualified | RG-NC-011, RG-NC-012 | Independent IPQ-1 only |
| M3-X / trust-boundary export while LE-F-002 open | RG-NC-009 | LE-F-002 |
| Programme-wide product crown / provider-neutral qual | RG-NC-007, RG-NC-018 | programme non-claims |
| `doctrine/main` is Fabric integration pin | — | Named ref only |

## Substrate (reference)

Fabric M3 qualified input @ `135bbd9…`; EffectGuard bridge @ `b7c5368…`. Recovery Governor qualification does not re-run Fabric Lane E or EffectGuard IPQ-1.
