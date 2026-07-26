# sdkwork-api-im-standalone-gateway Specs

`component.spec.json` is the machine-readable contract for the canonical SDKWork IM standalone
gateway. It declares the `runtime-gateway` role, single `application.public-ingress` surface,
source-config keys, global standards, and verification commands.

The gateway owns process startup, one HTTP listener, framework infrastructure, assembly composition,
required dependency readiness, and graceful shutdown. API contracts, business rules, persistence
schemas, and generated SDKs remain in their owning components.

The embedded Agents App API is consumed through
`sdkwork_api_agents_assembly::assemble_app_runtime_contribution`. Its HTTP contribution and
in-process session facade are built from one owner-managed repository state; the gateway binds the
route manifest and domain-context injector before mounting the router.

The framework-owned readiness surface must compose IM database, configured Redis, embedded Agents,
registered worker, and realtime plane checks. This directory narrows no global health or security
rule; `HEALTH_CHECK_SPEC.md` and `SECURITY_SPEC.md` remain authoritative.
