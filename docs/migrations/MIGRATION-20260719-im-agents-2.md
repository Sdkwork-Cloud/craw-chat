# IM Agents 2.0 Migration Record

1. Paired `0005_agents_integration_expand` creates the three IM-owned tables.
2. Projection, binding, durable dispatch, lease recovery, timeout reconciliation,
   and atomic reply correlation are implemented and covered by store/worker tests.
3. Assignment replay remains sourced from `im_commit_journal`; payload hashes and
   aggregate versions fence duplicate or conflicting projection writes.
4. Public Agents runtime facade dispatch uses the fixed
   `service.sdkwork-im.agent-dispatch` service principal and owner-user scope.
5. Integration subject columns are BIGINT from creation. Decimal-string inputs
   from existing IM contracts are signed-range checked before conversion, and
   paired `0006_agents_integration_subject_guard` CHECK constraints reject
   invalid scope or actor values after online validation.
6. Database contract `2.0.0` is active with immutable migration provenance in
   `specs/database-table-registry.json`.

Rollback disables dispatch first. The down migration fails when integration rows
exist, preventing accidental loss of dispatch/audit history.

Any future in-place conversion of an existing TEXT subject column is a separate
reviewed migration and must use expand/dual-write/backfill/contract phases. No
legacy TEXT integration column exists in `0005`, so no synthetic shadow column
or destructive backfill is part of this activation.
