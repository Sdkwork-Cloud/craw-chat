# Feature Overview

This section answers two practical questions:

1. What can the current application do today?
2. Which capabilities are delivered runtime behavior versus governance, operations, or SDK
   boundary documentation?

## Capability Domains

### App collaboration

- conversation creation
- agent dialog, handoff, and system-channel flows
- membership changes, ownership transfer, role changes, and leave
- message send, edit, recall, and timeline reads
- inbox, conversation summary, and read-cursor views

### Realtime and session delivery

- client route resume and disconnect
- presence heartbeat and current-presence reads
- realtime subscription sync, poll windows, ack flow, and websocket upgrade
- client route registration and client-route event window reads

### Media, streaming, and calls

- media upload initiation, completion, lookup, signed download URL, and attachment
- stream open, frame append, list, checkpoint, complete, and abort
- IM call create, invite, accept, reject, end, signal, and RTC participant credential handoff

### Platform and operations

- notifications and automation execution
- audit record creation, listing, and export
- ops health, cluster, lag, replay status, runtime directory, provider bindings, drift, and diagnostics

### Governance and extension

- provider health for media, RTC media runtime, principal-profile, and AIoT integration
- AIoT uplink and downlink ingestion through sdkwork-aiot
- protocol registry and governance
- provider registry and policy lifecycle
- node drain, activate, and route migration

## Maturity Model Used In These Docs

::: info Documentation maturity rule
Capability maturity here means "implemented and verifiable in the current repository". It does not
mean roadmap intent or naming-only presence.
:::

- Implemented runtime capability: exposed as a route or script and backed by tests or executable
  runtime wiring.
- Implemented governance capability: exposed by the control plane and validated by control-plane
  tests.
- Repo package or workspace contract: package metadata, import names, generation wrappers, or
  workspace directories exist, but publication state still comes from the release catalog.
- Materialized SDK boundary: checked-in OpenAPI authority, generated packages, or assembled
  consumer packages already exist and can be verified in-repo.
- Scaffolded SDK boundary: the workspace contract is reserved and named, but one or more language
  lines are not yet materialized, verified, or publication-ready.

## What To Read Next

- Continue with [Capability Matrix](/features/capabilities) when you need the evidence-oriented
  implementation breakdown.
- Switch to [API Reference](/api-reference/index) when you need route-level request or response
  semantics.
- Switch to [SDK Overview](/sdk/index) when you need import names, language surfaces, or publication
  boundaries.
