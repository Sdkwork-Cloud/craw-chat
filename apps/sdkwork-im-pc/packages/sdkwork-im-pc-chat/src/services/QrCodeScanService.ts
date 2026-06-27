export const SDKWORK_QR_CODE_STANDARD_VERSION = 1;

export type QrCodeScenarioKind = 'user' | 'group' | 'community' | 'url' | 'unknown';

interface BaseQrCodePayload {
  rawContent: string;
  source: 'json' | 'link' | 'text';
}

export interface UserQrCodePayload extends BaseQrCodePayload {
  kind: 'user';
  chatId?: string;
  userId: string;
}

export interface GroupQrCodePayload extends BaseQrCodePayload {
  kind: 'group';
  conversationId?: string;
  groupId: string;
  inviteCode?: string;
}

export interface CommunityQrCodePayload extends BaseQrCodePayload {
  kind: 'community';
  communityId: string;
  inviteCode?: string;
}

export interface UrlQrCodePayload extends BaseQrCodePayload {
  kind: 'url';
  url: string;
}

export interface UnknownQrCodePayload extends BaseQrCodePayload {
  kind: 'unknown';
}

export type QrCodeScanPayload =
  | UserQrCodePayload
  | GroupQrCodePayload
  | CommunityQrCodePayload
  | UrlQrCodePayload
  | UnknownQrCodePayload;

export type QrCodeScanActionKind =
  | 'viewUserProfile'
  | 'sendFriendRequest'
  | 'openGroup'
  | 'joinGroup'
  | 'openCommunity'
  | 'joinCommunity'
  | 'openEmbeddedBrowser'
  | 'showUnknownContentModal'
  | 'copyRawContent';

export interface QrCodeScenarioDefinition {
  kind: Exclude<QrCodeScenarioKind, 'url' | 'unknown'>;
  jsonTypeAliases: readonly string[];
  pathAliases: readonly string[];
  primaryIdKeys: readonly string[];
}

export interface QrCodeScanAction {
  kind: QrCodeScanActionKind;
  labelKey: string;
  role: 'primary' | 'secondary';
  requiresResolvedEntity?: boolean;
  unavailableWithoutSdkContract?: boolean;
}

export interface QrCodeScenarioActionDefinition {
  actions: readonly QrCodeScanAction[];
  kind: QrCodeScenarioKind;
  resultLabelKey: string;
}

export const QR_CODE_SCENARIO_DEFINITIONS: readonly QrCodeScenarioDefinition[] = [
  {
    kind: 'user',
    jsonTypeAliases: ['user', 'contact', 'friend', 'sdkwork.chat.user', 'im.user'],
    pathAliases: ['user', 'users', 'contact', 'contacts', 'friend', 'friends'],
    primaryIdKeys: ['userId', 'user_id', 'uid', 'id', 'chatId', 'chat_id'],
  },
  {
    kind: 'group',
    jsonTypeAliases: ['group', 'conversation', 'sdkwork.chat.group', 'im.group'],
    pathAliases: ['group', 'groups', 'conversation', 'conversations'],
    primaryIdKeys: ['groupId', 'group_id', 'conversationId', 'conversation_id', 'id'],
  },
  {
    kind: 'community',
    jsonTypeAliases: ['community', 'circle', 'sdkwork.chat.community', 'sdkwork.chat.circle', 'im.community'],
    pathAliases: ['community', 'communities', 'circle', 'circles'],
    primaryIdKeys: ['communityId', 'community_id', 'circleId', 'circle_id', 'id'],
  },
] as const;

export const QR_CODE_SCENARIO_ACTION_DEFINITIONS: readonly QrCodeScenarioActionDefinition[] = [
  {
    kind: 'user',
    resultLabelKey: 'scanQr.result.user',
    actions: [
      {
        kind: 'viewUserProfile',
        labelKey: 'scanQr.actions.viewUserProfile',
        role: 'secondary',
      },
      {
        kind: 'sendFriendRequest',
        labelKey: 'scanQr.actions.addFriend',
        role: 'primary',
      },
    ],
  },
  {
    kind: 'group',
    resultLabelKey: 'scanQr.result.group',
    actions: [
      {
        kind: 'openGroup',
        labelKey: 'scanQr.actions.openGroup',
        role: 'primary',
        requiresResolvedEntity: true,
      },
      {
        kind: 'joinGroup',
        labelKey: 'scanQr.actions.joinGroup',
        role: 'primary',
        unavailableWithoutSdkContract: true,
      },
    ],
  },
  {
    kind: 'community',
    resultLabelKey: 'scanQr.result.community',
    actions: [
      {
        kind: 'openCommunity',
        labelKey: 'scanQr.actions.openCommunity',
        role: 'primary',
        requiresResolvedEntity: true,
      },
      {
        kind: 'joinCommunity',
        labelKey: 'scanQr.actions.joinCommunity',
        role: 'primary',
        unavailableWithoutSdkContract: true,
      },
    ],
  },
  {
    kind: 'url',
    resultLabelKey: 'scanQr.result.url',
    actions: [
      {
        kind: 'openEmbeddedBrowser',
        labelKey: 'scanQr.actions.openLink',
        role: 'primary',
      },
      {
        kind: 'copyRawContent',
        labelKey: 'scanQr.actions.copyContent',
        role: 'secondary',
      },
    ],
  },
  {
    kind: 'unknown',
    resultLabelKey: 'scanQr.result.unknown',
    actions: [
      {
        kind: 'showUnknownContentModal',
        labelKey: 'scanQr.actions.viewContent',
        role: 'primary',
      },
      {
        kind: 'copyRawContent',
        labelKey: 'scanQr.actions.copyContent',
        role: 'secondary',
      },
    ],
  },
] as const;

