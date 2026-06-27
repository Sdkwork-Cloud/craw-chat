from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"

def serialize_path_parameter(value: Any, spec: Dict[str, Any]) -> str:
    if value is None:
        return ''

    style = str(spec.get('style') or 'simple')
    name = str(spec.get('name') or '')
    explode = bool(spec.get('explode'))
    if isinstance(value, (list, tuple)):
        return serialize_path_array(name, value, style, explode)
    if isinstance(value, dict):
        return serialize_path_object(name, value, style, explode)
    return path_prefix(name, style) + encode_path_value(serialize_path_primitive(value))


def serialize_path_array(name: str, values: Any, style: str, explode: bool) -> str:
    serialized = [encode_path_value(serialize_path_primitive(item)) for item in values if item is not None]
    if not serialized:
        return path_prefix(name, style)
    if style == 'matrix':
        return ''.join(f";{name}={item}" for item in serialized) if explode else f";{name}={','.join(serialized)}"
    return path_prefix(name, style) + ('.' if explode else ',').join(serialized)


def serialize_path_object(name: str, value: Dict[str, Any], style: str, explode: bool) -> str:
    entries = [(key, entry_value) for key, entry_value in value.items() if entry_value is not None]
    if not entries:
        return path_prefix(name, style)
    if style == 'matrix':
        if explode:
            return ''.join(f";{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
        return f";{name}={serialized}"
    if explode:
        separator = '.' if style == 'label' else ','
        serialized = separator.join(f"{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
    else:
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
    return path_prefix(name, style) + serialized


def path_prefix(name: str, style: str) -> str:
    if style == 'label':
        return '.'
    if style == 'matrix':
        return f";{name}"
    return ''


def encode_path_value(value: str) -> str:
    from urllib.parse import quote

    return quote(value, safe='')


def serialize_path_primitive(value: Any) -> str:
    if isinstance(value, dict):
        import json

        return json.dumps(value, separators=(',', ':'))
    return str(value)





class AdminApi:
    """admin admin API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.api_key_groups = AdminApiKeyGroupsApi(client)
        self.api_keys = AdminApiKeysApi(client)
        self.billing = AdminBillingApi(client)
        self.channel_models = AdminChannelModelsApi(client)
        self.channels = AdminChannelsApi(client)
        self.credentials = AdminCredentialsApi(client)
        self.extensions = AdminExtensionsApi(client)
        self.gateway = AdminGatewayApi(client)
        self.marketing = AdminMarketingApi(client)
        self.model_prices = AdminModelPricesApi(client)
        self.models = AdminModelsApi(client)
        self.providers = AdminProvidersApi(client)
        self.routing = AdminRoutingApi(client)
        self.storage = AdminStorageApi(client)
        self.usage = AdminUsageApi(client)


class AdminApiKeyGroupsApi:
    """admin admin.api_key_groups API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listApiKeyGroups"""
        return self._client.get(f"/backend/v3/api/admin/api_key_groups")

    def create(self, body: Dict[str, Any]) -> Any:
        """createApiKeyGroup"""
        return self._client.post(f"/backend/v3/api/admin/api_key_groups", json=body)

    def update(self, group_id: str, body: Dict[str, Any]) -> Any:
        """updateApiKeyGroup"""
        return self._client.patch(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, group_id: str) -> Any:
        """deleteApiKeyGroup"""
        return self._client.delete(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

    def status(self, group_id: str, body: Dict[str, Any]) -> Any:
        """updateApiKeyGroupStatus"""
        return self._client.post(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/status", json=body)

class AdminApiKeysApi:
    """admin admin.api_keys API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listApiKeys"""
        return self._client.get(f"/backend/v3/api/admin/api_keys")

    def create(self, body: Dict[str, Any]) -> Any:
        """createApiKey"""
        return self._client.post(f"/backend/v3/api/admin/api_keys", json=body)

    def update(self, hashed_key: str, body: Dict[str, Any]) -> Any:
        """updateApiKey"""
        return self._client.put(f"/backend/v3/api/admin/api_keys/{serialize_path_parameter(hashed_key, {'name': 'hashedKey', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, hashed_key: str) -> Any:
        """deleteApiKey"""
        return self._client.delete(f"/backend/v3/api/admin/api_keys/{serialize_path_parameter(hashed_key, {'name': 'hashedKey', 'style': 'simple', 'explode': False})}")

    def status(self, hashed_key: str, body: Dict[str, Any]) -> Any:
        """updateApiKeyStatus"""
        return self._client.post(f"/backend/v3/api/admin/api_keys/{serialize_path_parameter(hashed_key, {'name': 'hashedKey', 'style': 'simple', 'explode': False})}/status", json=body)

class AdminBillingApi:
    """admin admin.billing API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.events = AdminBillingEventsApi(client)
        self.summary = AdminBillingSummaryApi(client)


class AdminBillingEventsApi:
    """admin admin.billing.events API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.summary = AdminBillingEventsSummaryApi(client)


    def list(self) -> Any:
        """listBillingEvents"""
        return self._client.get(f"/backend/v3/api/admin/billing/events")

class AdminBillingEventsSummaryApi:
    """admin admin.billing.events.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Any:
        """getBillingEventSummary"""
        return self._client.get(f"/backend/v3/api/admin/billing/events/summary")

class AdminBillingSummaryApi:
    """admin admin.billing.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Any:
        """getBillingSummary"""
        return self._client.get(f"/backend/v3/api/admin/billing/summary")

class AdminChannelModelsApi:
    """admin admin.channel_models API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.models = AdminChannelModelsModelsApi(client)


    def list(self) -> Any:
        """listChannelModels"""
        return self._client.get(f"/backend/v3/api/admin/channel_models")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveChannelModel"""
        return self._client.post(f"/backend/v3/api/admin/channel_models", json=body)

class AdminChannelModelsModelsApi:
    """admin admin.channel_models.models API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, channel_id: str, model_id: str) -> Any:
        """deleteChannelModel"""
        return self._client.delete(f"/backend/v3/api/admin/channel_models/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/models/{serialize_path_parameter(model_id, {'name': 'modelId', 'style': 'simple', 'explode': False})}")

class AdminChannelsApi:
    """admin admin.channels API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listChannels"""
        return self._client.get(f"/backend/v3/api/admin/channels")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveChannel"""
        return self._client.post(f"/backend/v3/api/admin/channels", json=body)

    def delete(self, channel_id: str) -> Any:
        """deleteChannel"""
        return self._client.delete(f"/backend/v3/api/admin/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

class AdminCredentialsApi:
    """admin admin.credentials API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.providers = AdminCredentialsProvidersApi(client)


    def list(self) -> Any:
        """listCredentials"""
        return self._client.get(f"/backend/v3/api/admin/credentials")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveCredential"""
        return self._client.post(f"/backend/v3/api/admin/credentials", json=body)

class AdminCredentialsProvidersApi:
    """admin admin.credentials.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.keys = AdminCredentialsProvidersKeysApi(client)


class AdminCredentialsProvidersKeysApi:
    """admin admin.credentials.providers.keys API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, tenant_id: str, provider_id: str, key_reference: str) -> Any:
        """deleteCredential"""
        return self._client.delete(f"/backend/v3/api/admin/credentials/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}/providers/{serialize_path_parameter(provider_id, {'name': 'providerId', 'style': 'simple', 'explode': False})}/keys/{serialize_path_parameter(key_reference, {'name': 'keyReference', 'style': 'simple', 'explode': False})}")

class AdminExtensionsApi:
    """admin admin.extensions API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.runtime_reloads = AdminExtensionsRuntimeReloadsApi(client)
        self.runtime_statuses = AdminExtensionsRuntimeStatusesApi(client)


class AdminExtensionsRuntimeReloadsApi:
    """admin admin.extensions.runtime_reloads API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: Dict[str, Any]) -> Any:
        """reloadExtensionRuntimes"""
        return self._client.post(f"/backend/v3/api/admin/extensions/runtime_reloads", json=body)

class AdminExtensionsRuntimeStatusesApi:
    """admin admin.extensions.runtime_statuses API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listRuntimeStatuses"""
        return self._client.get(f"/backend/v3/api/admin/extensions/runtime_statuses")

class AdminGatewayApi:
    """admin admin.gateway API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.rate_limit_policies = AdminGatewayRateLimitPoliciesApi(client)
        self.rate_limit_windows = AdminGatewayRateLimitWindowsApi(client)


class AdminGatewayRateLimitPoliciesApi:
    """admin admin.gateway.rate_limit_policies API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listRateLimitPolicies"""
        return self._client.get(f"/backend/v3/api/admin/gateway/rate_limit_policies")

    def create(self, body: Dict[str, Any]) -> Any:
        """createRateLimitPolicy"""
        return self._client.post(f"/backend/v3/api/admin/gateway/rate_limit_policies", json=body)

class AdminGatewayRateLimitWindowsApi:
    """admin admin.gateway.rate_limit_windows API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listRateLimitWindows"""
        return self._client.get(f"/backend/v3/api/admin/gateway/rate_limit_windows")

class AdminMarketingApi:
    """admin admin.marketing API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.campaigns = AdminMarketingCampaignsApi(client)


class AdminMarketingCampaignsApi:
    """admin admin.marketing.campaigns API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listMarketingCampaigns"""
        return self._client.get(f"/backend/v3/api/admin/marketing/campaigns")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveMarketingCampaign"""
        return self._client.post(f"/backend/v3/api/admin/marketing/campaigns", json=body)

    def status(self, marketing_campaign_id: str, body: Dict[str, Any]) -> Any:
        """updateMarketingCampaignStatus"""
        return self._client.post(f"/backend/v3/api/admin/marketing/campaigns/{serialize_path_parameter(marketing_campaign_id, {'name': 'marketingCampaignId', 'style': 'simple', 'explode': False})}/status", json=body)

class AdminModelPricesApi:
    """admin admin.model_prices API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.models = AdminModelPricesModelsApi(client)


    def list(self) -> Any:
        """listModelPrices"""
        return self._client.get(f"/backend/v3/api/admin/model_prices")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveModelPrice"""
        return self._client.post(f"/backend/v3/api/admin/model_prices", json=body)

class AdminModelPricesModelsApi:
    """admin admin.model_prices.models API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.providers = AdminModelPricesModelsProvidersApi(client)


class AdminModelPricesModelsProvidersApi:
    """admin admin.model_prices.models.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, channel_id: str, model_id: str, proxy_provider_id: str) -> Any:
        """deleteModelPrice"""
        return self._client.delete(f"/backend/v3/api/admin/model_prices/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/models/{serialize_path_parameter(model_id, {'name': 'modelId', 'style': 'simple', 'explode': False})}/providers/{serialize_path_parameter(proxy_provider_id, {'name': 'proxyProviderId', 'style': 'simple', 'explode': False})}")

