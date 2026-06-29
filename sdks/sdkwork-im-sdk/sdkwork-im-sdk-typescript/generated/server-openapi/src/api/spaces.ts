import { imApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PageInfo, SpaceBanCreateRequest, SpaceBanView, SpaceChannelAccessRuleCreateRequest, SpaceChannelAccessRuleView, SpaceChannelCreateRequest, SpaceChannelUpdateRequest, SpaceChannelView, SpaceCreateRequest, SpaceGroupCreateRequest, SpaceGroupMemberCreateRequest, SpaceGroupMemberUpdateRequest, SpaceGroupMemberView, SpaceGroupUpdateRequest, SpaceGroupView, SpaceInviteCreateRequest, SpaceInviteView, SpaceMemberCreateRequest, SpaceMemberUpdateRequest, SpaceMemberView, SpaceUpdateRequest, SpaceView } from '../types';


export class SpacesBansApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces bans */
  async list(spaceId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans`));
  }

/** Create spaces bans */
  async create(spaceId: string, body: SpaceBanCreateRequest): Promise<SpaceBanView> {
    return this.client.post<SpaceBanView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans`), body, undefined, undefined, 'application/json');
  }

/** Get spaces bans */
  async get(spaceId: string, userId: string): Promise<SpaceBanView> {
    return this.client.get<SpaceBanView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }

/** Delete spaces bans */
  async delete(spaceId: string, userId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/bans/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }
}

export class SpacesInvitesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces invites */
  async list(spaceId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites`));
  }

/** Create spaces invites */
  async create(spaceId: string, body: SpaceInviteCreateRequest): Promise<SpaceInviteView> {
    return this.client.post<SpaceInviteView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites`), body, undefined, undefined, 'application/json');
  }

/** Get spaces invites */
  async get(spaceId: string, inviteCode: string): Promise<SpaceInviteView> {
    return this.client.get<SpaceInviteView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}`));
  }

/** Revoke spaces invites */
  async revoke(spaceId: string, inviteCode: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}`));
  }

/** Accept spaces invites */
  async accept(spaceId: string, inviteCode: string): Promise<void> {
    return this.client.post<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/invites/${serializePathParameter(inviteCode, { name: 'inviteCode', style: 'simple', explode: false })}/accept`));
  }
}

export class SpacesChannelsAccessRulesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces channels access Rules */
  async list(spaceId: string, channelId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules`));
  }

/** Create spaces channels access Rules */
  async create(spaceId: string, channelId: string, body: SpaceChannelAccessRuleCreateRequest): Promise<SpaceChannelAccessRuleView> {
    return this.client.post<SpaceChannelAccessRuleView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules`), body, undefined, undefined, 'application/json');
  }

/** Delete spaces channels access Rules */
  async delete(spaceId: string, channelId: string, ruleId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/access_rules/${serializePathParameter(ruleId, { name: 'ruleId', style: 'simple', explode: false })}`));
  }
}

export class SpacesChannelsApi {
  private client: HttpClient;
  public readonly accessRules: SpacesChannelsAccessRulesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.accessRules = new SpacesChannelsAccessRulesApi(client);
  }


/** List spaces channels */
  async list(spaceId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels`));
  }

/** Create spaces channels */
  async create(spaceId: string, body: SpaceChannelCreateRequest): Promise<SpaceChannelView> {
    return this.client.post<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels`), body, undefined, undefined, 'application/json');
  }

/** Get spaces channels */
  async get(spaceId: string, channelId: string): Promise<SpaceChannelView> {
    return this.client.get<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`));
  }

/** Update spaces channels */
  async update(spaceId: string, channelId: string, body: SpaceChannelUpdateRequest): Promise<SpaceChannelView> {
    return this.client.patch<SpaceChannelView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete spaces channels */
  async delete(spaceId: string, channelId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`));
  }
}

export class SpacesGroupsMembersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces groups members */
  async list(spaceId: string, groupId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members`));
  }

/** Create spaces groups members */
  async create(spaceId: string, groupId: string, body: SpaceGroupMemberCreateRequest): Promise<SpaceGroupMemberView> {
    return this.client.post<SpaceGroupMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members`), body, undefined, undefined, 'application/json');
  }

/** Get spaces groups members */
  async get(spaceId: string, groupId: string, userId: string): Promise<SpaceGroupMemberView> {
    return this.client.get<SpaceGroupMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }

/** Update spaces groups members */
  async update(spaceId: string, groupId: string, userId: string, body: SpaceGroupMemberUpdateRequest): Promise<void> {
    return this.client.patch<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete spaces groups members */
  async delete(spaceId: string, groupId: string, userId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }
}

export class SpacesGroupsApi {
  private client: HttpClient;
  public readonly members: SpacesGroupsMembersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.members = new SpacesGroupsMembersApi(client);
  }


/** List spaces groups */
  async list(spaceId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups`));
  }

/** Create spaces groups */
  async create(spaceId: string, body: SpaceGroupCreateRequest): Promise<SpaceGroupView> {
    return this.client.post<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups`), body, undefined, undefined, 'application/json');
  }

/** Get spaces groups */
  async get(spaceId: string, groupId: string): Promise<SpaceGroupView> {
    return this.client.get<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`));
  }

/** Update spaces groups */
  async update(spaceId: string, groupId: string, body: SpaceGroupUpdateRequest): Promise<SpaceGroupView> {
    return this.client.patch<SpaceGroupView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete spaces groups */
  async delete(spaceId: string, groupId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`));
  }
}

export class SpacesMembersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List spaces members */
  async list(spaceId: string): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members`));
  }

/** Create spaces members */
  async create(spaceId: string, body: SpaceMemberCreateRequest): Promise<SpaceMemberView> {
    return this.client.post<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members`), body, undefined, undefined, 'application/json');
  }

/** Get spaces members */
  async get(spaceId: string, userId: string): Promise<SpaceMemberView> {
    return this.client.get<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }

/** Update spaces members */
  async update(spaceId: string, userId: string, body: SpaceMemberUpdateRequest): Promise<SpaceMemberView> {
    return this.client.patch<SpaceMemberView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete spaces members */
  async delete(spaceId: string, userId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}/members/${serializePathParameter(userId, { name: 'userId', style: 'simple', explode: false })}`));
  }
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
  async create(body: SpaceCreateRequest): Promise<SpaceView> {
    return this.client.post<SpaceView>(imApiPath(`/spaces`), body, undefined, undefined, 'application/json');
  }

/** List spaces */
  async list(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(imApiPath(`/spaces`));
  }

/** Get a space */
  async get(spaceId: string): Promise<SpaceView> {
    return this.client.get<SpaceView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`));
  }

/** Update a space */
  async update(spaceId: string, body: SpaceUpdateRequest): Promise<SpaceView> {
    return this.client.patch<SpaceView>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete a space */
  async delete(spaceId: string): Promise<void> {
    return this.client.delete<void>(imApiPath(`/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`));
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
