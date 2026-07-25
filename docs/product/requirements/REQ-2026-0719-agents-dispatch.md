# REQ-2026-0719 IM Agents Dispatch

- Owner: im-platform
- Status: accepted
- Priority: P0
- Review: human-approved 2026-07-19

## Requirement

Provide durable normalized Conversation-to-Agent assignments, an opaque Agents
Session binding, and source-Message-to-Agents-Turn-to-visible-reply dispatch
correlation without making Agents depend on IM or writing Agents-owned tables.

## Acceptance Criteria

- IM owns and migrates `im_conversation_agent_assignments`,
  `im_conversation_agent_binding`, and `im_agent_dispatch`.
- Assignment generations use compare-and-set replacement and stale generations
  cannot replace current state. Ordinary reads query normalized assignments
  directly; journal replay is reserved for explicit offline recovery and audit.
- Dispatch is idempotent per source message/agent/generation, lease-safe,
  retryable, reconcilable after timeout, and dead-letter capable.
- Agents calls use the public canonical `AgentsSessionFacade` with one trusted
  service context; IM does not expose or retain a second chat/session facade,
  raw HTTP client, or manual auth header path.
- Visible replies use the existing IM message sequence allocator and outbox.
- Tenant/organization isolation, timeout reconciliation, retry, cancellation,
  retention, and cross-module SQL denial tests pass.

## Traceability

- Spec: `specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`
- Decision: `docs/architecture/decisions/ADR-20260719-im-agents-dispatch.md`
- Runtime: `services/sdkwork-comms-conversation-service/src/runtime/agents.rs`
  and `crates/sdkwork-api-im-standalone-gateway/src/main.rs`
- Persistence: `adapters/postgres-journal/src/agent_integration_store.rs`
- Verification: `pnpm test:agents-integration-migration`,
  `pnpm test:normalized-im-authority-standard`, IM database contract gates,
  Rust dispatch tests, and standalone gateway compilation.
