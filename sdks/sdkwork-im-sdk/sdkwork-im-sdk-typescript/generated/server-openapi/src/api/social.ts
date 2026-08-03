import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { BlockUserRequest, ContactPreferencesView, ContactRecommendationView, ContactTagView, ContactView, CreateContactRecommendationRequest, CreateContactTagRequest, OpenApiUserBlockResponse, SdkWorkPageData, SocialFriendRequestAcceptanceResponse, SocialFriendRequestMutationResponse, SocialFriendRequestPendingCountResponse, SocialFriendshipMutationResponse, SocialUserSearchResult, SubmitFriendRequestRequest, UpdateContactPreferencesRequest, UpdateContactTagRequest } from '../types';


export class SocialContactsPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve contact preferences */
  async retrieve(targetUserId: string, requestOptions?: ApiRequestOptions): Promise<ContactPreferencesView> {
    return this.client.request<ContactPreferencesView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/preferences`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update contact preferences */
  async update(targetUserId: string, body: UpdateContactPreferencesRequest, requestOptions?: ApiRequestOptions): Promise<ContactPreferencesView> {
    return this.client.request<ContactPreferencesView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/preferences`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class SocialContactsRecommendationsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a contact recommendation */
  async create(targetUserId: string, body: CreateContactRecommendationRequest, requestOptions?: ApiRequestOptions): Promise<ContactRecommendationView> {
    return this.client.request<ContactRecommendationView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/recommendations`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface SocialContactsTagsListParams {
  pageSize?: number;
  cursor?: string;
}

export class SocialContactsTagsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List contact tags */
  async list(params?: SocialContactsTagsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(imApiPath(`/social/contacts/tags`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create a contact tag */
  async create(body: CreateContactTagRequest, requestOptions?: ApiRequestOptions): Promise<ContactTagView> {
    return this.client.request<ContactTagView>(imApiPath(`/social/contacts/tags`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Update a contact tag */
  async update(tagId: string, body: UpdateContactTagRequest, requestOptions?: ApiRequestOptions): Promise<ContactTagView> {
    return this.client.request<ContactTagView>(imApiPath(`/social/contacts/tags/${serializePathParameter(tagId, { name: 'tagId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete a contact tag */
  async delete(tagId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/social/contacts/tags/${serializePathParameter(tagId, { name: 'tagId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface SocialContactsListParams {
  pageSize?: number;
  cursor?: string;
}

export class SocialContactsApi {
  private client: HttpClient;
  public readonly tags: SocialContactsTagsApi;
  public readonly recommendations: SocialContactsRecommendationsApi;
  public readonly preferences: SocialContactsPreferencesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.tags = new SocialContactsTagsApi(client);
    this.recommendations = new SocialContactsRecommendationsApi(client);
    this.preferences = new SocialContactsPreferencesApi(client);
  }


/** List social contacts */
  async list(params?: SocialContactsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: ContactView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: ContactView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/social/contacts`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class SocialUserBlocksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Block a social user */
  async create(body: BlockUserRequest, requestOptions?: ApiRequestOptions): Promise<OpenApiUserBlockResponse> {
    return this.client.request<OpenApiUserBlockResponse>(imApiPath(`/social/user_blocks`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Release a social user block */
  async delete(blockId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/social/user_blocks/${serializePathParameter(blockId, { name: 'blockId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export class SocialFriendshipsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Remove a friendship */
  async remove(friendshipId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendshipMutationResponse> {
    return this.client.request<SocialFriendshipMutationResponse>(imApiPath(`/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}/remove`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class SocialFriendRequestsPendingCountApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve pending incoming friend request count */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestPendingCountResponse> {
    return this.client.request<SocialFriendRequestPendingCountResponse>(imApiPath(`/social/friend_requests/pending/count`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class SocialFriendRequestsPendingApi {
  private client: HttpClient;
  public readonly count: SocialFriendRequestsPendingCountApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.count = new SocialFriendRequestsPendingCountApi(client);
  }

}

export interface SocialFriendRequestsListParams {
  direction?: 'incoming' | 'outgoing';
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
  pageSize?: number;
  cursor?: string;
}

export class SocialFriendRequestsApi {
  private client: HttpClient;
  public readonly pending: SocialFriendRequestsPendingApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.pending = new SocialFriendRequestsPendingApi(client);
  }


/** List friend requests */
  async list(params?: SocialFriendRequestsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'direction', value: params?.direction, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(imApiPath(`/social/friend_requests`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create a friend request */
  async create(body: SubmitFriendRequestRequest, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestMutationResponse> {
    return this.client.request<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Accept a friend request */
  async accept(friendRequestId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestAcceptanceResponse> {
    return this.client.request<SocialFriendRequestAcceptanceResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(friendRequestId, { name: 'friendRequestId', style: 'simple', explode: false })}/accept`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Decline a friend request */
  async decline(friendRequestId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestMutationResponse> {
    return this.client.request<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(friendRequestId, { name: 'friendRequestId', style: 'simple', explode: false })}/decline`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Cancel a friend request */
  async cancel(friendRequestId: string, requestOptions?: ApiRequestOptions): Promise<SocialFriendRequestMutationResponse> {
    return this.client.request<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(friendRequestId, { name: 'friendRequestId', style: 'simple', explode: false })}/cancel`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface SocialUsersListParams {
  q?: string;
  pageSize?: number;
  cursor?: string;
}

export class SocialUsersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Search social users */
  async list(params?: SocialUsersListParams, requestOptions?: ApiRequestOptions): Promise<{ items: SocialUserSearchResult[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: SocialUserSearchResult[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/social/users`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class SocialApi {
  private client: HttpClient;
  public readonly users: SocialUsersApi;
  public readonly friendRequests: SocialFriendRequestsApi;
  public readonly friendships: SocialFriendshipsApi;
  public readonly userBlocks: SocialUserBlocksApi;
  public readonly contacts: SocialContactsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.users = new SocialUsersApi(client);
    this.friendRequests = new SocialFriendRequestsApi(client);
    this.friendships = new SocialFriendshipsApi(client);
    this.userBlocks = new SocialUserBlocksApi(client);
    this.contacts = new SocialContactsApi(client);
  }

}

export function createSocialApi(client: HttpClient): SocialApi {
  return new SocialApi(client);
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
