import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { AcceptFriendRequestRequest, ActivateFriendshipRequest, ApplySharedChannelPolicyRequest, BindDirectChatRequest, BindExternalMemberLinkRequest, BlockUserRequest, CancelFriendRequestRequest, DeclineFriendRequestRequest, EstablishExternalConnectionRequest, MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse, ProviderBindingCommitResponse, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse, RemoveFriendshipRequest, RouteMigrationResult, RouteNodeLifecycle, SdkWorkPageData, SocialDirectChatCommitResponse, SocialDirectChatSnapshotResponse, SocialExternalConnectionCommitResponse, SocialExternalConnectionSnapshotResponse, SocialExternalMemberLinkCommitResponse, SocialExternalMemberLinkSnapshotResponse, SocialFriendRequestCommitResponse, SocialFriendRequestSnapshotResponse, SocialFriendshipCommitResponse, SocialFriendshipSnapshotResponse, SocialRuntimeRepairResponse, SocialSharedChannelPolicyCommitResponse, SocialSharedChannelPolicySnapshotResponse, SocialSharedChannelSyncDeadLetterRequeueResponse, SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SocialSharedChannelSyncPendingClaimResponse, SocialSharedChannelSyncPendingReleaseResponse, SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncPendingTakeoverResponse, SocialSharedChannelSyncPendingTargetedClaimRequest, SocialSharedChannelSyncPendingTargetedReleaseRequest, SocialSharedChannelSyncPendingTargetedTakeoverRequest, SocialSharedChannelSyncRepairResponse, SocialSharedChannelSyncTargetedRepublishRequest, SocialSharedChannelSyncTargetedRepublishResponse, SocialUserBlockCommitResponse, SocialUserBlockSnapshotResponse, SubmitFriendRequestRequest, UpsertProviderBindingPolicyRequest } from '../types';


