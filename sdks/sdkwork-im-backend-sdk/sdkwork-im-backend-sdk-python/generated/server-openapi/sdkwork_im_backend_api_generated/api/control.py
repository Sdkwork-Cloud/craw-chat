from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AcceptFriendRequestRequest, ActivateFriendshipRequest, ApplySharedChannelPolicyRequest, BindDirectChatRequest, BindExternalMemberLinkRequest, BlockUserRequest, CancelFriendRequestRequest, DeclineFriendRequestRequest, EstablishExternalConnectionRequest, MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse, ProviderBindingCommitResponse, ProviderBindingsResponse, ProviderPolicyDiffResponse, ProviderPolicyHistoryResponse, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse, RemoveFriendshipRequest, RouteMigrationResult, RouteNodeLifecycle, SocialDirectChatCommitResponse, SocialDirectChatSnapshotResponse, SocialExternalConnectionCommitResponse, SocialExternalConnectionSnapshotResponse, SocialExternalMemberLinkCommitResponse, SocialExternalMemberLinkSnapshotResponse, SocialFriendRequestCommitResponse, SocialFriendRequestSnapshotResponse, SocialFriendshipCommitResponse, SocialFriendshipSnapshotResponse, SocialRuntimeRepairResponse, SocialSharedChannelPolicyCommitResponse, SocialSharedChannelPolicySnapshotResponse, SocialSharedChannelSyncDeadLetterInventoryResponse, SocialSharedChannelSyncDeadLetterRequeueResponse, SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SocialSharedChannelSyncDeliveredInventoryResponse, SocialSharedChannelSyncDeliveryStateInventoryResponse, SocialSharedChannelSyncPendingClaimResponse, SocialSharedChannelSyncPendingInventoryResponse, SocialSharedChannelSyncPendingReleaseResponse, SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncPendingTakeoverResponse, SocialSharedChannelSyncPendingTargetedClaimRequest, SocialSharedChannelSyncPendingTargetedReleaseRequest, SocialSharedChannelSyncPendingTargetedTakeoverRequest, SocialSharedChannelSyncRepairResponse, SocialSharedChannelSyncTargetedRepublishRequest, SocialSharedChannelSyncTargetedRepublishResponse, SocialUserBlockCommitResponse, SocialUserBlockSnapshotResponse, SubmitFriendRequestRequest, UpsertProviderBindingPolicyRequest

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



class ControlApi:
    """control control API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.nodes = ControlNodesApi(client)
        self.protocol_governance = ControlProtocolGovernanceApi(client)
        self.protocol_registry = ControlProtocolRegistryApi(client)
        self.provider_policies = ControlProviderPoliciesApi(client)
        self.provider_registry = ControlProviderRegistryApi(client)
        self.provider_bindings = ControlProviderBindingsApi(client)
        self.social = ControlSocialApi(client)


class ControlNodesApi:
    """control control.nodes API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.routes = ControlNodesRoutesApi(client)


    def activate(self, node_id: str) -> RouteNodeLifecycle:
        """Activate a realtime node and clear drain state."""
        return self._client.post(f"/backend/v3/api/control/nodes/{serialize_path_parameter(node_id, {'name': 'nodeId', 'style': 'simple', 'explode': False})}/activate")

    def drain(self, node_id: str) -> RouteNodeLifecycle:
        """Mark a realtime node as draining."""
        return self._client.post(f"/backend/v3/api/control/nodes/{serialize_path_parameter(node_id, {'name': 'nodeId', 'style': 'simple', 'explode': False})}/drain")

class ControlNodesRoutesApi:
    """control control.nodes.routes API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def migrate(self, node_id: str, body: MigrateRoutesRequest) -> RouteMigrationResult:
        """Migrate owned routes from the source node to the target node."""
        return self._client.post(f"/backend/v3/api/control/nodes/{serialize_path_parameter(node_id, {'name': 'nodeId', 'style': 'simple', 'explode': False})}/routes/migrate", json=body)

class ControlProtocolGovernanceApi:
    """control control.protocol_governance API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> ProtocolGovernanceResponse:
        """Read the control-plane protocol governance snapshot."""
        return self._client.get(f"/backend/v3/api/control/protocol_governance")

