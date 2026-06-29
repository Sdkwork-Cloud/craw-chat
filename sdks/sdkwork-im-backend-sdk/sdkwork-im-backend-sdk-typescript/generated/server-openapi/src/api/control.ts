import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AcceptFriendRequestRequest, ActivateFriendshipRequest, ApplySharedChannelPolicyRequest, BindDirectChatRequest, BindExternalMemberLinkRequest, BlockUserRequest, CancelFriendRequestRequest, DeclineFriendRequestRequest, EstablishExternalConnectionRequest, MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse, ProviderBindingCommitResponse, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse, RemoveFriendshipRequest, RouteMigrationResult, RouteNodeLifecycle, SdkWorkPageData, SocialDirectChatCommitResponse, SocialDirectChatSnapshotResponse, SocialExternalConnectionCommitResponse, SocialExternalConnectionSnapshotResponse, SocialExternalMemberLinkCommitResponse, SocialExternalMemberLinkSnapshotResponse, SocialFriendRequestCommitResponse, SocialFriendRequestSnapshotResponse, SocialFriendshipCommitResponse, SocialFriendshipSnapshotResponse, SocialRuntimeRepairResponse, SocialSharedChannelPolicyCommitResponse, SocialSharedChannelPolicySnapshotResponse, SocialSharedChannelSyncDeadLetterRequeueResponse, SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SocialSharedChannelSyncPendingClaimResponse, SocialSharedChannelSyncPendingReleaseResponse, SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncPendingTakeoverResponse, SocialSharedChannelSyncPendingTargetedClaimRequest, SocialSharedChannelSyncPendingTargetedReleaseRequest, SocialSharedChannelSyncPendingTargetedTakeoverRequest, SocialSharedChannelSyncRepairResponse, SocialSharedChannelSyncTargetedRepublishRequest, SocialSharedChannelSyncTargetedRepublishResponse, SocialUserBlockCommitResponse, SocialUserBlockSnapshotResponse, SubmitFriendRequestRequest, UpsertProviderBindingPolicyRequest } from '../types';


export class ControlSocialUserBlocksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Block a user in the social graph. */
  async create(body: BlockUserRequest): Promise<SocialUserBlockCommitResponse> {
    return this.client.post<SocialUserBlockCommitResponse>(backendApiPath(`/control/social/user_blocks`), body, undefined, undefined, 'application/json');
  }

/** Read a user block snapshot. */
  async retrieve(blockId: string): Promise<SocialUserBlockSnapshotResponse> {
    return this.client.get<SocialUserBlockSnapshotResponse>(backendApiPath(`/control/social/user_blocks/${serializePathParameter(blockId, { name: 'blockId', style: 'simple', explode: false })}`));
  }
}

export class ControlSocialSharedChannelPoliciesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Apply a shared-channel policy. */
  async create(body: ApplySharedChannelPolicyRequest): Promise<SocialSharedChannelPolicyCommitResponse> {
    return this.client.post<SocialSharedChannelPolicyCommitResponse>(backendApiPath(`/control/social/shared_channel_policies`), body, undefined, undefined, 'application/json');
  }

/** Read a shared-channel policy snapshot. */
  async retrieve(policyId: string): Promise<SocialSharedChannelPolicySnapshotResponse> {
    return this.client.get<SocialSharedChannelPolicySnapshotResponse>(backendApiPath(`/control/social/shared_channel_policies/${serializePathParameter(policyId, { name: 'policyId', style: 'simple', explode: false })}`));
  }
}

