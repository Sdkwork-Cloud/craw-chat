# SQLite baseline DDL

`0001_im_baseline.sql` provides **contract parity** for the SDKWork database lifecycle checker.

## Runtime authority

| Surface | Engine | Notes |
| --- | --- | --- |
| IM core (journal, projection, social PG materializer, message search) | **PostgreSQL only** | Production and staging require `SDKWORK_IM_DATABASE_ENGINE=postgresql` |
| Desktop `~/.sdkwork/chat/data/chat.sqlite` | SQLite | Gateway webstore (audit, idempotency, rate limits) and sibling module DB files |
| `pnpm dev:*:sqlite` | In-memory IM | IM services log ephemeral authority; data is not durable across restarts |

Message search in this baseline uses `search_text` + SQLite triggers. PostgreSQL runtime uses `search_vector` + GIN (see postgres baseline).
