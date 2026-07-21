from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import ApiKeyGroupsCreateResponse201, ApiKeyGroupsStatusResponse, ApiKeyGroupsUpdateResponse, ApiKeysCreateResponse201, ApiKeysStatusResponse, ApiKeysUpdateResponse, BillingEventsSummaryRetrieveResponse, BillingSummaryRetrieveResponse, ChannelModelsCreateResponse201, ChannelsCreateResponse201, CredentialsCreateResponse201, ExtensionsRuntimeReloadsCreateResponse201, GatewayRateLimitPoliciesCreateResponse201, MarketingCampaignsCreateResponse201, MarketingCampaignsStatusResponse, ModelPricesCreateResponse201, ModelsCreateResponse201, ProvidersCreateResponse201, RoutingHealthSnapshotsRetrieveResponse, RoutingProfilesCreateResponse201, SdkWorkListResponse, StorageConfigCreateResponse201, StorageConfigRetrieveResponse, StorageConfigTenantsCreateResponse201, StorageConfigTenantsRetrieveResponse, StorageEffectiveTenantsRetrieveResponse, StorageValidationCreateResponse201, StorageValidationTenantsCreateResponse201, UsageSummaryRetrieveResponse

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


def build_query_string(parameters: List[Dict[str, Any]]) -> str:
    pairs: List[str] = []
    for parameter in parameters:
        append_serialized_parameter(pairs, parameter)
    return '&'.join(pairs)


def append_serialized_parameter(pairs: List[str], parameter: Dict[str, Any]) -> None:
    value = parameter.get('value')
    if value is None:
        return

    name = str(parameter.get('name') or '')
    allow_reserved = bool(parameter.get('allow_reserved'))
    content_type = parameter.get('content_type')
    if content_type:
        import json

        pairs.append(f"{encode_query_component(name)}={encode_query_value(json.dumps(value, separators=(',', ':')), allow_reserved)}")
        return

    style = str(parameter.get('style') or 'form')
    explode = bool(parameter.get('explode'))
    if style == 'deepObject':
        append_deep_object_parameter(pairs, name, value, allow_reserved)
        return
    if isinstance(value, (list, tuple)):
        append_array_parameter(pairs, name, value, style, explode, allow_reserved)
        return
    if isinstance(value, dict):
        append_object_parameter(pairs, name, value, style, explode, allow_reserved)
        return

    pairs.append(f"{encode_query_component(name)}={encode_query_value(serialize_primitive(value), allow_reserved)}")


def append_array_parameter(
    pairs: List[str],
    name: str,
    value: Any,
    style: str,
    explode: bool,
    allow_reserved: bool,
) -> None:
    values = [serialize_primitive(item) for item in value if item is not None]
    if not values:
        return

    if style == 'form' and explode:
        for item in values:
            pairs.append(f"{encode_query_component(name)}={encode_query_value(item, allow_reserved)}")
        return

    pairs.append(f"{encode_query_component(name)}={encode_query_value(','.join(values), allow_reserved)}")


def append_object_parameter(
    pairs: List[str],
    name: str,
    value: Dict[str, Any],
    style: str,
    explode: bool,
    allow_reserved: bool,
) -> None:
    entries = [(key, entry_value) for key, entry_value in value.items() if entry_value is not None]
    if not entries:
        return

    if style == 'form' and explode:
        for key, entry_value in entries:
            pairs.append(f"{encode_query_component(str(key))}={encode_query_value(serialize_primitive(entry_value), allow_reserved)}")
        return

    serialized = ','.join(
        item
        for key, entry_value in entries
        for item in (str(key), serialize_primitive(entry_value))
    )
    pairs.append(f"{encode_query_component(name)}={encode_query_value(serialized, allow_reserved)}")


def append_deep_object_parameter(pairs: List[str], name: str, value: Any, allow_reserved: bool) -> None:
    if not isinstance(value, dict):
        pairs.append(f"{encode_query_component(name)}={encode_query_value(serialize_primitive(value), allow_reserved)}")
        return

    for key, entry_value in value.items():
        if entry_value is None:
            continue
        pairs.append(f"{encode_query_component(f'{name}[{key}]')}={encode_query_value(serialize_primitive(entry_value), allow_reserved)}")


def serialize_primitive(value: Any) -> str:
    if isinstance(value, dict):
        import json

        return json.dumps(value, separators=(',', ':'))
    return str(value)


def encode_query_component(value: str) -> str:
    from urllib.parse import quote

    return quote(value, safe='')


