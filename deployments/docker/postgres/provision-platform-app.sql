-- Provision the platform runtime application (app_<tenant_id>) required by
-- the credential-entry-bootstrap login flow (equivalent to
-- sdkwork-iam-web-adapter::ensure_platform_tenant_application).
INSERT INTO iam_application_template (id, owner_tenant_id, app_key, name, display_name, app_type, version, channel, status, runtime_config_json, artifacts_config_json, default_access_permissions_json, created_at, updated_at)
VALUES ('tmpl_sdkwork_platform', '0', 'sdkwork-platform', 'sdkwork-platform', 'SDKWork Platform', 'WEB', '1.0.0', 'stable', 'active', '{}'::jsonb, '{}'::jsonb, '["iam:self"]'::jsonb, now(), now())
ON CONFLICT (id) DO UPDATE SET status = 'active', updated_at = EXCLUDED.updated_at;

INSERT INTO iam_tenant_application (id, app_id, tenant_id, organization_id, template_id, template_version, instance_key, display_name, environment, status, primary_domain, domain_config_json, access_permissions_json, runtime_config_json, provisioned_at, activated_at, created_at, updated_at)
VALUES ('tapp_100001_0_sdkwork_platform', 'app_100001', '100001', '0', 'tmpl_sdkwork_platform', '1.0.0', 'default', 'SDKWork Platform', 'prod', 'enabled', 'platform.sdkwork.local', '{}'::jsonb, '["iam:self"]'::jsonb, '{}'::jsonb, now(), now(), now(), now())
ON CONFLICT (id) DO UPDATE SET app_id = EXCLUDED.app_id, status = 'enabled', primary_domain = EXCLUDED.primary_domain, access_permissions_json = EXCLUDED.access_permissions_json, activated_at = EXCLUDED.activated_at, updated_at = EXCLUDED.updated_at;
