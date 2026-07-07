# Release Documentation

Status: active
Owner: SDKWork IM maintainers
Specs: `../sdkwork-specs/RELEASE_SPEC.md`, `../sdkwork-specs/SUPPLY_CHAIN_SECURITY_SPEC.md`, `../sdkwork-specs/QUALITY_GATE_SPEC.md`, `../sdkwork-specs/DOCUMENTATION_SPEC.md`

## Purpose

`docs/release/` records release changelogs, release notes, go/no-go decisions, commercial readiness evidence, and rollout or rollback conclusions for SDKWork IM. It is an evidence layer, not a place to lower SDKWork standards or preserve compatibility debt.

## Required Files

Each release or pre-release verification loop maintains:

- `docs/release/CHANGELOG.md`
- `docs/release/YYYY-MM-DD-vX.Y.Z-loop-XX.md`

Add these when a release decision is made:

- `docs/release/YYYY-MM-DD-vX.Y.Z-release-notes.md`
- `docs/release/YYYY-MM-DD-vX.Y.Z-go-no-go.md`

## Version Rules

- Before commercial GA, versions stay in the `0.y.z` range.
- `minor`: new capability, completed wave/step closure, or externally visible enhancement.
- `patch`: bug fix, standard alignment, documentation, test, script, or non-breaking optimization.
- `major`: breaking contract change or formal commercial GA.
- `1.0.0` is allowed only after the commercial release gate passes with real evidence.

## Changelog Requirements

Every changelog entry must include:

- date and version;
- affected step, component, API, SDK, database, or deployment surface;
- behavior and contract impact;
- migration, downgrade, and rollback notes when applicable;
- verification commands and important outputs;
- documentation updates;
- remaining risks or blockers.

## Commercial Readiness Gate

Before release or go/no-go review, run:

```bash
node scripts/release/commercial-readiness.mjs
```

The gate must fail closed:

- `exit code 1`: code, config, dependency, package, or verification failure.
- `exit code 2`: implementation checks pass but required release evidence is incomplete.
- `exit code 0`: all required implementation checks and release evidence pass.

## Evidence Rules

Release evidence must be real and reproducible:

- If `security.checksumRequired=true`, every enabled direct distribution package must record a real SHA-256 checksum.
- If `security.signatureRequired=true`, every enabled direct distribution package must record signing, notarization, Sigstore, GPG, Authenticode, or equivalent signing evidence.
- If `security.sbomRequired=true`, every enabled direct distribution package must record CycloneDX/SPDX SBOM evidence plus provenance or attestation evidence.
- Enabled media assets under `media.icons`, `media.screenshots`, and `media.previews` must not retain `metadata.generatedPlaceholder=true`.
- Step 11 capacity evidence must include `capacity-tier-evidence-index.json` collected from real load, soak, failover, and resource-limit runs.
- Step 11 pre-release evidence must include `pre-release-tier-evidence-index.json` collected from real verification runs.
- UI release evidence that covers browser or desktop flows must include Playwright run output, screenshots, traces, or equivalent recorded artifacts from the exercised release candidate.

Missing checksums, signatures, SBOMs, provenance, attestations, media assets, or Step 11 evidence are release blockers. Do not replace them with empty values, placeholder strings, inferred values, or documentation claims.

## Go/No-Go Rule

Documentation must not state that SDKWork IM is commercially deliverable until:

- `node scripts/release/commercial-readiness.mjs` exits `0`;
- `sdkwork.app.config.json` and app-level manifests contain real required release evidence;
- no enabled package keeps null checksum/signature/SBOM evidence where the security policy requires it;
- no enabled release media keeps `metadata.generatedPlaceholder=true`;
- API, SDK, pagination, database, security, performance, and deployment gates relevant to the release have passed.
