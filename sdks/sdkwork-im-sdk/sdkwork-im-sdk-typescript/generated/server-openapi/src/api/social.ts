import { imApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { ContactPreferencesView, ContactRecommendationView, ContactTagsResponse, ContactTagView, CreateContactRecommendationRequest, CreateContactTagRequest, DeleteContactTagResponse, SocialFriendRequestAcceptanceResponse, SocialFriendRequestListResponse, SocialFriendRequestMutationResponse, SocialFriendshipMutationResponse, SocialUserSearchResponse, SubmitFriendRequestRequest, UpdateContactPreferencesRequest, UpdateContactTagRequest } from '../types';


export class SocialContactsPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve contact preferences */
  async retrieve(targetUserId: string): Promise<ContactPreferencesView> {
    return this.client.get<ContactPreferencesView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/preferences`));
  }

/** Update contact preferences */
  async update(targetUserId: string, body: UpdateContactPreferencesRequest): Promise<ContactPreferencesView> {
    return this.client.patch<ContactPreferencesView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/preferences`), body, undefined, undefined, 'application/json');
  }
}

export class SocialContactsRecommendationsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a contact recommendation */
  async create(targetUserId: string, body: CreateContactRecommendationRequest): Promise<ContactRecommendationView> {
    return this.client.post<ContactRecommendationView>(imApiPath(`/social/contacts/${serializePathParameter(targetUserId, { name: 'targetUserId', style: 'simple', explode: false })}/recommendations`), body, undefined, undefined, 'application/json');
  }
}

export interface SocialContactsTagsListParams {
  limit?: number;
  cursor?: string;
}

export class SocialContactsTagsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List contact tags */
  async list(params?: SocialContactsTagsListParams): Promise<ContactTagsResponse> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ContactTagsResponse>(appendQueryString(imApiPath(`/social/contacts/tags`), query));
  }

/** Create a contact tag */
  async create(body: CreateContactTagRequest): Promise<ContactTagView> {
    return this.client.post<ContactTagView>(imApiPath(`/social/contacts/tags`), body, undefined, undefined, 'application/json');
  }

/** Update a contact tag */
  async update(tagId: string, body: UpdateContactTagRequest): Promise<ContactTagView> {
    return this.client.patch<ContactTagView>(imApiPath(`/social/contacts/tags/${serializePathParameter(tagId, { name: 'tagId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete a contact tag */
  async delete(tagId: string): Promise<DeleteContactTagResponse> {
    return this.client.delete<DeleteContactTagResponse>(imApiPath(`/social/contacts/tags/${serializePathParameter(tagId, { name: 'tagId', style: 'simple', explode: false })}`));
  }
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

}

export class SocialFriendshipsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Remove a friendship */
  async remove(friendshipId: string): Promise<SocialFriendshipMutationResponse> {
    return this.client.post<SocialFriendshipMutationResponse>(imApiPath(`/social/friendships/${serializePathParameter(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}/remove`));
  }
}

export interface SocialFriendRequestsListParams {
  direction?: 'incoming' | 'outgoing';
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
  limit?: number;
  cursor?: string;
}

export class SocialFriendRequestsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List friend requests */
  async list(params?: SocialFriendRequestsListParams): Promise<SocialFriendRequestListResponse> {
    const query = buildQueryString([
      { name: 'direction', value: params?.direction, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SocialFriendRequestListResponse>(appendQueryString(imApiPath(`/social/friend_requests`), query));
  }

/** Create a friend request */
  async create(body: SubmitFriendRequestRequest): Promise<SocialFriendRequestMutationResponse> {
    return this.client.post<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests`), body, undefined, undefined, 'application/json');
  }

/** Accept a friend request */
  async accept(requestId: string): Promise<SocialFriendRequestAcceptanceResponse> {
    return this.client.post<SocialFriendRequestAcceptanceResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/accept`));
  }

/** Decline a friend request */
  async decline(requestId: string): Promise<SocialFriendRequestMutationResponse> {
    return this.client.post<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/decline`));
  }

/** Cancel a friend request */
  async cancel(requestId: string): Promise<SocialFriendRequestMutationResponse> {
    return this.client.post<SocialFriendRequestMutationResponse>(imApiPath(`/social/friend_requests/${serializePathParameter(requestId, { name: 'requestId', style: 'simple', explode: false })}/cancel`));
  }
}

export interface SocialUsersListParams {
  q?: string;
  limit?: number;
  cursor?: string;
}

export class SocialUsersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Search social users */
  async list(params?: SocialUsersListParams): Promise<SocialUserSearchResponse> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SocialUserSearchResponse>(appendQueryString(imApiPath(`/social/users`), query));
  }
}

export class SocialApi {
  private client: HttpClient;
  public readonly users: SocialUsersApi;
  public readonly friendRequests: SocialFriendRequestsApi;
  public readonly friendships: SocialFriendshipsApi;
  public readonly contacts: SocialContactsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.users = new SocialUsersApi(client);
    this.friendRequests = new SocialFriendRequestsApi(client);
    this.friendships = new SocialFriendshipsApi(client);
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
