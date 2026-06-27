# Wave D server package release checklist

- state: `template_only_pending_execution`
- This checklist freezes the minimum go / no-go gates for the `sdkwork-im-server` package release flow.
- Step 1: run `cargo build -p web-gateway --release --bin sdkwork-im-server --offline`
- Step 2: stage platform artifacts under each platform `artifacts/` root
- Step 3: recompute `artifact-file-list.txt`
- Step 4: recompute `SHA256SUMS`
- Step 5: perform go / no-go review against the staged artifacts, `acceptance-manifest.json`, package manifests, and startup contract
- go / no-go must confirm:
  - staged package names match the frozen templates
  - checksums and artifact-file-list were regenerated after staging
  - every platform `acceptance-manifest.json` still matches the frozen package matrix and service-manager contract
  - packages still resolve back to the canonical payload layout and `server.yaml` startup contract
