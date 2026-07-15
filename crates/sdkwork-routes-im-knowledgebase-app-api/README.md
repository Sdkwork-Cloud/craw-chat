# sdkwork-routes-im-knowledgebase-app-api

## Purpose

HTTP route crate for the SDKWork IM group knowledgebase `app-api` surface at
`/app/v3/api/chat/conversations/{conversationId}/knowledgebase`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Path constants (`paths.rs`)
- Route manifest metadata (`manifest.rs`)
- Axum route mounting (`routes.rs`)
- IM web-framework wrapping (`web_bootstrap.rs`)

## Forbidden Content

- Business logic, persistence, or Knowledgebase transport clients
- Raw HTTP credential parsing outside `sdkwork-web-framework`
- Generated SDK imports for the same App API authority

## Verification

```bash
cargo test -p sdkwork-routes-im-knowledgebase-app-api
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
