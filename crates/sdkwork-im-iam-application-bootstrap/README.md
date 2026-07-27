# sdkwork-im-iam-application-bootstrap

Thin Sdkwork IM adapter over the shared embedded IAM tenant application bootstrap framework.

## Responsibility

This crate resolves the IM application root and delegates manifest discovery, mapping, Postgres `search_path`, default subject seeding, and tenant-application reconcile/upsert to:

```text
sdkwork-iam/crates/sdkwork-iam-embedded-application-bootstrap
```

Standalone IM embeds IAM locally, so the gateway calls `ensure_im_tenant_application_runtime_from_env` after IAM schema bootstrap and before credential-entry routes go live.

`ensure_im_tenant_application_runtime_from_env` resolves the IM repository app root through `resolve_im_repo_root()` and calls `ensure_tenant_application_from_app_root_with_env_and_fallback`. The existing-pool entrypoint calls `ensure_tenant_applications_from_app_root_on_pool`. Both shared paths discover the repository manifest and direct manifest-bearing `apps/*` roots. The adapter must not use `ensure_tenant_application_from_app_root_with_env`, which silently skips provisioning when `SDKWORK_*_APP_ROOT` is unset.

## Runtime identities

| Surface manifest | `backend.appId` |
| --- | --- |
| `apps/sdkwork-im-pc/sdkwork.app.config.json` | `sdkwork-im-pc` |
| `apps/sdkwork-im-h5/sdkwork.app.config.json` | `sdkwork-im-h5` |
| `apps/sdkwork-im-flutter-mobile/sdkwork.app.config.json` | `sdkwork-im-flutter-mobile` |

Runtime identities come only from application manifests. The IM adapter does not infer architecture suffixes or keep a hardcoded surface list.

## Verification

- `cargo test -p sdkwork-im-iam-application-bootstrap`
- `node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs`

## Related specs

- `sdkwork-specs/IAM_APPLICATION_BOOTSTRAP_SPEC.md`
- `sdkwork-iam/crates/sdkwork-iam-embedded-application-bootstrap/README.md`