class ControlProtocolRegistryApi:
    """control control.protocol_registry API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> ProtocolRegistryResponse:
        """Read the control-plane protocol registry snapshot."""
        return self._client.get(f"/backend/v3/api/control/protocol_registry")

class ControlProviderPoliciesApi:
    """control control.provider_policies API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.diff = ControlProviderPoliciesDiffApi(client)


    def list(self) -> ProviderPolicyHistoryResponse:
        """Read provider policy history."""
        return self._client.get(f"/backend/v3/api/control/provider_policies")

    def preview(self, body: UpsertProviderBindingPolicyRequest) -> ProviderBindingCommitResponse:
        """Preview the effective provider policy result before commit."""
        return self._client.post(f"/backend/v3/api/control/provider_policies/preview", json=body)

    def rollback(self, body: ProviderPolicyRollbackRequest) -> ProviderBindingCommitResponse:
        """Rollback provider policy history to a target version."""
        return self._client.post(f"/backend/v3/api/control/provider_policies/rollback", json=body)

class ControlProviderPoliciesDiffApi:
    """control control.provider_policies.diff API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, from_version: int, to_version: int) -> ProviderPolicyDiffResponse:
        """Read provider policy diff between two versions."""
        query = build_query_string([
            {'name': 'fromVersion', 'value': from_version, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'toVersion', 'value': to_version, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/control/provider_policies/diff", query))

class ControlProviderRegistryApi:
    """control control.provider_registry API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> ProviderRegistrySnapshotResponse:
        """Read the provider registry snapshot."""
        return self._client.get(f"/backend/v3/api/control/provider_registry")

class ControlProviderBindingsApi:
    """control control.provider_bindings API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, tenant_id: Optional[str] = None) -> ProviderBindingsResponse:
        """Read effective provider bindings."""
        query = build_query_string([
            {'name': 'tenantId', 'value': tenant_id, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/backend/v3/api/control/provider_bindings", query))

    def create(self, body: UpsertProviderBindingPolicyRequest) -> ProviderBindingCommitResponse:
        """Upsert a provider binding policy."""
        return self._client.post(f"/backend/v3/api/control/provider_bindings", json=body)

class ControlSocialApi:
    """control control.social API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.direct_chats = ControlSocialDirectChatsApi(client)
        self.external_connections = ControlSocialExternalConnectionsApi(client)
        self.external_member_links = ControlSocialExternalMemberLinksApi(client)
        self.friend_requests = ControlSocialFriendRequestsApi(client)
        self.friendships = ControlSocialFriendshipsApi(client)
        self.runtime = ControlSocialRuntimeApi(client)
        self.shared_channel_policies = ControlSocialSharedChannelPoliciesApi(client)
        self.user_blocks = ControlSocialUserBlocksApi(client)


class ControlSocialDirectChatsApi:
    """control control.social.direct_chats API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.bindings = ControlSocialDirectChatsBindingsApi(client)


    def retrieve(self, direct_chat_id: str) -> SocialDirectChatSnapshotResponse:
        """Read a direct chat snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/direct_chats/{serialize_path_parameter(direct_chat_id, {'name': 'directChatId', 'style': 'simple', 'explode': False})}")

