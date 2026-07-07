# IM RPC Availability Matrix

**Updated**: 2026-07-07  
**Authority**: `crates/sdkwork-im-rpc-service-rust/src/service_manifest.rs`, Phase 1 RPC hosts under `services/*-rpc-bin`.

## Summary

HTTP + WebSocket is the **production client path** for all IM surfaces. gRPC is Phase 1 partial: only services with a dedicated `*-rpc-bin` host are callable at runtime.

## App RPC (`sdkwork.communication.app.v3.*`)

| Service | Unary | Server-stream | Runtime host |
| --- | --- | --- | --- |
| `PresenceService` | ✅ | ✅ `WatchPresence` | `session-gateway-rpc-bin` |
| `RealtimeService` | ✅ | ✅ `WatchRealtimeEvents` | `session-gateway-rpc-bin` |
| `ConversationService` | ✅ | ❌ unimplemented (use session-gateway WS) | `sdkwork-comms-conversation-rpc-bin` |
| `MessageService` | ✅ | ❌ unimplemented | `sdkwork-comms-conversation-rpc-bin` |
| `RoomService` | ✅ | ❌ unimplemented | `sdkwork-comms-conversation-rpc-bin` |
| `ContactService` | ❌ no host | ❌ | Use HTTP open-api / app-sdk |
| `SocialService` | ❌ no host | ❌ | Use HTTP open-api / app-sdk |
| `StreamService` | ❌ no host | ❌ `WatchStreamFrames` | Use HTTP + WS |
| `CallService` | ❌ no host | ❌ `WatchCallSignals` | Use HTTP `/im/v3/api/calls/*` + WS |
| `NotificationService` | ❌ no host | ❌ `WatchNotifications` | Use HTTP + WS |
| `AutomationService` | ❌ no host | ❌ | Use HTTP open-api |

## Internal RPC (`sdkwork.communication.internal.v1.*`)

| Service | Status |
| --- | --- |
| `RoomOrchestrationService` | ✅ `sdkwork-comms-conversation-internal-rpc-bin` |
| `MessageDispatchService` | ✅ `sdkwork-comms-conversation-internal-rpc-bin` |
| `RuntimeTopologyService`, `RouteLeaseService`, `DomainEventRelayService` | Contract-only (Phase 2); **do not call** |

## Backend admin RPC (`sdkwork.communication.backend.v3.*`)

All admin RPC services are **contract-only** until dedicated admin rpc-bin hosts ship. Use HTTP backend-sdk / admin open-api.

## Client integration guidance

1. **Mobile / PC / H5**: consume `@sdkwork/im-app-sdk` over HTTP; realtime via WebSocket through session-gateway.
2. **gRPC SDK**: limit to `PresenceService`, `RealtimeService`, and conversation unary ops bound to Phase 1 rpc-bin topology env vars.
3. **Do not** assume manifest-listed services are reachable without verifying the host column above.

## Verification

```bash
cargo check -p session-gateway-rpc-bin -p sdkwork-comms-conversation-rpc-bin
node scripts/dev/sdkwork-im-rpc-contract.test.mjs
node scripts/dev/sdkwork-im-session-gateway-rpc-bin.test.mjs
node scripts/dev/sdkwork-im-comms-conversation-rpc-bin.test.mjs
node scripts/dev/sdkwork-im-comms-conversation-internal-rpc-bin.test.mjs
```
