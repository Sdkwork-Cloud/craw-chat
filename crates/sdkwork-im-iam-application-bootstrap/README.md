# sdkwork-im-iam-application-bootstrap

Thin Sdkwork IM adapter over the shared embedded IAM tenant application bootstrap framework.

## Responsibility

This crate supplies IM-specific runtime bindings (PC, H5, and Flutter mobile) and delegates manifest mapping, Postgres `search_path`, default subject seeding, and tenant-application reconcile/upsert to:

```text
sdkwork-iam/crates/sdkwork-iam-embedded-application-bootstrap
```

Standalone IM embeds IAM locally, so the gateway calls `ensure_im_tenant_application_runtime_from_env` after IAM schema bootstrap and before credential-entry routes go live.

`ensure_im_tenant_application_runtime_from_env` resolves the IM repository app root through `resolve_im_repo_root()` and calls `ensure_tenant_application_from_app_root_with_env_and_fallback`. It must not use `ensure_tenant_application_from_app_root_with_env`, which silently skips provisioning when `SDKWORK_*_APP_ROOT` is unset.

## Runtime bindings

| Surface | `runtime_app_id` | Example `instance_key` (dev) |
| --- | --- | --- |
| PC | `sdkwork-im-pc` | `sdkwork_im_pc_dev` |
| H5 | `sdkwork-im-h5` | `sdkwork_im_h5_dev` |
| Flutter mobile | `sdkwork-im-flutter-mobile` | `sdkwork_im_flutter_mobile_dev` |

Instance keys are derived by the shared framework through `tenant_application_instance_key` so IM tenant applications do not collide with platform defaults such as `default`.

## Verification

- `cargo test -p sdkwork-im-iam-application-bootstrap`
- `node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs`

## Related specs

- `sdkwork-specs/IAM_APPLICATION_BOOTSTRAP_SPEC.md`
- `sdkwork-iam/crates/sdkwork-iam-embedded-application-bootstrap/README.md`