type JsonRecord = Record<string, unknown>;

function findActionDefinition(kind: QrCodeScenarioKind): QrCodeScenarioActionDefinition {
  return QR_CODE_SCENARIO_ACTION_DEFINITIONS.find((definition) => definition.kind === kind)
    ?? QR_CODE_SCENARIO_ACTION_DEFINITIONS[QR_CODE_SCENARIO_ACTION_DEFINITIONS.length - 1];
}

export function getQrCodeResultLabelKey(payload: Pick<QrCodeScanPayload, 'kind'>): string {
  return findActionDefinition(payload.kind).resultLabelKey;
}

export function getQrCodeScanActions(payload: Pick<QrCodeScanPayload, 'kind'>): readonly QrCodeScanAction[] {
  return findActionDefinition(payload.kind).actions;
}

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    const normalized = normalizeString(value);
    if (normalized) {
      return normalized;
    }
  }
  return undefined;
}

function pickRecord(value: unknown): JsonRecord | undefined {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as JsonRecord
    : undefined;
}

function parseJsonRecord(rawContent: string): JsonRecord | undefined {
  try {
    return pickRecord(JSON.parse(rawContent));
  } catch {
    return undefined;
  }
}

function normalizeAlias(value: string | undefined): string | undefined {
  return value?.trim().toLowerCase().replace(/[_\s-]+/gu, '.');
}

function findScenarioDefinitionByType(typeValue: string | undefined): QrCodeScenarioDefinition | undefined {
  const normalizedType = normalizeAlias(typeValue);
  if (!normalizedType) {
    return undefined;
  }
  return QR_CODE_SCENARIO_DEFINITIONS.find((definition) => (
    definition.jsonTypeAliases.some((alias) => normalizeAlias(alias) === normalizedType)
  ));
}

function findScenarioDefinitionByPathSegment(segment: string | undefined): QrCodeScenarioDefinition | undefined {
  const normalizedSegment = segment?.trim().toLowerCase();
  if (!normalizedSegment) {
    return undefined;
  }
  return QR_CODE_SCENARIO_DEFINITIONS.find((definition) => (
    definition.pathAliases.includes(normalizedSegment)
  ));
}

function pickJsonScenarioType(record: JsonRecord): string | undefined {
  return pickString(
    record.scenario,
    record.kind,
    record.type,
    record.qrType,
    record.qr_type,
  );
}

function pickJsonId(record: JsonRecord, keys: readonly string[]): string | undefined {
  for (const key of keys) {
    const value = pickString(record[key]);
    if (value) {
      return value;
    }
  }
  return undefined;
}

function decodeUrlPathSegment(segment: string): string | null {
  try {
    return decodeURIComponent(segment).trim();
  } catch {
    return null;
  }
}

function getUrlPathSegments(url: URL): string[] | null {
  const segments = isSdkworkQrUrl(url)
    ? [url.hostname, ...url.pathname.split('/')]
    : url.pathname.split('/');
  const decodedSegments: string[] = [];
  for (const segment of segments) {
    const decodedSegment = decodeUrlPathSegment(segment);
    if (decodedSegment === null) {
      return null;
    }
    if (decodedSegment) {
      decodedSegments.push(decodedSegment);
    }
  }
  return decodedSegments;
}

function pickSearchParam(url: URL, ...keys: string[]): string | undefined {
  for (const key of keys) {
    const value = normalizeString(url.searchParams.get(key));
    if (value) {
      return value;
    }
  }
  return undefined;
}

function createUnknownPayload(rawContent: string, source: BaseQrCodePayload['source'] = 'text'): UnknownQrCodePayload {
  return {
    kind: 'unknown',
    rawContent,
    source,
  };
}

function isSafeHttpUrl(url: URL): boolean {
  return url.protocol === 'http:' || url.protocol === 'https:';
}

function isSdkworkQrUrl(url: URL): boolean {
  return url.protocol === 'sdkwork:' || url.protocol === 'im:';
}

function isTrustedSdkworkHttpHost(url: URL): boolean {
  return /(^|\.)sdkwork\.(?:com|cn)$/iu.test(url.hostname)
    || /(^|\.)im\.(?:com|cn)$/iu.test(url.hostname);
}

