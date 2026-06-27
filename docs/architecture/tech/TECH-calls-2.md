> Migrated from `docs/sites/sdk/modules/calls.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Calls

## What This Module Is For

This module covers IM-owned call session lifecycle, signaling, and participant credential flows.
RTC provider media/runtime remains in `@sdkwork/rtc-sdk`; invite, accept, reject, end, and signal
delivery belong to `@sdkwork/im-sdk`.

## Public Entrypoints

- TypeScript: `sdk.calls.start(...)`, `sdk.calls.invite(...)`, `sdk.calls.accept(...)`,
  `sdk.calls.reject(...)`, `sdk.calls.end(...)`, `sdk.calls.sendSignal(...)`,
  `sdk.calls.issueParticipantCredential(...)`, `sdk.calls.watchIncoming(...)`, and
  `sdk.calls.subscribe(...)`.
- Route-aligned generated transports use the `calls` route group.

## API Mapping

The primary IM API alignment is [Calls](/api-reference/im/calls).

## Common Workflows

Typical flows include creating sessions, inviting participants, and posting signals.

## Ownership and Status

IM owns the call signaling protocol and conversation realtime delivery. RTC SDKs own media runtime,
provider selection, native driver bridge, and provider join behavior after IM authorization.

## Example

Use this page together with [Realtime And Presence](/api-reference/im/session-and-realtime) and
[Calls](/api-reference/im/calls).

