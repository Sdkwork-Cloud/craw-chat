> Migrated from `docs/架构/71-agent-handoff-closed-capability-gate-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 71. Agent Handoff Closed Capability Gate Standard (2026-04-06)

## 1. Objective

For a commercial IM platform, `agent_handoff.status = closed` cannot only mean:

- no more chat messages

It must also mean:

- no more new interactive conversation-bound capability writes

Otherwise the platform still allows operational work to continue after the handoff has been declared finished.

## 2. Scope

This standard applies to conversation-bound capabilities attached to a conversation lifecycle, specifically:

- `stream`
- `RTC`

It currently governs write paths only.

## 3. Rule

When all of the following are true:

- `conversationType = agent_handoff`
- durable handoff state exists
- `handoff.status = closed`

then the server must reject further conversation-bound write capabilities with:

- HTTP status: `409 Conflict`
- code: `conversation_conflict`

## 4. Covered Capability Writes

### 4.1 Stream

The following operations must be rejected for a closed handoff conversation:

- `stream.open`
- `stream.append`
- `stream.checkpoint`
- `stream.complete`
- `stream.abort`

### 4.2 RTC

The following operations must be rejected for a closed handoff conversation:

- `rtc.create`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- `rtc.signal`

## 5. Read Boundary

This standard does not yet require the server to reject read-only stream history access.

That boundary must be decided explicitly later:

- allow read for audit/debug/history
- or freeze all capability access once handoff is closed

Until that decision is frozen, only write paths are mandatory gates.

## 6. Architecture Rule

Lifecycle truth must stay in `conversation-runtime`, not be re-derived in leaf services.

Therefore:

- `conversation-runtime` owns the durable rule
- edge/profile access layers call into runtime lifecycle checks before allowing cross-service capability writes

This prevents:

- duplicated lifecycle logic in `streaming-service`
- duplicated lifecycle logic in `im-call-runtime`
- partial-side-effect mutations where sub-service state changes before conversation-layer rejection

## 7. Error Rule

The rejection must be surfaced as a conversation lifecycle conflict, not as a stream-specific or rtc-specific local validation error.

Reason:

- the root cause is not malformed stream/rtc input
- the root cause is conversation lifecycle state

So clients can correctly understand the failure as:

- the handoff is closed
- not merely a transport/session formatting problem

## 8. Test Standard

Implementations must include:

1. a test proving an existing conversation-bound stream write is rejected after handoff close
2. a test proving a new conversation-bound stream open is rejected after handoff close
3. a test proving an existing rtc mutation is rejected after handoff close
4. a test proving a new rtc session create is rejected after handoff close

At least one local-profile end-to-end implementation must carry these tests.

## 9. Non-Goals

This standard does not yet define:

- `agent_dialog` close/archive lifecycle
- `system_channel` scheduled or delegated publish lifecycle
- read-only stream/rtc visibility after close

Those remain future waves and must not weaken the write gate defined here.