class AdminModelsApi:
    """admin admin.models API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.providers = AdminModelsProvidersApi(client)


    def list(self) -> Any:
        """listModels"""
        return self._client.get(f"/backend/v3/api/admin/models")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveModel"""
        return self._client.post(f"/backend/v3/api/admin/models", json=body)

class AdminModelsProvidersApi:
    """admin admin.models.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, external_name: str, provider_id: str) -> Any:
        """deleteModel"""
        return self._client.delete(f"/backend/v3/api/admin/models/{serialize_path_parameter(external_name, {'name': 'externalName', 'style': 'simple', 'explode': False})}/providers/{serialize_path_parameter(provider_id, {'name': 'providerId', 'style': 'simple', 'explode': False})}")

class AdminProvidersApi:
    """admin admin.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listProviders"""
        return self._client.get(f"/backend/v3/api/admin/providers")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveProvider"""
        return self._client.post(f"/backend/v3/api/admin/providers", json=body)

    def delete(self, provider_id: str) -> Any:
        """deleteProvider"""
        return self._client.delete(f"/backend/v3/api/admin/providers/{serialize_path_parameter(provider_id, {'name': 'providerId', 'style': 'simple', 'explode': False})}")

class AdminRoutingApi:
    """admin admin.routing API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.decision_logs = AdminRoutingDecisionLogsApi(client)
        self.health_snapshots = AdminRoutingHealthSnapshotsApi(client)
        self.profiles = AdminRoutingProfilesApi(client)
        self.snapshots = AdminRoutingSnapshotsApi(client)


class AdminRoutingDecisionLogsApi:
    """admin admin.routing.decision_logs API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listRoutingDecisionLogs"""
        return self._client.get(f"/backend/v3/api/admin/routing/decision_logs")

class AdminRoutingHealthSnapshotsApi:
    """admin admin.routing.health_snapshots API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Any:
        """listProviderHealthSnapshots"""
        return self._client.get(f"/backend/v3/api/admin/routing/health_snapshots")