export class ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Take over selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest): Promise<SocialSharedChannelSyncPendingTakeoverResponse> {
    return this.client.post<SocialSharedChannelSyncPendingTakeoverResponse>(backendApiPath(`/control/social/runtime/takeover_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Requeue selected dead-letter shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest): Promise<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse> {
    return this.client.post<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Requeue all dead-letter shared-channel sync entries. */
  async create(): Promise<SocialSharedChannelSyncDeadLetterRequeueResponse> {
    return this.client.post<SocialSharedChannelSyncDeadLetterRequeueResponse>(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Republish selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncTargetedRepublishRequest): Promise<SocialSharedChannelSyncTargetedRepublishResponse> {
    return this.client.post<SocialSharedChannelSyncTargetedRepublishResponse>(backendApiPath(`/control/social/runtime/republish_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialRuntimeRepairSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Repair shared-channel sync backlog state. */
  async create(): Promise<SocialSharedChannelSyncRepairResponse> {
    return this.client.post<SocialSharedChannelSyncRepairResponse>(backendApiPath(`/control/social/runtime/repair_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeRepairDerivedSnapshotApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Repair the persisted social runtime derived snapshot. */
  async create(): Promise<SocialRuntimeRepairResponse> {
    return this.client.post<SocialRuntimeRepairResponse>(backendApiPath(`/control/social/runtime/repair_derived_snapshot`));
  }
}

export class ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Release selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedReleaseRequest): Promise<SocialSharedChannelSyncPendingReleaseResponse> {
    return this.client.post<SocialSharedChannelSyncPendingReleaseResponse>(backendApiPath(`/control/social/runtime/release_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Reclaim stale shared-channel sync pending ownership. */
  async create(): Promise<SocialSharedChannelSyncPendingStaleReclaimResponse> {
    return this.client.post<SocialSharedChannelSyncPendingStaleReclaimResponse>(backendApiPath(`/control/social/runtime/reclaim_stale_pending_shared_channel_sync`));
  }
}

export class ControlSocialRuntimePendingSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the pending shared-channel sync queue. */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/control/social/runtime/pending_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeDeliveryStateSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read merged shared-channel sync delivery state. */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/control/social/runtime/delivery_state_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeDeliveredSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the delivered shared-channel sync ledger. */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/control/social/runtime/delivered_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeDeadLetterSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the dead-letter shared-channel sync queue. */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/control/social/runtime/dead_letter_shared_channel_sync`));
  }
}

export class ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Claim selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedClaimRequest): Promise<SocialSharedChannelSyncPendingClaimResponse> {
    return this.client.post<SocialSharedChannelSyncPendingClaimResponse>(backendApiPath(`/control/social/runtime/claim_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialRuntimeApi {
  private client: HttpClient;
  public readonly claimPendingSharedChannelSyncTargeted: ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi;
  public readonly deadLetterSharedChannelSync: ControlSocialRuntimeDeadLetterSharedChannelSyncApi;
  public readonly deliveredSharedChannelSync: ControlSocialRuntimeDeliveredSharedChannelSyncApi;
  public readonly deliveryStateSharedChannelSync: ControlSocialRuntimeDeliveryStateSharedChannelSyncApi;
  public readonly pendingSharedChannelSync: ControlSocialRuntimePendingSharedChannelSyncApi;
  public readonly reclaimStalePendingSharedChannelSync: ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi;
  public readonly releasePendingSharedChannelSyncTargeted: ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi;
  public readonly repairDerivedSnapshot: ControlSocialRuntimeRepairDerivedSnapshotApi;
  public readonly repairSharedChannelSync: ControlSocialRuntimeRepairSharedChannelSyncApi;
  public readonly republishPendingSharedChannelSyncTargeted: ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi;
  public readonly requeueDeadLetterSharedChannelSync: ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi;
  public readonly requeueDeadLetterSharedChannelSyncTargeted: ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi;
  public readonly takeoverPendingSharedChannelSyncTargeted: ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.claimPendingSharedChannelSyncTargeted = new ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi(client);
    this.deadLetterSharedChannelSync = new ControlSocialRuntimeDeadLetterSharedChannelSyncApi(client);
    this.deliveredSharedChannelSync = new ControlSocialRuntimeDeliveredSharedChannelSyncApi(client);
    this.deliveryStateSharedChannelSync = new ControlSocialRuntimeDeliveryStateSharedChannelSyncApi(client);
    this.pendingSharedChannelSync = new ControlSocialRuntimePendingSharedChannelSyncApi(client);
    this.reclaimStalePendingSharedChannelSync = new ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi(client);
    this.releasePendingSharedChannelSyncTargeted = new ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi(client);
    this.repairDerivedSnapshot = new ControlSocialRuntimeRepairDerivedSnapshotApi(client);
    this.repairSharedChannelSync = new ControlSocialRuntimeRepairSharedChannelSyncApi(client);
    this.republishPendingSharedChannelSyncTargeted = new ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi(client);
    this.requeueDeadLetterSharedChannelSync = new ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi(client);
    this.requeueDeadLetterSharedChannelSyncTargeted = new ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi(client);
    this.takeoverPendingSharedChannelSyncTargeted = new ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi(client);
  }

}

export class ControlSocialFriendshipsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Activate a friendship event. */
  async create(body: ActivateFriendshipRequest): Promise<SocialFriendshipCommitResponse> {
    return this.client.post<SocialFriendshipCommitResponse>(backendApiPath(`/control/social/friendships`), body, undefined, undefined, 'application/json');
  }

/** Read a friendship snapshot. */
  async retrieve(friendshipId: string): Promise<SocialFriendshipSnapshotResponse> {
    return this.client.get<SocialFriendshipSnapshotResponse>(backendApiPath(`/control/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}`));
  }

/** Remove a friendship. */
  async remove(friendshipId: string, body: RemoveFriendshipRequest): Promise<SocialFriendshipCommitResponse> {
    return this.client.post<SocialFriendshipCommitResponse>(backendApiPath(`/control/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}/remove`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialFriendRequestsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Submit a friend request event. */
  async create(body: SubmitFriendRequestRequest): Promise<SocialFriendRequestCommitResponse> {
    return this.client.post<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests`), body, undefined, undefined, 'application/json');
  }

/** Read a friend request snapshot. */
  async retrieve(requestId: string): Promise<SocialFriendRequestSnapshotResponse> {
    return this.client.get<SocialFriendRequestSnapshotResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}`));
  }

/** Accept a friend request. */
  async accept(requestId: string, body: AcceptFriendRequestRequest): Promise<SocialFriendRequestCommitResponse> {
    return this.client.post<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/accept`), body, undefined, undefined, 'application/json');
  }

/** Decline a friend request. */
  async decline(requestId: string, body: DeclineFriendRequestRequest): Promise<SocialFriendRequestCommitResponse> {
    return this.client.post<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/decline`), body, undefined, undefined, 'application/json');
  }

/** Cancel a friend request. */
  async cancel(requestId: string, body: CancelFriendRequestRequest): Promise<SocialFriendRequestCommitResponse> {
    return this.client.post<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/cancel`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialExternalMemberLinksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Bind an external member link. */
  async create(body: BindExternalMemberLinkRequest): Promise<SocialExternalMemberLinkCommitResponse> {
    return this.client.post<SocialExternalMemberLinkCommitResponse>(backendApiPath(`/control/social/external_member_links`), body, undefined, undefined, 'application/json');
  }

/** Read an external member link snapshot. */
  async retrieve(linkId: string): Promise<SocialExternalMemberLinkSnapshotResponse> {
    return this.client.get<SocialExternalMemberLinkSnapshotResponse>(backendApiPath(`/control/social/external_member_links/${serializePathParameter(linkId, { name: 'linkId', style: 'simple', explode: false })}`));
  }
}

export class ControlSocialExternalConnectionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Establish an external collaboration connection. */
  async create(body: EstablishExternalConnectionRequest): Promise<SocialExternalConnectionCommitResponse> {
    return this.client.post<SocialExternalConnectionCommitResponse>(backendApiPath(`/control/social/external_connections`), body, undefined, undefined, 'application/json');
  }

/** Read an external connection snapshot. */
  async retrieve(connectionId: string): Promise<SocialExternalConnectionSnapshotResponse> {
    return this.client.get<SocialExternalConnectionSnapshotResponse>(backendApiPath(`/control/social/external_connections/${serializePathParameter(connectionId, { name: 'connectionId', style: 'simple', explode: false })}`));
  }
}

export class ControlSocialDirectChatsBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Bind a direct chat to a conversation. */
  async create(body: BindDirectChatRequest): Promise<SocialDirectChatCommitResponse> {
    return this.client.post<SocialDirectChatCommitResponse>(backendApiPath(`/control/social/direct_chats/bindings`), body, undefined, undefined, 'application/json');
  }
}

export class ControlSocialDirectChatsApi {
  private client: HttpClient;
  public readonly bindings: ControlSocialDirectChatsBindingsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.bindings = new ControlSocialDirectChatsBindingsApi(client);
  }


/** Read a direct chat snapshot. */
  async retrieve(directChatId: string): Promise<SocialDirectChatSnapshotResponse> {
    return this.client.get<SocialDirectChatSnapshotResponse>(backendApiPath(`/control/social/direct_chats/${serializePathParameter(directChatId, { name: 'directChatId', style: 'simple', explode: false })}`));
  }
}

export class ControlSocialApi {
  private client: HttpClient;
  public readonly directChats: ControlSocialDirectChatsApi;
  public readonly externalConnections: ControlSocialExternalConnectionsApi;
  public readonly externalMemberLinks: ControlSocialExternalMemberLinksApi;
  public readonly friendRequests: ControlSocialFriendRequestsApi;
  public readonly friendships: ControlSocialFriendshipsApi;
  public readonly runtime: ControlSocialRuntimeApi;
  public readonly sharedChannelPolicies: ControlSocialSharedChannelPoliciesApi;
  public readonly userBlocks: ControlSocialUserBlocksApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.directChats = new ControlSocialDirectChatsApi(client);
    this.externalConnections = new ControlSocialExternalConnectionsApi(client);
    this.externalMemberLinks = new ControlSocialExternalMemberLinksApi(client);
    this.friendRequests = new ControlSocialFriendRequestsApi(client);
    this.friendships = new ControlSocialFriendshipsApi(client);
    this.runtime = new ControlSocialRuntimeApi(client);
    this.sharedChannelPolicies = new ControlSocialSharedChannelPoliciesApi(client);
    this.userBlocks = new ControlSocialUserBlocksApi(client);
  }

}

export interface ControlProviderBindingsListParams {
  tenantId?: string;
}

export class ControlProviderBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read effective provider bindings. */
  async list(params?: ControlProviderBindingsListParams): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'tenantId', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/provider_bindings`), query));
  }

