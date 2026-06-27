> Migrated from `docs/部署/database-table-naming-standard.md` on 2026-06-24.
> Owner: SDKWork maintainers

# SDKWork Chat database table naming standard

This document narrows the root SDKWork `DATABASE_SPEC.md` for the `chat` app.

## Canonical prefix

`im_` is the controlled business module prefix for SDKWork Chat tables whose
source of truth is instant messaging: conversations, messages, realtime
delivery, presence, route binding, RTC signaling, message projections,
IM-specific notifications, IM-specific automation, and stream frames.

Examples:

| Table family | Purpose |
| --- | --- |
| `im_conversation_*` | conversation and message facts |
| `im_message_*` | message attachments and media references |
| `im_realtime_*` | device event windows, checkpoints, subscriptions, and disconnect fences |
| `im_presence_*` | online/offline device presence |
| `im_route_*` | realtime route ownership |
| `im_rtc_*` | RTC session and signal state |
| `im_projection_*` | IM read models and sync projections |
| `im_stream_*` | streaming response sessions and frames |

## Non-IM tables

Tables that are not owned by the instant messaging bounded context must not be
renamed to `im_`. IAM, Drive, integration, ops, billing, notification-platform,
or other product/platform tables keep their own business-domain prefix or
approved legacy name. The `im_` prefix is a business-domain marker, not a
product-wide or app-wide default.

When a table is ambiguous, use the system of record:

| System of record | Prefix |
| --- | --- |
| Chat message/realtime state | `im_` |
| Platform IAM/user/session authority | IAM-owned prefix, not `im_` |
| Drive file/object lifecycle authority | Drive-owned prefix, not `im_` |
| Generic notification platform queue | notification-owned prefix, not `im_` |
| IM notification derived from chat delivery | `im_notification_*` |
| Generic automation platform history | automation-owned prefix, not `im_` |
| IM automation execution against chat flows | `im_automation_*` |

## Registry artifacts

The local registry artifacts are the source of truth for this app:

- `specs/database-prefix-registry.json` registers `im` as the active
  instant-messaging prefix and records forbidden aliases such as `chat_`,
  `craw_`, `sdkwork_`, `app_`, `sys_`, `common_`, and `comms_`.
- `specs/database-table-registry.json` lists every checked-in IM table, table
  profile, write owner, and migration path.
- `scripts/dev/sdkwork-im-database-naming-standard.test.mjs` scans the
  PostgreSQL migration and Rust SQL contracts so new IM tables cannot drift away
  from `im_` or skip registry registration.

Desktop SQLite keeps the same logical table prefix policy when it materializes
IM tables locally. This does not change the desktop database file policy:
desktop data remains in `~/.sdkwork/chat/data/chat.sqlite` or the Windows
equivalent `%USERPROFILE%/.sdkwork/chat/data/chat.sqlite`.

## Temporary checks

Deployment guides may use a short-lived table such as
`sdkwork_ai_dev.__manual_smoke_check` only for manual PostgreSQL connectivity
verification. These tables are not IM business tables, must not be registered
in `specs/database-table-registry.json`, must not be created by checked-in
migrations, and must be dropped by the same manual smoke procedure.