def encode_query_value(value: str, allow_reserved: bool) -> str:
    from urllib.parse import quote

    return quote(value, safe=':/?#[]@!$&\'()*+,;=' if allow_reserved else '')



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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listApiKeyGroups"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/api_key_groups", query))

    def create(self, body: Dict[str, Any]) -> ApiKeyGroupsCreateResponse201:
        """createApiKeyGroup"""
        return self._client.post(f"/backend/v3/api/admin/api_key_groups", json=body)

    def update(self, group_id: str, body: Dict[str, Any]) -> ApiKeyGroupsUpdateResponse:
        """updateApiKeyGroup"""
        return self._client.patch(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, group_id: str) -> None:
        """deleteApiKeyGroup"""
        return self._client.delete(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

    def status(self, group_id: str, body: Dict[str, Any]) -> ApiKeyGroupsStatusResponse:
        """updateApiKeyGroupStatus"""
        return self._client.post(f"/backend/v3/api/admin/api_key_groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/status", json=body)

class AdminApiKeysApi:
    """admin admin.api_keys API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listApiKeys"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/api_keys", query))

    def create(self, body: Dict[str, Any]) -> ApiKeysCreateResponse201:
        """createApiKey"""
        return self._client.post(f"/backend/v3/api/admin/api_keys", json=body)

    def update(self, hashed_key: str, body: Dict[str, Any]) -> ApiKeysUpdateResponse:
        """updateApiKey"""
        return self._client.put(f"/backend/v3/api/admin/api_keys/{serialize_path_parameter(hashed_key, {'name': 'hashedKey', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, hashed_key: str) -> None:
        """deleteApiKey"""
        return self._client.delete(f"/backend/v3/api/admin/api_keys/{serialize_path_parameter(hashed_key, {'name': 'hashedKey', 'style': 'simple', 'explode': False})}")

    def status(self, hashed_key: str, body: Dict[str, Any]) -> ApiKeysStatusResponse:
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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listBillingEvents"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/billing/events", query))

class AdminBillingEventsSummaryApi:
    """admin admin.billing.events.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> BillingEventsSummaryRetrieveResponse:
        """getBillingEventSummary"""
        return self._client.get(f"/backend/v3/api/admin/billing/events/summary")

class AdminBillingSummaryApi:
    """admin admin.billing.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> BillingSummaryRetrieveResponse:
        """getBillingSummary"""
        return self._client.get(f"/backend/v3/api/admin/billing/summary")

class AdminChannelModelsApi:
    """admin admin.channel_models API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.models = AdminChannelModelsModelsApi(client)


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listChannelModels"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/channel_models", query))

    def create(self, body: Dict[str, Any]) -> ChannelModelsCreateResponse201:
        """saveChannelModel"""
        return self._client.post(f"/backend/v3/api/admin/channel_models", json=body)

class AdminChannelModelsModelsApi:
    """admin admin.channel_models.models API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, channel_id: str, model_id: str) -> None:
        """deleteChannelModel"""
        return self._client.delete(f"/backend/v3/api/admin/channel_models/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/models/{serialize_path_parameter(model_id, {'name': 'modelId', 'style': 'simple', 'explode': False})}")

class AdminChannelsApi:
    """admin admin.channels API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listChannels"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/channels", query))

    def create(self, body: Dict[str, Any]) -> ChannelsCreateResponse201:
        """saveChannel"""
        return self._client.post(f"/backend/v3/api/admin/channels", json=body)

    def delete(self, channel_id: str) -> None:
        """deleteChannel"""
        return self._client.delete(f"/backend/v3/api/admin/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

class AdminCredentialsApi:
    """admin admin.credentials API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.providers = AdminCredentialsProvidersApi(client)


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listCredentials"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/credentials", query))

    def create(self, body: Dict[str, Any]) -> CredentialsCreateResponse201:
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


    def delete(self, tenant_id: str, provider_id: str, key_reference: str) -> None:
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


    def create(self, body: Dict[str, Any]) -> ExtensionsRuntimeReloadsCreateResponse201:
        """reloadExtensionRuntimes"""
        return self._client.post(f"/backend/v3/api/admin/extensions/runtime_reloads", json=body)

class AdminExtensionsRuntimeStatusesApi:
    """admin admin.extensions.runtime_statuses API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listRuntimeStatuses"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/extensions/runtime_statuses", query))

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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listRateLimitPolicies"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/gateway/rate_limit_policies", query))

    def create(self, body: Dict[str, Any]) -> GatewayRateLimitPoliciesCreateResponse201:
        """createRateLimitPolicy"""
        return self._client.post(f"/backend/v3/api/admin/gateway/rate_limit_policies", json=body)

class AdminGatewayRateLimitWindowsApi:
    """admin admin.gateway.rate_limit_windows API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listRateLimitWindows"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/gateway/rate_limit_windows", query))

class AdminMarketingApi:
    """admin admin.marketing API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.campaigns = AdminMarketingCampaignsApi(client)


