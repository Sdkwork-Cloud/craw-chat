# 152CJ - Current Architecture As-Built Alignment - 2026-04-09

## 1. Purpose

This document is the stable as-built alignment anchor for the `150CJ` / `151CJ` IM social-space and conversation-domain architecture work.

Later files named `152CJ-Loop*.md` are incremental addenda. They do not replace this anchor; they refine it with loop-specific evidence and deferred work.

## 2. Current As-Built State

- `sdkwork-im-server` exposes the direct-chat binding route used by the IM app SDK contract.
- Direct-chat binding retries are treated as an idempotent business operation instead of a missing-route gap.
- Contacts, friend requests, preferences, tags, recommendations, reaction summaries, and pin summaries are SDK-backed runtime capabilities rather than local UI seed data.
- Conversation runtime membership and social durable truth remain separate boundaries: conversation membership is a runtime projection, while friendship/contact state is owned by social-domain APIs.
- Current app-side directory reads are served by appbase IAM app APIs and must not fall back to demo directory rows.

## 3. Direct Chat Binding Alignment

The direct-chat binding capability is no longer an architecture gap for `sdkwork-im-server`.

Current expected behavior:

- duplicate direct-chat binding requests are idempotent;
- the route remains under the IM app API surface;
- UI and SDK consumers must use the generated SDK or approved app SDK facade;
- tests must not preserve old claims that the dedicated route is absent.

## 4. Addenda

Use the `152CJ-Loop*.md` files for post-2026-04-09 refinements. This file remains the canonical current-state anchor referenced by architecture indexes and commercial gate tests.
