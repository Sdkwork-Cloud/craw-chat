# IM Agents 2.0 Migration Record

Status: historical (superseded by baseline ownership)

This record documents the original rollout of the IM-to-Agents integration
schema. The additive migrations referenced below have since been **squashed
into the immutable baseline** `database/ddl/baseline/postgres/0001_im_baseline.sql`
and removed from `database/migrations/postgres/`. The current provenance for
`im_conversation_agent_assignments`, `im_conversation_agent_binding`, and
`im_agent_dispatch` is the baseline file (see `database/contract/table-registry.json`).

Original rollout intent (preserved for audit history):

1. The three IM-owned tables carry the target BIGINT subject profile with
   range-safe guards enforced in the baseline: subject guard
   (`tenant_id > 0 AND organization_id >= 0 AND assigned_by > 0`,
   `created_by > 0 AND updated_by > 0`, `requested_by > 0`) and the
   system-actor compatibility relaxation (`assigned_by >= 0`).
2. Assignment state, binding, durable dispatch, lease recovery, timeout
   reconciliation, and atomic reply correlation are implemented and covered by
   store/worker tests.
3. Assignment replay remains sourced from `im_commit_journal`; payload hashes and
   aggregate versions fence duplicate or conflicting assignment writes.
4. Public Agents runtime facade dispatch uses the fixed
   `service.sdkwork-im.agent-dispatch` service principal and owner-user scope.
5. Database contract `2.1.0` is active with immutable baseline provenance in
   `database/contract/table-registry.json`.

Rollback disables dispatch first. Any future in-place conversion of an existing
subject column requires a separate reviewed migration. No legacy TEXT
integration column exists in the baseline, so no compatibility column, shadow
table, dual write, or destructive backfill is part of this activation.