/** Upsert a provider binding policy. */
  async create(body: UpsertProviderBindingPolicyRequest): Promise<ProviderBindingCommitResponse> {
    return this.client.post<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_bindings`), body, undefined, undefined, 'application/json');
  }
}

export class ControlProviderRegistryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the provider registry snapshot. */
  async retrieve(): Promise<ProviderRegistrySnapshotResponse> {
    return this.client.get<ProviderRegistrySnapshotResponse>(backendApiPath(`/control/provider_registry`));
  }
}

export interface ControlProviderPoliciesDiffListParams {
  fromVersion: string;
  toVersion: string;
}

export class ControlProviderPoliciesDiffApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read provider policy diff between two versions. */
  async list(params: ControlProviderPoliciesDiffListParams): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'fromVersion', value: params.fromVersion, style: 'form', explode: true, allowReserved: false },
      { name: 'toVersion', value: params.toVersion, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/provider_policies/diff`), query));
  }
}

export class ControlProviderPoliciesApi {
  private client: HttpClient;
  public readonly diff: ControlProviderPoliciesDiffApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.diff = new ControlProviderPoliciesDiffApi(client);
  }


/** Read provider policy history. */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/control/provider_policies`));
  }

/** Preview the effective provider policy result before commit. */
  async preview(body: UpsertProviderBindingPolicyRequest): Promise<ProviderBindingCommitResponse> {
    return this.client.post<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_policies/preview`), body, undefined, undefined, 'application/json');
  }

