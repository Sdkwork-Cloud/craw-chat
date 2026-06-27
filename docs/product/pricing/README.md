# Sdkwork IM Commercialization & Pricing

Status: draft
Owner: SDKWork maintainers
Updated: 2026-06-26
Specs: DOCUMENTATION_SPEC.md, RELEASE_SPEC.md

## Overview

Sdkwork IM is a multi-tenant real-time messaging platform designed for commercial deployment. This document outlines the pricing model, packaging strategy, and operational cost structure.

## Editions

| Edition | Target | Session-gateway | Projection | Audit | Support |
| --- | --- | --- | --- | --- | --- |
| Community | Self-hosted, small teams | Single-node | In-memory | In-memory | Community |
| Professional | Growing teams | Single-node | Postgres | Postgres (planned) | Business |
| Enterprise | Large organizations | Multi-node cluster | Postgres HA | Postgres HA | Enterprise |

## Pricing Model

### Professional (per tenant)

- Base: includes up to 100 concurrent users per tenant
- Overage: per additional concurrent user
- Storage: per GB of message and media storage
- Features: full chat, @mention, search, file sharing

### Enterprise (per tenant)

- Base: includes up to 1,000 concurrent users per tenant
- Overage: per additional concurrent user
- Storage: per GB of message and media storage
- Features: all Professional features plus multi-node HA, cross-tenant shared channels, audit export, SSO/SAML
- SLA: 99.9% uptime with 1-hour response time

## Operational Cost Drivers

| Component | Cost driver | Scaling factor |
| --- | --- | --- |
| session-gateway | WebSocket connections, message throughput | Linear with active users |
| projection-service | Read query volume, snapshot size | Linear with conversation count |
| Postgres | Storage, IOPS, connections | Linear with message volume |
| Redis | Connections, memory | Linear with realtime cluster size |
| Media storage (SDKWork Drive) | File storage, bandwidth | Linear with file sharing volume |

## Packaging

### Self-hosted (Community)

- Docker image: `registry.sdkwork.com/apps/chat`
- Helm chart for Kubernetes deployment
- Configuration via environment variables

### Cloud-hosted (Professional/Enterprise)

- Managed service with tenant isolation
- SDKWork Drive integration for media
- SDKWork IAM integration for authentication
- SDKWork Drive integration for media storage

## Billing Integration

Billing is handled through the SDKWork platform billing system. Tenant usage metrics are collected via:
- WebSocket connection duration (session-gateway)
- Message volume (projection-service)
- Storage consumption (SDKWork Drive)
- API call count (all services)

## Roadmap Dependencies

- Audit Postgres persistence (Phase 2) — required for Enterprise compliance features
- Multi-node cluster (Phase 3) — required for Enterprise HA SLA
- Rate limiting and quota (Phase 3) — required for overage billing
