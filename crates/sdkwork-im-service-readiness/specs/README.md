# sdkwork-im-service-readiness Specs

`component.spec.json` declares the shared IM process-readiness composition contract. This crate
provides `ReadinessCheck` values to runtime hosts and does not own probe routes, business health APIs,
or domain state.

Global `HEALTH_CHECK_SPEC.md`, `SECURITY_SPEC.md`, and `WEB_FRAMEWORK_SPEC.md` remain authoritative.
Host-specific required checks extend this component through the public composition function rather
than introducing local probe handlers or an alternate readiness registry.