class AdminRoutingProfilesApi:
    """admin admin.routing.profiles API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listRoutingProfiles"""
        return self._client.get(f"/backend/v3/api/admin/routing/profiles")

    def create(self, body: Dict[str, Any]) -> Any:
        """createRoutingProfile"""
        return self._client.post(f"/backend/v3/api/admin/routing/profiles", json=body)

class AdminRoutingSnapshotsApi:
    """admin admin.routing.snapshots API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listCompiledRoutingSnapshots"""
        return self._client.get(f"/backend/v3/api/admin/routing/snapshots")

class AdminStorageApi:
    """admin admin.storage API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.audit = AdminStorageAuditApi(client)
        self.config = AdminStorageConfigApi(client)
        self.effective = AdminStorageEffectiveApi(client)
        self.providers = AdminStorageProvidersApi(client)
        self.validation = AdminStorageValidationApi(client)


class AdminStorageAuditApi:
    """admin admin.storage.audit API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listStorageAuditTrail"""
        return self._client.get(f"/backend/v3/api/admin/storage/audit")

class AdminStorageConfigApi:
    """admin admin.storage.config API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tenants = AdminStorageConfigTenantsApi(client)


    def retrieve(self) -> Any:
        """getGlobalStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/config")

    def create(self, body: Dict[str, Any]) -> Any:
        """saveGlobalStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/config", json=body)

