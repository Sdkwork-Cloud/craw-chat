# Sdkwork IM Audit Report (Retired)

The HTML audit report in this directory was a one-off Trae Work snapshot from early 2026. It described issues that have since been remediated or superseded by automated gates.

Do not use `sdkwork-im-audit-report.html` for release decisions.

## Current verification authority

Pre-launch alignment is enforced by repository gates:

- `pnpm run test:production-security-standard` — production JWT and dev-secret fail-closed policy
- `pnpm run test:app-context-module-standard` — single `im-app-context` implementation surface
- `pnpm run test:flutter-drive-standard` — Flutter Drive/IM SDK integration (no manual scope headers)
- `pnpm run test:chat-drive-upload-attribution-standard` — canonical chat upload attribution
- `node scripts/run-sdkwork-im-standards-verification.mjs` — full standards sweep
- `pnpm run check:commercial-readiness` — install, build, lint, and mobile analyze/test bundle

Canonical integration rules: `specs/im-app-api-sdk-integration.spec.md`.
