# sdkwork-comms-conversation-rpc-bin

Phase 1 gRPC host for conversation write/read RPC surfaces. Registers a service-scoped IM RPC router (`ConversationService`, `MessageService`) with gRPC health checks and runtime delegation through `ConversationRpcDispatcher`.

Environment:

- `SDKWORK_IM_COMMS_CONVERSATION_RPC_BIND_ADDR` — listener bind address (default `127.0.0.1:50052`)
- `SDKWORK_IM_COMMS_CONVERSATION_RPC_PUBLIC_ENDPOINT` — optional advertised endpoint for topology manifests
- `SDKWORK_IM_DATABASE_URL` — required in production for postgres commit journal
- `SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH` — principal directory JSON for app-session auth