class AdminMarketingCampaignsApi:
    """admin admin.marketing.campaigns API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listMarketingCampaigns"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/marketing/campaigns", query))

    def create(self, body: Dict[str, Any]) -> MarketingCampaignsCreateResponse201:
        """saveMarketingCampaign"""
        return self._client.post(f"/backend/v3/api/admin/marketing/campaigns", json=body)

    def status(self, marketing_campaign_id: str, body: Dict[str, Any]) -> MarketingCampaignsStatusResponse:
        """updateMarketingCampaignStatus"""
        return self._client.post(f"/backend/v3/api/admin/marketing/campaigns/{serialize_path_parameter(marketing_campaign_id, {'name': 'marketingCampaignId', 'style': 'simple', 'explode': False})}/status", json=body)

class AdminModelPricesApi:
    """admin admin.model_prices API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.models = AdminModelPricesModelsApi(client)


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listModelPrices"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/model_prices", query))

    def create(self, body: Dict[str, Any]) -> ModelPricesCreateResponse201:
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


    def delete(self, channel_id: str, model_id: str, proxy_provider_id: str) -> None:
        """deleteModelPrice"""
        return self._client.delete(f"/backend/v3/api/admin/model_prices/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/models/{serialize_path_parameter(model_id, {'name': 'modelId', 'style': 'simple', 'explode': False})}/providers/{serialize_path_parameter(proxy_provider_id, {'name': 'proxyProviderId', 'style': 'simple', 'explode': False})}")

class AdminModelsApi:
    """admin admin.models API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.providers = AdminModelsProvidersApi(client)


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listModels"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/models", query))

    def create(self, body: Dict[str, Any]) -> ModelsCreateResponse201:
        """saveModel"""
        return self._client.post(f"/backend/v3/api/admin/models", json=body)

class AdminModelsProvidersApi:
    """admin admin.models.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, external_name: str, provider_id: str) -> None:
        """deleteModel"""
        return self._client.delete(f"/backend/v3/api/admin/models/{serialize_path_parameter(external_name, {'name': 'externalName', 'style': 'simple', 'explode': False})}/providers/{serialize_path_parameter(provider_id, {'name': 'providerId', 'style': 'simple', 'explode': False})}")

class AdminProvidersApi:
    """admin admin.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listProviders"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/providers", query))

    def create(self, body: Dict[str, Any]) -> ProvidersCreateResponse201:
        """saveProvider"""
        return self._client.post(f"/backend/v3/api/admin/providers", json=body)

    def delete(self, provider_id: str) -> None:
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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listRoutingDecisionLogs"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/routing/decision_logs", query))

class AdminRoutingHealthSnapshotsApi:
    """admin admin.routing.health_snapshots API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> RoutingHealthSnapshotsRetrieveResponse:
        """listProviderHealthSnapshots"""
        return self._client.get(f"/backend/v3/api/admin/routing/health_snapshots")

class AdminRoutingProfilesApi:
    """admin admin.routing.profiles API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listRoutingProfiles"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/routing/profiles", query))

    def create(self, body: Dict[str, Any]) -> RoutingProfilesCreateResponse201:
        """createRoutingProfile"""
        return self._client.post(f"/backend/v3/api/admin/routing/profiles", json=body)

class AdminRoutingSnapshotsApi:
    """admin admin.routing.snapshots API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listCompiledRoutingSnapshots"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/routing/snapshots", query))

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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listStorageAuditTrail"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/storage/audit", query))

class AdminStorageConfigApi:
    """admin admin.storage.config API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tenants = AdminStorageConfigTenantsApi(client)


    def retrieve(self) -> StorageConfigRetrieveResponse:
        """getGlobalStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/config")

    def create(self, body: Dict[str, Any]) -> StorageConfigCreateResponse201:
        """saveGlobalStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/config", json=body)

class AdminStorageConfigTenantsApi:
    """admin admin.storage.config.tenants API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, tenant_id: str) -> StorageConfigTenantsRetrieveResponse:
        """getTenantStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/config/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}")

    def create(self, tenant_id: str, body: Dict[str, Any]) -> StorageConfigTenantsCreateResponse201:
        """saveTenantStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/config/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, tenant_id: str) -> None:
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


    def retrieve(self, tenant_id: str) -> StorageEffectiveTenantsRetrieveResponse:
        """getTenantEffectiveStorageConfig"""
        return self._client.get(f"/backend/v3/api/admin/storage/effective/tenants/{serialize_path_parameter(tenant_id, {'name': 'tenantId', 'style': 'simple', 'explode': False})}")

class AdminStorageProvidersApi:
    """admin admin.storage.providers API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listStorageProviders"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/storage/providers", query))

class AdminStorageValidationApi:
    """admin admin.storage.validation API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tenants = AdminStorageValidationTenantsApi(client)


    def create(self, body: Dict[str, Any]) -> StorageValidationCreateResponse201:
        """validateGlobalStorageConfig"""
        return self._client.post(f"/backend/v3/api/admin/storage/validate", json=body)

class AdminStorageValidationTenantsApi:
    """admin admin.storage.validation.tenants API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, tenant_id: str, body: Dict[str, Any]) -> StorageValidationTenantsCreateResponse201:
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


    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None, page: Optional[int] = None, q: Optional[str] = None) -> SdkWorkListResponse:
        """listUsageRecords"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page', 'value': page, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/admin/usage/records", query))

class AdminUsageSummaryApi:
    """admin admin.usage.summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> UsageSummaryRetrieveResponse:
        """getUsageSummary"""
        return self._client.get(f"/backend/v3/api/admin/usage/summary")
