# REQ-2026-0719 IM Agents Dispatch

- Owner: im-platform
- Status: approved
- Priority: P0
- Review: human-approved 2026-07-19

## Requirement

Provide durable conversation-agent assignment projection, Agents session
binding, and source-message-to-Agents-turn-to-visible-reply dispatch correlation
without making Agents depend on IM or writing Agents-owned tables.

## Acceptance Criteria

- IM owns and migrates `im_projection_conversation_agent`,
  `im_conversation_agent_binding`, and `im_agent_dispatch`.
- Assignment replay is ordered and stale generations cannot replace current
  state.
- Dispatch is idempotent per source message/agent/generation, lease-safe,
  retryable, reconcilable after timeout, and dead-letter capable.
- Agents calls use public generated/composed SDKs with one trusted service
  credential context; raw HTTP and manual auth headers are absent.
- Visible replies use the existing IM message sequence allocator and outbox.
- Tenant/organization isolation, timeout reconciliation, retry, cancellation,
  retention, and cross-module SQL denial tests pass.

## Traceability

- Spec: `specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`
- Decision: `docs/architecture/decisions/ADR-20260719-im-agents-dispatch.md`
- Verification: IM database, dependency, SDK consumer, Rust integration, and
  release gates.