class ControlSocialDirectChatsBindingsApi:
    """control control.social.direct_chats.bindings API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: BindDirectChatRequest) -> SocialDirectChatCommitResponse:
        """Bind a direct chat to a conversation."""
        return self._client.post(f"/backend/v3/api/control/social/direct_chats/bindings", json=body)

class ControlSocialExternalConnectionsApi:
    """control control.social.external_connections API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: EstablishExternalConnectionRequest) -> SocialExternalConnectionCommitResponse:
        """Establish an external collaboration connection."""
        return self._client.post(f"/backend/v3/api/control/social/external_connections", json=body)

    def retrieve(self, connection_id: str) -> SocialExternalConnectionSnapshotResponse:
        """Read an external connection snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/external_connections/{serialize_path_parameter(connection_id, {'name': 'connectionId', 'style': 'simple', 'explode': False})}")

class ControlSocialExternalMemberLinksApi:
    """control control.social.external_member_links API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: BindExternalMemberLinkRequest) -> SocialExternalMemberLinkCommitResponse:
        """Bind an external member link."""
        return self._client.post(f"/backend/v3/api/control/social/external_member_links", json=body)

    def retrieve(self, link_id: str) -> SocialExternalMemberLinkSnapshotResponse:
        """Read an external member link snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/external_member_links/{serialize_path_parameter(link_id, {'name': 'linkId', 'style': 'simple', 'explode': False})}")

class ControlSocialFriendRequestsApi:
    """control control.social.friend_requests API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SubmitFriendRequestRequest) -> SocialFriendRequestCommitResponse:
        """Submit a friend request event."""
        return self._client.post(f"/backend/v3/api/control/social/friend_requests", json=body)

    def retrieve(self, request_id: str) -> SocialFriendRequestSnapshotResponse:
        """Read a friend request snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}")

    def accept(self, request_id: str, body: AcceptFriendRequestRequest) -> SocialFriendRequestCommitResponse:
        """Accept a friend request."""
        return self._client.post(f"/backend/v3/api/control/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/accept", json=body)

    def decline(self, request_id: str, body: DeclineFriendRequestRequest) -> SocialFriendRequestCommitResponse:
        """Decline a friend request."""
        return self._client.post(f"/backend/v3/api/control/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/decline", json=body)

    def cancel(self, request_id: str, body: CancelFriendRequestRequest) -> SocialFriendRequestCommitResponse:
        """Cancel a friend request."""
        return self._client.post(f"/backend/v3/api/control/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/cancel", json=body)

class ControlSocialFriendshipsApi:
    """control control.social.friendships API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: ActivateFriendshipRequest) -> SocialFriendshipCommitResponse:
        """Activate a friendship event."""
        return self._client.post(f"/backend/v3/api/control/social/friendships", json=body)

    def retrieve(self, friendship_id: str) -> SocialFriendshipSnapshotResponse:
        """Read a friendship snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/friendships/{serialize_path_parameter(friendship_id, {'name': 'friendshipId', 'style': 'simple', 'explode': False})}")

    def remove(self, friendship_id: str, body: RemoveFriendshipRequest) -> SocialFriendshipCommitResponse:
        """Remove a friendship."""
        return self._client.post(f"/backend/v3/api/control/social/friendships/{serialize_path_parameter(friendship_id, {'name': 'friendshipId', 'style': 'simple', 'explode': False})}/remove", json=body)

class ControlSocialRuntimeApi:
    """control control.social.runtime API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.claim_pending_shared_channel_sync_targeted = ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi(client)
        self.dead_letter_shared_channel_sync = ControlSocialRuntimeDeadLetterSharedChannelSyncApi(client)
        self.delivered_shared_channel_sync = ControlSocialRuntimeDeliveredSharedChannelSyncApi(client)
        self.delivery_state_shared_channel_sync = ControlSocialRuntimeDeliveryStateSharedChannelSyncApi(client)
        self.pending_shared_channel_sync = ControlSocialRuntimePendingSharedChannelSyncApi(client)
        self.reclaim_stale_pending_shared_channel_sync = ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi(client)
        self.release_pending_shared_channel_sync_targeted = ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi(client)
        self.repair_derived_snapshot = ControlSocialRuntimeRepairDerivedSnapshotApi(client)
        self.repair_shared_channel_sync = ControlSocialRuntimeRepairSharedChannelSyncApi(client)
        self.republish_pending_shared_channel_sync_targeted = ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi(client)
        self.requeue_dead_letter_shared_channel_sync = ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi(client)
        self.requeue_dead_letter_shared_channel_sync_targeted = ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi(client)
        self.takeover_pending_shared_channel_sync_targeted = ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi(client)


class ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi:
    """control control.social.runtime.claim_pending_shared_channel_sync_targeted API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SocialSharedChannelSyncPendingTargetedClaimRequest) -> SocialSharedChannelSyncPendingClaimResponse:
        """Claim selected pending shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted", json=body)

