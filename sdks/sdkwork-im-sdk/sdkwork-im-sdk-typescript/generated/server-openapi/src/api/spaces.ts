import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { PageInfo, SdkWorkCommandData, SpaceBanCreateRequest, SpaceBanView, SpaceChannelAccessRuleCreateRequest, SpaceChannelAccessRuleView, SpaceChannelCreateRequest, SpaceChannelUpdateRequest, SpaceChannelView, SpaceCreateRequest, SpaceGroupCreateRequest, SpaceGroupMemberCreateRequest, SpaceGroupMemberUpdateRequest, SpaceGroupMemberView, SpaceGroupUpdateRequest, SpaceGroupView, SpaceInviteCreateRequest, SpaceInviteView, SpaceMemberCreateRequest, SpaceMemberUpdateRequest, SpaceMemberView, SpaceUpdateRequest, SpaceView } from '../types';


export interface SpacesBansListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesBansApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces bans */
  async list(spaceId: string, params?: SpacesBansListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceBanView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceBanView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces bans */
  async create(spaceId: string, body: SpaceBanCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceBanView> {
    return this.client.request<SpaceBanView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces bans */
  async retrieve(spaceId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<SpaceBanView> {
    return this.client.request<SpaceBanView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces bans */
  async delete(spaceId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesInvitesListParams {
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
  pageSize?: number;
  cursor?: string;
}

export class SpacesInvitesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces invites */
  async list(spaceId: string, params?: SpacesInvitesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceInviteView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceInviteView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces invites */
  async create(spaceId: string, body: SpaceInviteCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceInviteView> {
    return this.client.request<SpaceInviteView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces invites */
  async retrieve(spaceId: string, inviteCode: string, requestOptions?: ApiRequestOptions): Promise<SpaceInviteView> {
    return this.client.request<SpaceInviteView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces invites */
  async delete(spaceId: string, inviteCode: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }

/** Accept spaces invites */
  async accept(spaceId: string, inviteCode: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}/accept`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, sdkworkUnwrapKind: 'command' });
  }
}

export interface SpacesChannelsAccessRulesListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesChannelsAccessRulesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces channels access Rules */
  async list(spaceId: string, channelId: string, params?: SpacesChannelsAccessRulesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceChannelAccessRuleView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceChannelAccessRuleView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces channels access Rules */
  async create(spaceId: string, channelId: string, body: SpaceChannelAccessRuleCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceChannelAccessRuleView> {
    return this.client.request<SpaceChannelAccessRuleView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces channels access Rules */
  async delete(spaceId: string, channelId: string, ruleId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules/${serializePathParameter(ruleId, { name: 'ruleId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesChannelsListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesChannelsApi {
  private client: HttpClient;
  public readonly accessRules: SpacesChannelsAccessRulesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.accessRules = new SpacesChannelsAccessRulesApi(client);
  }


/** List spaces channels */
  async list(spaceId: string, params?: SpacesChannelsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceChannelView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceChannelView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces channels */
  async create(spaceId: string, body: SpaceChannelCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceChannelView> {
    return this.client.request<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces channels */
  async retrieve(spaceId: string, channelId: string, requestOptions?: ApiRequestOptions): Promise<SpaceChannelView> {
    return this.client.request<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update spaces channels */
  async update(spaceId: string, channelId: string, body: SpaceChannelUpdateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceChannelView> {
    return this.client.request<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces channels */
  async delete(spaceId: string, channelId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesGroupsMembersListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesGroupsMembersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces groups members */
  async list(spaceId: string, groupId: string, params?: SpacesGroupsMembersListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceGroupMemberView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceGroupMemberView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces groups members */
  async create(spaceId: string, groupId: string, body: SpaceGroupMemberCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceGroupMemberView> {
    return this.client.request<SpaceGroupMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces groups members */
  async retrieve(spaceId: string, groupId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<SpaceGroupMemberView> {
    return this.client.request<SpaceGroupMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update spaces groups members */
  async update(spaceId: string, groupId: string, userId: string, body: SpaceGroupMemberUpdateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceGroupMemberView> {
    return this.client.request<SpaceGroupMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces groups members */
  async delete(spaceId: string, groupId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesGroupsListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesGroupsApi {
  private client: HttpClient;
  public readonly members: SpacesGroupsMembersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.members = new SpacesGroupsMembersApi(client);
  }


/** List spaces groups */
  async list(spaceId: string, params?: SpacesGroupsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceGroupView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceGroupView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces groups */
  async create(spaceId: string, body: SpaceGroupCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceGroupView> {
    return this.client.request<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces groups */
  async retrieve(spaceId: string, groupId: string, requestOptions?: ApiRequestOptions): Promise<SpaceGroupView> {
    return this.client.request<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update spaces groups */
  async update(spaceId: string, groupId: string, body: SpaceGroupUpdateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceGroupView> {
    return this.client.request<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces groups */
  async delete(spaceId: string, groupId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesMembersListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesMembersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces members */
  async list(spaceId: string, params?: SpacesMembersListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceMemberView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceMemberView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create spaces members */
  async create(spaceId: string, body: SpaceMemberCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceMemberView> {
    return this.client.request<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** retrieve spaces members */
  async retrieve(spaceId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<SpaceMemberView> {
    return this.client.request<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update spaces members */
  async update(spaceId: string, userId: string, body: SpaceMemberUpdateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceMemberView> {
    return this.client.request<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete spaces members */
  async delete(spaceId: string, userId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SpacesListParams {
  pageSize?: number;
  cursor?: string;
}

export class SpacesApi {
  private client: HttpClient;
  public readonly members: SpacesMembersApi;
  public readonly groups: SpacesGroupsApi;
  public readonly channels: SpacesChannelsApi;
  public readonly invites: SpacesInvitesApi;
  public readonly bans: SpacesBansApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.members = new SpacesMembersApi(client);
    this.groups = new SpacesGroupsApi(client);
    this.channels = new SpacesChannelsApi(client);
    this.invites = new SpacesInvitesApi(client);
    this.bans = new SpacesBansApi(client);
  }


/** Create a space */
  async create(body: SpaceCreateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceView> {
    return this.client.request<SpaceView>(imApiPath(`/spaces`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** List spaces */
  async list(params?: SpacesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SpaceView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SpaceView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/spaces`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Retrieve a space */
  async retrieve(spaceId: string, requestOptions?: ApiRequestOptions): Promise<SpaceView> {
    return this.client.request<SpaceView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update a space */
  async update(spaceId: string, body: SpaceUpdateRequest, requestOptions?: ApiRequestOptions): Promise<SpaceView> {
    return this.client.request<SpaceView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete a space */
  async delete(spaceId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export function createSpacesApi(client: HttpClient): SpacesApi {
  return new SpacesApi(client);
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
