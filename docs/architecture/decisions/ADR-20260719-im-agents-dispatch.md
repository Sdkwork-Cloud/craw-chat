# ADR-20260719 IM To Agents Dispatch Ownership

- Status: accepted
- Date: 2026-07-19
- Owner: im-platform

## Decision

IM consumes Agents through public Agents SDK/facade surfaces. IM materializes
assignment events from `im_commit_journal`, binds each conversation-agent
generation to an opaque Agents session id, and records dispatch correlation in
IM-owned tables. Agents owns execution sessions, turns, messages, inference, and
usage; IM owns the visible timeline, retry, reply publication, and correlation.

No cross-module write, foreign key, generated transport merge, or reverse
Agents-to-IM dependency is permitted.

## Rollout

The three IM-owned tables (`im_conversation_agent_assignments`,
`im_conversation_agent_binding`, `im_agent_dispatch`) use the target BIGINT
subject profile and live in the immutable baseline
(`database/ddl/baseline/postgres/0001_im_baseline.sql`) with their range-safe
guards. The original additive migrations (`0005_agents_integration_expand` and
follow-ups) were squashed into the baseline and removed from
`database/migrations/postgres/`. Existing IM TEXT subject columns migrate
through a separately reviewed expand/backfill/contract sequence before the
`2.0.0` contract is declared complete. Until then, source/reply message
identity is validated by the IM service rather than a cross-type physical
foreign key.

## Consequences

Timeout is indeterminate and requires Agents idempotency/turn reconciliation.
Late results from stale assignment generations cannot become visible replies.
All visible message writes continue through normal IM commit and sequence paths.