class ControlSocialRuntimeDeadLetterSharedChannelSyncApi:
    """control control.social.runtime.dead_letter_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> SocialSharedChannelSyncDeadLetterInventoryResponse:
        """Read the dead-letter shared-channel sync queue."""
        return self._client.get(f"/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync")

class ControlSocialRuntimeDeliveredSharedChannelSyncApi:
    """control control.social.runtime.delivered_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> SocialSharedChannelSyncDeliveredInventoryResponse:
        """Read the delivered shared-channel sync ledger."""
        return self._client.get(f"/backend/v3/api/control/social/runtime/delivered_shared_channel_sync")

class ControlSocialRuntimeDeliveryStateSharedChannelSyncApi:
    """control control.social.runtime.delivery_state_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> SocialSharedChannelSyncDeliveryStateInventoryResponse:
        """Read merged shared-channel sync delivery state."""
        return self._client.get(f"/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync")

class ControlSocialRuntimePendingSharedChannelSyncApi:
    """control control.social.runtime.pending_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self) -> SocialSharedChannelSyncPendingInventoryResponse:
        """Read the pending shared-channel sync queue."""
        return self._client.get(f"/backend/v3/api/control/social/runtime/pending_shared_channel_sync")

class ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi:
    """control control.social.runtime.reclaim_stale_pending_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> SocialSharedChannelSyncPendingStaleReclaimResponse:
        """Reclaim stale shared-channel sync pending ownership."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync")

class ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi:
    """control control.social.runtime.release_pending_shared_channel_sync_targeted API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SocialSharedChannelSyncPendingTargetedReleaseRequest) -> SocialSharedChannelSyncPendingReleaseResponse:
        """Release selected pending shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted", json=body)

class ControlSocialRuntimeRepairDerivedSnapshotApi:
    """control control.social.runtime.repair_derived_snapshot API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> SocialRuntimeRepairResponse:
        """Repair the persisted social runtime derived snapshot."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/repair_derived_snapshot")

class ControlSocialRuntimeRepairSharedChannelSyncApi:
    """control control.social.runtime.repair_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> SocialSharedChannelSyncRepairResponse:
        """Repair shared-channel sync backlog state."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/repair_shared_channel_sync")

class ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi:
    """control control.social.runtime.republish_pending_shared_channel_sync_targeted API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SocialSharedChannelSyncTargetedRepublishRequest) -> SocialSharedChannelSyncTargetedRepublishResponse:
        """Republish selected pending shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted", json=body)

class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi:
    """control control.social.runtime.requeue_dead_letter_shared_channel_sync API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> SocialSharedChannelSyncDeadLetterRequeueResponse:
        """Requeue all dead-letter shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync")

class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi:
    """control control.social.runtime.requeue_dead_letter_shared_channel_sync_targeted API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) -> SocialSharedChannelSyncDeadLetterTargetedRequeueResponse:
        """Requeue selected dead-letter shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted", json=body)

class ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi:
    """control control.social.runtime.takeover_pending_shared_channel_sync_targeted API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: SocialSharedChannelSyncPendingTargetedTakeoverRequest) -> SocialSharedChannelSyncPendingTakeoverResponse:
        """Take over selected pending shared-channel sync entries."""
        return self._client.post(f"/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted", json=body)

class ControlSocialSharedChannelPoliciesApi:
    """control control.social.shared_channel_policies API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: ApplySharedChannelPolicyRequest) -> SocialSharedChannelPolicyCommitResponse:
        """Apply a shared-channel policy."""
        return self._client.post(f"/backend/v3/api/control/social/shared_channel_policies", json=body)

    def retrieve(self, policy_id: str) -> SocialSharedChannelPolicySnapshotResponse:
        """Read a shared-channel policy snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/shared_channel_policies/{serialize_path_parameter(policy_id, {'name': 'policyId', 'style': 'simple', 'explode': False})}")

class ControlSocialUserBlocksApi:
    """control control.social.user_blocks API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: BlockUserRequest) -> SocialUserBlockCommitResponse:
        """Block a user in the social graph."""
        return self._client.post(f"/backend/v3/api/control/social/user_blocks", json=body)

    def retrieve(self, block_id: str) -> SocialUserBlockSnapshotResponse:
        """Read a user block snapshot."""
        return self._client.get(f"/backend/v3/api/control/social/user_blocks/{serialize_path_parameter(block_id, {'name': 'blockId', 'style': 'simple', 'explode': False})}")
