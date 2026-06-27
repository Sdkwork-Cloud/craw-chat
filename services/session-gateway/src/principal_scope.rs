pub(crate) fn typed_principal_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> String {
    sdkwork_im_contract_control::realtime_principal_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
    )
}

pub(crate) fn typed_client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    sdkwork_im_contract_control::realtime_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    )
}

pub(crate) fn tenant_client_route_scope_key(tenant_id: &str, device_id: &str) -> String {
    sdkwork_im_contract_control::realtime_scope_key_parts(&[tenant_id, device_id])
}

#[cfg(test)]
#[path = "principal_scope_tests.rs"]
mod tests;