/** Rollback provider policy history to a target version. */
  async rollback(body: ProviderPolicyRollbackRequest): Promise<ProviderBindingCommitResponse> {
    return this.client.post<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_policies/rollback`), body, undefined, undefined, 'application/json');
  }
}

export class ControlProtocolRegistryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the control-plane protocol registry snapshot. */
  async retrieve(): Promise<ProtocolRegistryResponse> {
    return this.client.get<ProtocolRegistryResponse>(backendApiPath(`/control/protocol_registry`));
  }
}

export class ControlProtocolGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the control-plane protocol governance snapshot. */
  async retrieve(): Promise<ProtocolGovernanceResponse> {
    return this.client.get<ProtocolGovernanceResponse>(backendApiPath(`/control/protocol_governance`));
  }
}

export class ControlNodesRoutesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Migrate owned routes from the source node to the target node. */
  async migrate(nodeId: string, body: MigrateRoutesRequest): Promise<RouteMigrationResult> {
    return this.client.post<RouteMigrationResult>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/routes/migrate`), body, undefined, undefined, 'application/json');
  }
}

export class ControlNodesApi {
  private client: HttpClient;
  public readonly routes: ControlNodesRoutesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.routes = new ControlNodesRoutesApi(client);
  }


/** Activate a realtime node and clear drain state. */
  async activate(nodeId: string): Promise<RouteNodeLifecycle> {
    return this.client.post<RouteNodeLifecycle>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/activate`));
  }

/** Mark a realtime node as draining. */
  async drain(nodeId: string): Promise<RouteNodeLifecycle> {
    return this.client.post<RouteNodeLifecycle>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/drain`));
  }
}

export class ControlApi {
  private client: HttpClient;
  public readonly nodes: ControlNodesApi;
  public readonly protocolGovernance: ControlProtocolGovernanceApi;
  public readonly protocolRegistry: ControlProtocolRegistryApi;
  public readonly providerPolicies: ControlProviderPoliciesApi;
  public readonly providerRegistry: ControlProviderRegistryApi;
  public readonly providerBindings: ControlProviderBindingsApi;
  public readonly social: ControlSocialApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.nodes = new ControlNodesApi(client);
    this.protocolGovernance = new ControlProtocolGovernanceApi(client);
    this.protocolRegistry = new ControlProtocolRegistryApi(client);
    this.providerPolicies = new ControlProviderPoliciesApi(client);
    this.providerRegistry = new ControlProviderRegistryApi(client);
    this.providerBindings = new ControlProviderBindingsApi(client);
    this.social = new ControlSocialApi(client);
  }

}

export function createControlApi(client: HttpClient): ControlApi {
  return new ControlApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
