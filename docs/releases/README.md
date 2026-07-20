# SDKWork IM Release Contract

Status: DRAFT

Owner: SDKWork IM maintainers

## Authority

The repository-root `sdkwork.workflow.json` is the only release workflow authority. The root
`sdkwork.app.config.json` is the package inventory and commercial readiness declaration. PC, H5,
and Flutter child manifests describe their application surfaces; they do not own competing release
workflows.

The release matrix contains 18 real targets:

- PC Web and H5 browser ZIP bundles
- Standalone server archives for Linux, macOS, and Windows on x64 and arm64
- Tauri desktop ZIP bundles for Linux, macOS, and Windows on x64 and arm64
- Flutter Android APK and AAB packages
- Flutter iOS IPA package
- Cloud Kubernetes bundle produced from an immutable OCI image lock

No native Android, native iOS, Harmony, or Mini Program application root is declared. Capacitor is
deferred because `apps/sdkwork-im-h5/packages/sdkwork-im-h5-capacitor` has no Android or iOS host
project.

## Stage Boundaries

Build, package, sign, SBOM/provenance, validate, publish, and deploy are separate workflow phases.
Package jobs do not deploy. Deploy jobs consume immutable artifact evidence and do not rebuild.
GitHub Release publication is disabled while the application remains DRAFT.

Every package remains disabled with `releaseBuildDeferred: true` until its real artifact and scoped
evidence exist. DRAFT status does not weaken publish or deployment gates. Signing, checksum, SBOM,
provenance, OCI digest, approval, environment, rollback target, and store evidence must never be
invented.

## Toolchain Gates

- Android APK/AAB packaging requires a working Flutter/Android SDK and release keystore material.
  The build rejects debug signing. The local Android NDK installation must be repaired outside this
  repository if its `source.properties` is absent.
- iOS IPA packaging requires a macOS runner, Xcode, certificate, provisioning profile, export options,
  and the applicable notarization or store approval. Windows cannot produce the IPA.
- Cloud packaging requires `SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE` containing registry-resolved OCI
  digests for the current source revision. Source Kubernetes templates are not deployable artifacts.
- Cloud deploy apply remains fail-closed until an approved Kubernetes deployment lifecycle adapter
  is registered. `deployctl` does not silently fall back to the nginx driver.

## Verification

These commands are read-only or produce local build evidence. They do not publish or deploy:

```bash
node ../sdkwork-github-workflow/scripts/sdkwork-workflow.mjs validate --config sdkwork.workflow.json
pnpm deploy:validate:standalone
pnpm deploy:validate:cloud
node --test scripts/release/workflow-release-target.test.mjs
node --test scripts/release/workflow-supply-chain-evidence.test.mjs
node --test scripts/release/workflow-deploy.test.mjs
pnpm check:commercial-readiness
```

## Governing Standards

- `../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../../sdkwork-specs/RELEASE_SPEC.md`
- `../../sdkwork-specs/SUPPLY_CHAIN_SECURITY_SPEC.md`
- `../../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`
