# sdkwork-comms-conversation-internal-rpc-bin

Phase 1 internal gRPC host for conversation service-to-service RPC dispatch. Shares the IM RPC bootstrap (`sdkwork-im-rpc-service-rust`) and optional discovery registration through `SDKWORK_IM_DISCOVERY_ENDPOINT`.

Environment:

- `SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_BIND_ADDR` — listener bind address
- `SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_PUBLIC_ENDPOINT` — optional advertised endpoint
- `SDKWORK_IM_DATABASE_URL` — required in production for postgres-backed runtime paths
