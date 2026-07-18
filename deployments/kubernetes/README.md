# Kubernetes Deployment Artifacts

## Purpose

`cloud/` contains non-secret source templates for the 13 active IM services in `cloud.staging` and
`cloud.production`. Source templates deliberately use visible mutable tags so they cannot be confused
with release identity. They must never be applied directly to a cluster.

Deployable manifests are generated only from a build-produced image lock that contains the exact
repository and OCI `sha256` digest for every active service. The materializer rejects missing services,
unknown services, repository substitution, malformed digests, and a lock from another Git revision.

## Release Flow

1. On a Linux release runner, build each service with `pnpm release:cloud-image:build -- ...`. The
   hardened runtime base passed through `--runtime-image` must itself be pinned by digest.
2. Push the release-tagged images, resolve their registry digests, and create an image lock conforming
   to `cloud/image-lock.schema.json`. Do not create placeholder digest values.
3. From a clean worktree at the lock's `sourceRevision`, render the deployment bundle:

```bash
pnpm release:kubernetes:materialize -- \
  --image-lock /secure/release/sdkwork-im-image-lock.json \
  --output dist/kubernetes-cloud
```

4. Verify `dist/kubernetes-cloud/bundle-manifest.json`, SBOM/provenance, signatures, target-cluster
   policy, and rollback digest before applying the rendered directory.
5. Set `SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE` to the real lock when running
   `pnpm check:commercial-readiness`.

The generated bundle pins every container as `repository@sha256:<digest>` and includes SHA-256 checksums
for all rendered files. `kubectl apply` is valid only against that generated bundle.

## Availability Contract

- Every active Deployment has a zero-unavailable rolling strategy, readiness and liveness probes,
  bounded resources, and a termination grace period of at least 60 seconds.
- Pod placement is strictly spread across hostnames and best-effort spread across zones.
- PDB coverage exists for all 13 services; HPA coverage exists for all independently scalable internal
  services. The public gateway keeps an explicit replica count and is scaled through ingress capacity
  planning.
- Session gateway receives SIGTERM directly as PID 1 and performs its application-owned 45-second
  readiness/drain/fence workflow within Kubernetes' 75-second termination window. No shell-dependent
  `preStop` hook is required.
- These templates are implementation assets, not proof that a production or DR cluster exists.

## Verification

```bash
pnpm run test:commercial-deployment-contract
pnpm run test:kubernetes-release-materializer
pnpm run test:cloud-image-build-contract
pnpm run test:sdkwork-im-session-gateway-ha
```

After a reviewed deployment, verify the target cluster directly:

```bash
kubectl -n sdkwork-im rollout status deployment --timeout=10m
kubectl -n sdkwork-im get pods,pdb,hpa
curl -fsS https://<reviewed-ingress>/readyz
```

## Related Authorities

- `../../../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../../../sdkwork-specs/RELEASE_SPEC.md`
- `../../../sdkwork-specs/SUPPLY_CHAIN_SECURITY_SPEC.md`
- `cloud/image-inventory.json`
- `../docker/cloud-service-builds.json`
