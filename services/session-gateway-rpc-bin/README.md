# session-gateway-rpc-bin

Phase 1 gRPC host for session-gateway realtime RPC surfaces. Registers a service-scoped IM RPC router (`PresenceService`, `RealtimeService`) with gRPC health checks and runtime delegation through `SessionGatewayRpcDispatcher`.

Environment:

- `SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR` — listener bind address (default `127.0.0.1:50051`)
- `SDKWORK_IM_SESSION_GATEWAY_RPC_PUBLIC_ENDPOINT` — optional advertised endpoint for topology manifests