export class ControlSocialUserBlocksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Block a user in the social graph. */
  async create(body: BlockUserRequest, requestOptions?: ApiRequestOptions): Promise<SocialUserBlockCommitResponse> {
    return this.client.request<SocialUserBlockCommitResponse>(backendApiPath(`/control/social/user_blocks`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read a user block snapshot. */
  async retrieve(blockId: string, requestOptions?: ApiRequestOptions): Promise<SocialUserBlockSnapshotResponse> {
    return this.client.request<SocialUserBlockSnapshotResponse>(backendApiPath(`/control/social/user_blocks/${serializePathParameter(blockId, { name: 'blockId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialSharedChannelPoliciesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Apply a shared-channel policy. */
  async create(body: ApplySharedChannelPolicyRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelPolicyCommitResponse> {
    return this.client.request<SocialSharedChannelPolicyCommitResponse>(backendApiPath(`/control/social/shared_channel_policies`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read a shared-channel policy snapshot. */
  async retrieve(policyId: string, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelPolicySnapshotResponse> {
    return this.client.request<SocialSharedChannelPolicySnapshotResponse>(backendApiPath(`/control/social/shared_channel_policies/${serializePathParameter(policyId, { name: 'policyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Take over selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncPendingTakeoverResponse> {
    return this.client.request<SocialSharedChannelSyncPendingTakeoverResponse>(backendApiPath(`/control/social/runtime/takeover_pending_shared_channel_sync_targeted`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Requeue selected dead-letter shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse> {
    return this.client.request<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Requeue all dead-letter shared-channel sync entries. */
  async create(requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncDeadLetterRequeueResponse> {
    return this.client.request<SocialSharedChannelSyncDeadLetterRequeueResponse>(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Republish selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncTargetedRepublishRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncTargetedRepublishResponse> {
    return this.client.request<SocialSharedChannelSyncTargetedRepublishResponse>(backendApiPath(`/control/social/runtime/republish_pending_shared_channel_sync_targeted`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeRepairSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Repair shared-channel sync backlog state. */
  async create(requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncRepairResponse> {
    return this.client.request<SocialSharedChannelSyncRepairResponse>(backendApiPath(`/control/social/runtime/repair_shared_channel_sync`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeRepairDerivedSnapshotApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Repair the persisted social runtime derived snapshot. */
  async create(requestOptions?: ApiRequestOptions): Promise<SocialRuntimeRepairResponse> {
    return this.client.request<SocialRuntimeRepairResponse>(backendApiPath(`/control/social/runtime/repair_derived_snapshot`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Release selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedReleaseRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncPendingReleaseResponse> {
    return this.client.request<SocialSharedChannelSyncPendingReleaseResponse>(backendApiPath(`/control/social/runtime/release_pending_shared_channel_sync_targeted`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Reclaim stale shared-channel sync pending ownership. */
  async create(requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncPendingStaleReclaimResponse> {
    return this.client.request<SocialSharedChannelSyncPendingStaleReclaimResponse>(backendApiPath(`/control/social/runtime/reclaim_stale_pending_shared_channel_sync`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface ControlSocialRuntimePendingSharedChannelSyncListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlSocialRuntimePendingSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the pending shared-channel sync queue. */
  async list(params?: ControlSocialRuntimePendingSharedChannelSyncListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/social/runtime/pending_shared_channel_sync`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface ControlSocialRuntimeDeliveryStateSharedChannelSyncListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlSocialRuntimeDeliveryStateSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read merged shared-channel sync delivery state. */
  async list(params?: ControlSocialRuntimeDeliveryStateSharedChannelSyncListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/social/runtime/delivery_state_shared_channel_sync`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface ControlSocialRuntimeDeliveredSharedChannelSyncListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlSocialRuntimeDeliveredSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the delivered shared-channel sync ledger. */
  async list(params?: ControlSocialRuntimeDeliveredSharedChannelSyncListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/social/runtime/delivered_shared_channel_sync`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface ControlSocialRuntimeDeadLetterSharedChannelSyncListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlSocialRuntimeDeadLetterSharedChannelSyncApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the dead-letter shared-channel sync queue. */
  async list(params?: ControlSocialRuntimeDeadLetterSharedChannelSyncListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/social/runtime/dead_letter_shared_channel_sync`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Claim selected pending shared-channel sync entries. */
  async create(body: SocialSharedChannelSyncPendingTargetedClaimRequest, requestOptions?: ApiRequestOptions): Promise<SocialSharedChannelSyncPendingClaimResponse> {
    return this.client.request<SocialSharedChannelSyncPendingClaimResponse>(backendApiPath(`/control/social/runtime/claim_pending_shared_channel_sync_targeted`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialRuntimeApi {
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
  async create(body: ActivateFriendshipRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendshipCommitResponse> {
    return this.client.request<SocialFriendshipCommitResponse>(backendApiPath(`/control/social/friendships`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read a friendship snapshot. */
  async retrieve(friendshipId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendshipSnapshotResponse> {
    return this.client.request<SocialFriendshipSnapshotResponse>(backendApiPath(`/control/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Remove a friendship. */
  async remove(friendshipId: string, body: RemoveFriendshipRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendshipCommitResponse> {
    return this.client.request<SocialFriendshipCommitResponse>(backendApiPath(`/control/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}/remove`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialFriendRequestsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Submit a friend request event. */
  async create(body: SubmitFriendRequestRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestCommitResponse> {
    return this.client.request<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read a friend request snapshot. */
  async retrieve(requestId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestSnapshotResponse> {
    return this.client.request<SocialFriendRequestSnapshotResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Accept a friend request. */
  async accept(requestId: string, body: AcceptFriendRequestRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestCommitResponse> {
    return this.client.request<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/accept`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Decline a friend request. */
  async decline(requestId: string, body: DeclineFriendRequestRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestCommitResponse> {
    return this.client.request<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/decline`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Cancel a friend request. */
  async cancel(requestId: string, body: CancelFriendRequestRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestCommitResponse> {
    return this.client.request<SocialFriendRequestCommitResponse>(backendApiPath(`/control/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/cancel`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialExternalMemberLinksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Bind an external member link. */
  async create(body: BindExternalMemberLinkRequest, requestOptions?: ApiRequestOptions): Promise<SocialExternalMemberLinkCommitResponse> {
    return this.client.request<SocialExternalMemberLinkCommitResponse>(backendApiPath(`/control/social/external_member_links`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read an external member link snapshot. */
  async retrieve(linkId: string, requestOptions?: ApiRequestOptions): Promise<SocialExternalMemberLinkSnapshotResponse> {
    return this.client.request<SocialExternalMemberLinkSnapshotResponse>(backendApiPath(`/control/social/external_member_links/${serializePathParameter(linkId, { name: 'linkId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialExternalConnectionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Establish an external collaboration connection. */
  async create(body: EstablishExternalConnectionRequest, requestOptions?: ApiRequestOptions): Promise<SocialExternalConnectionCommitResponse> {
    return this.client.request<SocialExternalConnectionCommitResponse>(backendApiPath(`/control/social/external_connections`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Read an external connection snapshot. */
  async retrieve(connectionId: string, requestOptions?: ApiRequestOptions): Promise<SocialExternalConnectionSnapshotResponse> {
    return this.client.request<SocialExternalConnectionSnapshotResponse>(backendApiPath(`/control/social/external_connections/${serializePathParameter(connectionId, { name: 'connectionId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialDirectChatsBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Bind a direct chat to a conversation. */
  async create(body: BindDirectChatRequest, requestOptions?: ApiRequestOptions): Promise<SocialDirectChatCommitResponse> {
    return this.client.request<SocialDirectChatCommitResponse>(backendApiPath(`/control/social/direct_chats/bindings`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
  async retrieve(directChatId: string, requestOptions?: ApiRequestOptions): Promise<SocialDirectChatSnapshotResponse> {
    return this.client.request<SocialDirectChatSnapshotResponse>(backendApiPath(`/control/social/direct_chats/${serializePathParameter(directChatId, { name: 'directChatId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlSocialApi {
  public readonly directChats: ControlSocialDirectChatsApi;
  public readonly externalConnections: ControlSocialExternalConnectionsApi;
  public readonly externalMemberLinks: ControlSocialExternalMemberLinksApi;
  public readonly friendRequests: ControlSocialFriendRequestsApi;
  public readonly friendships: ControlSocialFriendshipsApi;
  public readonly runtime: ControlSocialRuntimeApi;
  public readonly sharedChannelPolicies: ControlSocialSharedChannelPoliciesApi;
  public readonly userBlocks: ControlSocialUserBlocksApi;

  constructor(client: HttpClient) {
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
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlProviderBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read effective provider bindings. */
  async list(params?: ControlProviderBindingsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'tenantId', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/provider_bindings`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Upsert a provider binding policy. */
  async create(body: UpsertProviderBindingPolicyRequest, requestOptions?: ApiRequestOptions): Promise<ProviderBindingCommitResponse> {
    return this.client.request<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_bindings`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlProviderRegistryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the provider registry snapshot. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<ProviderRegistrySnapshotResponse> {
    return this.client.request<ProviderRegistrySnapshotResponse>(backendApiPath(`/control/provider_registry`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface ControlProviderPoliciesDiffListParams {
  fromVersion: string;
  toVersion: string;
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlProviderPoliciesDiffApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read provider policy diff between two versions. */
  async list(params: ControlProviderPoliciesDiffListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'fromVersion', value: params.fromVersion, style: 'form', explode: true, allowReserved: false },
      { name: 'toVersion', value: params.toVersion, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/provider_policies/diff`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface ControlProviderPoliciesListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class ControlProviderPoliciesApi {
  private client: HttpClient;
  public readonly diff: ControlProviderPoliciesDiffApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.diff = new ControlProviderPoliciesDiffApi(client);
  }


/** Read provider policy history. */
  async list(params?: ControlProviderPoliciesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/control/provider_policies`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Preview the effective provider policy result before commit. */
  async preview(body: UpsertProviderBindingPolicyRequest, requestOptions?: ApiRequestOptions): Promise<ProviderBindingCommitResponse> {
    return this.client.request<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_policies/preview`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Rollback provider policy history to a target version. */
  async rollback(body: ProviderPolicyRollbackRequest, requestOptions?: ApiRequestOptions): Promise<ProviderBindingCommitResponse> {
    return this.client.request<ProviderBindingCommitResponse>(backendApiPath(`/control/provider_policies/rollback`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ControlProtocolRegistryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the control-plane protocol registry snapshot. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<ProtocolRegistryResponse> {
    return this.client.request<ProtocolRegistryResponse>(backendApiPath(`/control/protocol_registry`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlProtocolGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the control-plane protocol governance snapshot. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<ProtocolGovernanceResponse> {
    return this.client.request<ProtocolGovernanceResponse>(backendApiPath(`/control/protocol_governance`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlNodesRoutesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Migrate owned routes from the source node to the target node. */
  async migrate(nodeId: string, body: MigrateRoutesRequest, requestOptions?: ApiRequestOptions): Promise<RouteMigrationResult> {
    return this.client.request<RouteMigrationResult>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/routes/migrate`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
  async activate(nodeId: string, requestOptions?: ApiRequestOptions): Promise<RouteNodeLifecycle> {
    return this.client.request<RouteNodeLifecycle>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/activate`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Mark a realtime node as draining. */
  async drain(nodeId: string, requestOptions?: ApiRequestOptions): Promise<RouteNodeLifecycle> {
    return this.client.request<RouteNodeLifecycle>(backendApiPath(`/control/nodes/${serializePathParameter(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/drain`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ControlApi {
  public readonly nodes: ControlNodesApi;
  public readonly protocolGovernance: ControlProtocolGovernanceApi;
  public readonly protocolRegistry: ControlProtocolRegistryApi;
  public readonly providerPolicies: ControlProviderPoliciesApi;
  public readonly providerRegistry: ControlProviderRegistryApi;
  public readonly providerBindings: ControlProviderBindingsApi;
  public readonly social: ControlSocialApi;

  constructor(client: HttpClient) {
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
