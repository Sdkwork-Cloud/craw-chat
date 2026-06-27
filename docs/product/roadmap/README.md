# Sdkwork IM Product Roadmap

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-26
Specs: DOCUMENTATION_SPEC.md, REQUIREMENTS_SPEC.md

## Phase 1 — Core IM (Current)

- [x] 1:1 and group chat with real-time delivery
- [x] Conversation and message CRUD with dual-token auth
- [x] WebSocket-based CCP real-time protocol
- [x] Postgres-backed projection and conversation stores
- [x] Multi-app support (PC, H5, Flutter mobile)
- [x] IAM dual-token verification and tenant isolation
- [x] Audit ledger with hash chaining

## Phase 2 — Productivity & Compliance

- [ ] Message recall and edit with tombstone propagation
- [ ] @mention with user picker across all clients
- [ ] Message reactions, pins, and threads
- [ ] File and media sharing via SDKWork Drive integration
- [ ] Full-text message search with Postgres FTI
- [ ] Read receipts and typing indicators
- [ ] Audit Postgres persistence for compliance durability

## Phase 3 — Scale & Enterprise

- [ ] Multi-node session-gateway cluster with Redis route bus
- [ ] Cross-tenant shared channels with sync runtime
- [ ] Rate limiting and quota management
- [ ] Observability dashboards (Prometheus + Grafana)
- [ ] SBOM and supply-chain attestation for releases
- [ ] Multi-region deployment topology

## Phase 4 — AI & Automation

- [ ] Agent-backed automation streams
- [ ] AI-assisted message summarization
- [ ] Smart reply and draft suggestions
- [ ] Governance and audit for AI interactions

## Milestone Tracking

Each phase deliverable maps to a `PLAN-*.md` card under `docs/product/roadmap/` with six-question closure verification.