function createPayloadFromJson(rawContent: string, record: JsonRecord): QrCodeScanPayload | undefined {
  const nestedPayload = pickRecord(record.payload) ?? record;
  const definition = findScenarioDefinitionByType(
    pickJsonScenarioType(record) ?? pickJsonScenarioType(nestedPayload),
  );
  if (!definition) {
    return undefined;
  }

  const id = pickJsonId(nestedPayload, definition.primaryIdKeys);
  if (!id) {
    return undefined;
  }

  if (definition.kind === 'user') {
    return {
      kind: 'user',
      rawContent,
      source: 'json',
      userId: id,
      ...(pickString(nestedPayload.chatId, nestedPayload.chat_id) ? {
        chatId: pickString(nestedPayload.chatId, nestedPayload.chat_id),
      } : {}),
    };
  }

  if (definition.kind === 'group') {
    const conversationId = pickString(nestedPayload.conversationId, nestedPayload.conversation_id);
    return {
      kind: 'group',
      rawContent,
      source: 'json',
      groupId: id,
      ...(conversationId ? { conversationId } : {}),
      ...(pickString(nestedPayload.inviteCode, nestedPayload.invite_code, nestedPayload.token) ? {
        inviteCode: pickString(nestedPayload.inviteCode, nestedPayload.invite_code, nestedPayload.token),
      } : {}),
    };
  }

  return {
    kind: 'community',
    rawContent,
    source: 'json',
    communityId: id,
    ...(pickString(nestedPayload.inviteCode, nestedPayload.invite_code, nestedPayload.token) ? {
      inviteCode: pickString(nestedPayload.inviteCode, nestedPayload.invite_code, nestedPayload.token),
    } : {}),
  };
}

function createPayloadFromScenarioUrl(rawContent: string, url: URL): QrCodeScanPayload | undefined {
  const segments = getUrlPathSegments(url);
  if (segments === null) {
    return createUnknownPayload(rawContent, 'link');
  }
  const chatPrefixOffset = segments[0]?.toLowerCase() === 'chat' ? 1 : 0;
  const scenarioSegment = segments[chatPrefixOffset];
  const idSegment = segments[chatPrefixOffset + 1];
  const definition = findScenarioDefinitionByPathSegment(scenarioSegment);
  if (!definition) {
    return undefined;
  }
  const scenarioId = idSegment ?? pickSearchParam(url, ...definition.primaryIdKeys);
  if (!scenarioId) {
    return undefined;
  }

  if (definition.kind === 'user') {
    return {
      kind: 'user',
      rawContent,
      source: 'link',
      userId: scenarioId,
      ...(pickSearchParam(url, 'chatId', 'chat_id') ? {
        chatId: pickSearchParam(url, 'chatId', 'chat_id'),
      } : {}),
    };
  }

  if (definition.kind === 'group') {
    return {
      kind: 'group',
      rawContent,
      source: 'link',
      groupId: scenarioId,
      ...(pickSearchParam(url, 'conversationId', 'conversation_id') ? {
        conversationId: pickSearchParam(url, 'conversationId', 'conversation_id'),
      } : {}),
      ...(pickSearchParam(url, 'inviteCode', 'invite_code', 'token') ? {
        inviteCode: pickSearchParam(url, 'inviteCode', 'invite_code', 'token'),
      } : {}),
    };
  }

  return {
    kind: 'community',
    rawContent,
    source: 'link',
    communityId: scenarioId,
    ...(pickSearchParam(url, 'inviteCode', 'invite_code', 'token') ? {
      inviteCode: pickSearchParam(url, 'inviteCode', 'invite_code', 'token'),
    } : {}),
  };
}

function createPayloadFromUrl(rawContent: string): QrCodeScanPayload | undefined {
  let url: URL;
  try {
    url = new URL(rawContent);
  } catch {
    return undefined;
  }

  if (isSdkworkQrUrl(url)) {
    return createPayloadFromScenarioUrl(rawContent, url) ?? createUnknownPayload(rawContent, 'link');
  }

  if (!isSafeHttpUrl(url)) {
    return createUnknownPayload(rawContent, 'link');
  }

  if (isTrustedSdkworkHttpHost(url)) {
    const scenarioPayload = createPayloadFromScenarioUrl(rawContent, url);
    if (scenarioPayload) {
      return scenarioPayload;
    }
  }

  return {
    kind: 'url',
    rawContent,
    source: 'link',
    url: url.toString(),
  };
}

export function parseQrCodeContent(rawContent: string): QrCodeScanPayload {
  const normalizedContent = rawContent.trim();
  if (!normalizedContent) {
    throw new Error('QR code content is required');
  }

  const jsonPayload = parseJsonRecord(normalizedContent);
  if (jsonPayload) {
    return createPayloadFromJson(normalizedContent, jsonPayload)
      ?? createUnknownPayload(normalizedContent, 'json');
  }

  return createPayloadFromUrl(normalizedContent)
    ?? createUnknownPayload(normalizedContent);
}