class AdminStorageConfigTenantsApi:
    """admin admin.storage.config.tenants API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, tenant_id: str) -> Any:
        """getTenantStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/config/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}")

    def create(self, tenant_id: str, body: Dict[str, Any]) -> Any:
        """saveTenantStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/config/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, tenant_id: str) -> Any:
        """deleteTenantStorageConfig"""
        return self._client.delete(f"/backend/v3/api/admin/storage/config/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}")

class AdminStorageEffectiveApi:
    """admin admin.storage.effective API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tenants = AdminStorageEffectiveTenantsApi(client)


class AdminStorageEffectiveTenantsApi:
    """admin admin.storage.effective.tenants API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, tenant_id: str) -> Any:
        """getTenantEffectiveStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/effective/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}")

class AdminStorageProvidersApi:
    """admin admin.storage.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listStorageProviders"""
        return self._client.get(f"/backend/v3/api/admin/storage/providers")

class AdminStorageValidationApi:
    """admin admin.storage.validation API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tenants = AdminStorageValidationTenantsApi(client)


    def create(self, body: Dict[str, Any]) -> Any:
        """validateGlobalStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/validate", json=body)

class AdminStorageValidationTenantsApi:
    """admin admin.storage.validation.tenants API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, tenant_id: str, body: Dict[str, Any]) -> Any:
        """validateTenantStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/validate/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}", json=body)

class AdminUsageApi:
    """admin admin.usage API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.records = AdminUsageRecordsApi(client)
        self.summary = AdminUsageSummaryApi(client)


class AdminUsageRecordsApi:
    """admin admin.usage.records API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> Any:
        """listUsageRecords"""
        return self._client.get(f"/backend/v3/api/admin/usage/records")

class AdminUsageSummaryApi:
    """admin admin.usage.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Any:
        """getUsageSummary"""
        return self._client.get(f"/backend/v3/api/admin/usage/summary")
