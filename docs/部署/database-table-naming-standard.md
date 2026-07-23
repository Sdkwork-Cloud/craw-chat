# SDKWork IM Database Table Naming Standard

This document narrows the root SDKWork `DATABASE_SPEC.md` for the `im` module.

## Canonical Prefix

`im_` is the controlled prefix for tables whose system of record belongs to instant messaging:
Conversation, Message, Member, ReadCursor, realtime delivery, presence, routing, call signaling,
IM-specific notification/automation state, social relationships, and ordered streams.

| Table family | Purpose |
| --- | --- |
| `im_conversation_*` | Conversation, Message, membership, sequence, preference, and read state |
| `im_message_*` | Message media references and typed interactions |
| `im_realtime_*` | Device event windows, checkpoints, subscriptions, and disconnect fences |
| `im_presence_*` | Online/offline device presence |
| `im_route_*` | Realtime route ownership |
| `im_rtc_*` | IM call-signaling state; media runtime remains owned by `sdkwork-rtc` |
| `im_stream_*` | Ordered application-data stream sessions and frames |

No active table uses the retired `im_projection_*` prefix. Normalized IM tables are business
authorities, not persisted projections, mirrors, compatibility views, or a second Message timeline.

## Non-IM Tables

Tables outside the instant-messaging bounded context must keep the owning domain prefix. IAM,
Agents, Drive, Knowledgebase, RTC media/provider runtime, billing, and generic platform tables are
external authorities and are not registered or created by this repository.

| System of record | Prefix policy |
| --- | --- |
| IM communication and realtime state | `im_` |
| IAM user, token, or Session authority | IAM-owned prefix |
| Agents Project, Session, Turn, Item, or Interaction | Agents-owned prefix |
| Drive file/object lifecycle | Drive-owned prefix |
| Knowledgebase content and binding authority | Knowledgebase-owned prefix |
| Generic notification or automation platform state | Owning platform prefix |

## Canonical Registry

The only authored database registries are:

- `database/contract/prefix-registry.json` for the `im_` prefix, ownership, pattern, and forbidden aliases.
- `database/contract/table-registry.json` for all 60 active IM tables, profiles, write owners, and migration provenance.
- `database/contract/schema.yaml` for the portable active schema inventory.

`specs/` must not contain copied table or prefix registries. The effective table set is the 57-table
PostgreSQL baseline plus the three IM-to-Agents integration tables from migration `0005`; every
canonical contract must describe exactly the same 60 names.

## Verification

```bash
pnpm db:contract:check
pnpm test:database-naming-standard
pnpm test:database-framework-standard
```

Deployment guides may create `sdkwork_ai_dev.__manual_smoke_check` only for a short-lived manual
connectivity test. It is not an IM business table, must not be registered, and must be dropped by
the same procedure.
