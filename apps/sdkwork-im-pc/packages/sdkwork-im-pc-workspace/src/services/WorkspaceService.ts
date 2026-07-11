import {
  getAppSdkClientWithSession,
  retrievePortalHome,
  type SdkworkImAppClient,
} from '@sdkwork/im-pc-core/sdk/appSdkClient';
import {
  getDriveAppSdkClientWithSession,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-pc-core/sdk/driveAppSdkClient';
import {
  readAppSdkSessionTokens,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
} from '@sdkwork/im-pc-core/sdk/session';
import { WORKSPACE_APP_TAB_MAP, isCommercialRuntimeModule } from '@sdkwork/im-pc-shell';

export interface AppItem {
  id: string;
  nameKey: string;
  iconName: string;
  color: string;
  pinned: boolean;
  required: boolean;
}

export interface DocumentItem {
  id: string;
  name: string;
  nameKey: string;
  timestamp: number;
  type: string;
  kind?: WorkspaceDocumentKind;
  node?: WorkspaceDriveNodeMetadata;
  activity?: WorkspaceDocumentActivity;
  openTarget?: WorkspaceDocumentOpenTarget;
}

export interface WorkspaceRecentDocument extends DocumentItem {
  kind: WorkspaceDocumentKind;
  node: WorkspaceDriveNodeMetadata;
  activity: WorkspaceDocumentActivity;
  openTarget?: WorkspaceDocumentOpenTarget;
}

export type WorkspaceDriveNodeType = 'file' | 'folder' | 'shortcut' | 'virtual_reference';

export type WorkspaceDocumentKind =
  | 'folder'
  | 'shortcut'
  | 'pdf'
  | 'word'
  | 'spreadsheet'
  | 'presentation'
  | 'image'
  | 'video'
  | 'audio'
  | 'text'
  | 'archive'
  | 'document'
  | 'binary'
  | 'unknown';

export interface WorkspaceDriveNodeMetadata {
  nodeId: string;
  spaceId?: string;
  parentNodeId?: string;
  nodeType: WorkspaceDriveNodeType;
  shortcutTargetNodeId?: string;
  lifecycleStatus?: string;
  version?: string;
  spaceType?: string;
  contentState?: string;
  fileExtension?: string;
  contentType?: string;
  contentTypeGroup?: string;
  contentLength?: string;
  folderColor?: string;
}

export type WorkspaceDocumentActivityKind =
  | 'unknown'
  | 'created'
  | 'last-accessed'
  | 'last-modified'
  | 'modified';

export interface WorkspaceDocumentActivity {
  kind: WorkspaceDocumentActivityKind;
  occurredAt: string | null;
  timestamp: number | null;
}

export interface WorkspaceDocumentOpenTarget {
  kind: 'drive-node';
  appId: 'drive';
  resourceType: 'drive-node';
  resourceId: string;
  spaceId?: string;
  section: 'recent';
  intent: 'preview';
}

export interface WorkspacePageInfo {
  mode: 'offset' | 'cursor';
  page?: number;
  pageSize?: number;
  totalItems?: string;
  totalPages?: number;
  nextCursor?: string | null;
  hasMore: boolean;
  incompletePage?: boolean;
}

export type WorkspaceDataSource =
  | 'remote'
  | 'fallback'
  | 'offline'
  | 'permission-denied'
  | 'unavailable';

export interface WorkspaceCollection<TItem> {
  items: TItem[];
  source: WorkspaceDataSource;
  pageInfo?: WorkspacePageInfo;
}

export interface WorkspaceData {
  apps: WorkspaceCollection<AppItem>;
  documents: WorkspaceCollection<DocumentItem>;
}

export interface WorkspaceService {
  getApps(): Promise<AppItem[]>;
  getRecentDocuments(): Promise<DocumentItem[]>;
  getWorkspaceData(): Promise<WorkspaceData>;
  searchApps(query: string): Promise<AppItem[]>;
  savePinnedAppIds(ids: string[]): Promise<void>;
  addRecentDocument(doc: DocumentItem): Promise<void>;
  deleteRecentDocument(id: string): Promise<void>;
  addApp(app: AppItem): Promise<void>;
  removeApp(id: string): Promise<void>;
}

const REQUIRED_WORKSPACE_APP_IDS = new Set(['notary']);
const WORKSPACE_APPS_STORAGE_KEY = 'sdkwork-im-pc:workspace-apps';
const WORKSPACE_PINNED_APPS_STORAGE_KEY = 'sdkwork-im-pc:workspace-pinned-apps:v1';
const WORKSPACE_RECENT_DOCS_STORAGE_KEY = 'sdkwork-im-pc:workspace-recent-docs';
const DRIVE_RECENT_PAGE_SIZE = '12';
const WORKSPACE_RECENT_DOCS_CACHE_LIMIT = 20;
const WORKSPACE_RECENT_DOCS_CACHE_SCHEMA_VERSION = 1;

type RecordLike = Record<string, unknown>;
type WorkspaceAppDefinition = Omit<AppItem, 'pinned' | 'required'>;

interface StoredRecentDocuments {
  items: DocumentItem[];
  pageInfo: WorkspacePageInfo;
}

/** Commercial-runtime workbench catalog only; contract-pending apps ship in sibling products first. */
const workspaceAppCatalog: WorkspaceAppDefinition[] = [
  { id: 'notary', nameKey: 'apps.notary', iconName: 'ShieldCheck', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'drive', nameKey: 'apps.drive', iconName: 'Cloud', color: 'bg-cyan-500/20 text-cyan-400' },
  { id: 'knowledge', nameKey: 'apps.knowledge', iconName: 'BookOpen', color: 'bg-emerald-500/20 text-emerald-400' },
  { id: 'community', nameKey: 'apps.community', iconName: 'Users', color: 'bg-sky-500/20 text-sky-400' },
  { id: 'voice', nameKey: 'apps.voice', iconName: 'Mic', color: 'bg-violet-500/20 text-violet-400' },
  { id: 'shop', nameKey: 'apps.shop', iconName: 'Store', color: 'bg-amber-500/20 text-amber-400' },
  { id: 'orders', nameKey: 'apps.orders', iconName: 'ReceiptText', color: 'bg-orange-500/20 text-orange-400' },
];

function isRecord(value: unknown): value is RecordLike {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function pickString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .map((item) => pickString(item))
    .filter((item): item is string => Boolean(item));
}

function collectEnabledModules(snapshot: unknown): string[] {
  if (!isRecord(snapshot)) {
    return [];
  }
  for (const key of [
    'enabledModules',
    'enabled_modules',
    'sidebarModules',
    'sidebar_modules',
    'modules',
  ]) {
    const modules = pickStringArray(snapshot[key]);
    if (modules.length > 0) {
      return modules;
    }
    if (isRecord(snapshot[key])) {
      const nestedModules = pickStringArray((snapshot[key] as RecordLike).items);
      if (nestedModules.length > 0) {
        return nestedModules;
      }
    }
  }
  return [];
}

function getLocalStorage(): Storage | undefined {
  return typeof localStorage === 'undefined' ? undefined : localStorage;
}

function resolveWorkspaceStorageKey(baseKey: string): string | undefined {
  const session = readAppSdkSessionTokens();
  const tenantId = resolveAppSdkTenantId(session);
  const userId = resolveAppSdkUserId(session);
  if (!tenantId || !userId) {
    return undefined;
  }
  return `${baseKey}:${encodeURIComponent(tenantId)}:${encodeURIComponent(userId)}`;
}

function readStoredApps(): WorkspaceAppDefinition[] {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_APPS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return [];
  }
  try {
    const parsed = JSON.parse(storage.getItem(storageKey) ?? '[]') as unknown;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .filter((item): item is RecordLike & { id: string } => isRecord(item) && typeof item.id === 'string')
      .map((item) => ({
        id: item.id,
        nameKey: pickString(item.nameKey) ?? `apps.${item.id}`,
        iconName: pickString(item.iconName) ?? 'FileText',
        color: pickString(item.color) ?? 'bg-indigo-500/20 text-indigo-400',
      }));
  } catch {
    return [];
  }
}

function writeStoredApps(apps: WorkspaceAppDefinition[]): void {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_APPS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return;
  }
  storage.setItem(storageKey, JSON.stringify(apps));
}

function readPinnedAppIds(): Set<string> | undefined {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_PINNED_APPS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return undefined;
  }
  const rawValue = storage.getItem(storageKey);
  if (!rawValue) {
    return undefined;
  }
  try {
    const parsed = JSON.parse(rawValue) as unknown;
    if (!Array.isArray(parsed)) {
      return undefined;
    }
    return new Set(pickStringArray(parsed));
  } catch {
    storage.removeItem(storageKey);
    return undefined;
  }
}

function writePinnedAppIds(ids: string[]): void {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_PINNED_APPS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return;
  }
  const normalizedIds = new Set(pickStringArray(ids));
  for (const requiredId of REQUIRED_WORKSPACE_APP_IDS) {
    normalizedIds.add(requiredId);
  }
  storage.setItem(storageKey, JSON.stringify(Array.from(normalizedIds)));
}

function readStoredRecentDocuments(): StoredRecentDocuments {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_RECENT_DOCS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return {
      items: [],
      pageInfo: createDefaultPageInfo(0),
    };
  }
  try {
    const parsed = JSON.parse(storage.getItem(storageKey) ?? 'null') as unknown;
    const rawItems = Array.isArray(parsed)
      ? parsed
      : isRecord(parsed) && Array.isArray(parsed.items)
        ? parsed.items
        : [];
    const items = rawItems
      .map(normalizeStoredDocumentItem)
      .filter((item): item is DocumentItem => Boolean(item))
      .slice(0, WORKSPACE_RECENT_DOCS_CACHE_LIMIT);
    const pageInfo = isRecord(parsed)
      ? normalizePageInfo(parsed.pageInfo, items.length)
      : createDefaultPageInfo(items.length);
    return { items, pageInfo };
  } catch {
    storage.removeItem(storageKey);
    return {
      items: [],
      pageInfo: createDefaultPageInfo(0),
    };
  }
}

function writeStoredRecentDocuments(value: StoredRecentDocuments): void {
  const storage = getLocalStorage();
  const storageKey = resolveWorkspaceStorageKey(WORKSPACE_RECENT_DOCS_STORAGE_KEY);
  if (!storage || !storageKey) {
    return;
  }
  const items = value.items.slice(0, WORKSPACE_RECENT_DOCS_CACHE_LIMIT);
  try {
    storage.setItem(storageKey, JSON.stringify({
      schemaVersion: WORKSPACE_RECENT_DOCS_CACHE_SCHEMA_VERSION,
      items,
      pageInfo: value.pageInfo,
    }));
  } catch {
    // Recent documents are a recoverable cache; storage quota failures must not hide remote data.
  }
}

function resolveWorkspaceModuleId(appId: string): string {
  return WORKSPACE_APP_TAB_MAP[appId] ?? appId;
}

function isCommercialWorkspaceApp(appId: string): boolean {
  const moduleId = resolveWorkspaceModuleId(appId);
  return isCommercialRuntimeModule(moduleId);
}

function isWorkspaceAppEnabled(appId: string, enabledModules: string[]): boolean {
  if (REQUIRED_WORKSPACE_APP_IDS.has(appId)) {
    return true;
  }
  const moduleId = resolveWorkspaceModuleId(appId);
  return enabledModules.includes(moduleId) || enabledModules.includes(appId);
}

function buildCatalogApps(enabledModules: string[]): WorkspaceAppDefinition[] {
  return workspaceAppCatalog
    .filter((app) => isCommercialWorkspaceApp(app.id))
    .filter((app) => isWorkspaceAppEnabled(app.id, enabledModules));
}

function mergeApps(
  catalogApps: WorkspaceAppDefinition[],
  storedApps: WorkspaceAppDefinition[],
): WorkspaceAppDefinition[] {
  const storedById = new Map(storedApps.map((app) => [app.id, app]));
  return catalogApps.map((app) => {
    const storedApp = storedById.get(app.id);
    if (!storedApp || REQUIRED_WORKSPACE_APP_IDS.has(app.id)) {
      return app;
    }
    return {
      ...app,
      color: storedApp.color,
      iconName: storedApp.iconName,
    };
  });
}

function readErrorStatus(error: unknown): number | undefined {
  if (!isRecord(error)) {
    return undefined;
  }
  for (const value of [error.httpStatus, error.status, error.statusCode]) {
    if (typeof value === 'number') {
      return value;
    }
  }
  const response = isRecord(error.response) ? error.response : undefined;
  return typeof response?.status === 'number' ? response.status : undefined;
}

function classifyWorkspaceDataError(
  error: unknown,
): Exclude<WorkspaceDataSource, 'remote' | 'fallback'> {
  const status = readErrorStatus(error);
  const errorRecord = isRecord(error) ? error : undefined;
  const code = pickString(errorRecord?.code)?.toUpperCase() ?? '';
  const message = error instanceof Error ? error.message : String(error ?? '');
  if (
    status === 401
    || status === 403
    || code === 'UNAUTHORIZED'
    || code === 'FORBIDDEN'
    || /\b(?:401|403)\b|unauthori[sz]ed|forbidden/iu.test(message)
  ) {
    return 'permission-denied';
  }
  if (
    code === 'NETWORK_ERROR'
    || code === 'TIMEOUT'
    || /offline|network|timeout|failed to fetch|network request/iu.test(message)
  ) {
    return 'offline';
  }
  return 'unavailable';
}

function applyPinnedPreferences(apps: WorkspaceAppDefinition[]): AppItem[] {
  const storedPinnedIds = readPinnedAppIds();
  const pinnedIds = storedPinnedIds ?? new Set(apps.map((app) => app.id));
  return apps.map((app) => {
    const required = REQUIRED_WORKSPACE_APP_IDS.has(app.id);
    return {
      ...app,
      pinned: required || pinnedIds.has(app.id),
      required,
    };
  });
}

function pickFiniteNumber(value: unknown): number | undefined {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }
  const text = pickString(value);
  if (!text) {
    return undefined;
  }
  const parsed = Number(text);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function pickNonNegativeInteger(value: unknown): number | undefined {
  const parsed = pickFiniteNumber(value);
  return parsed !== undefined && Number.isSafeInteger(parsed) && parsed >= 0
    ? parsed
    : undefined;
}

function pickBoolean(value: unknown): boolean | undefined {
  return typeof value === 'boolean' ? value : undefined;
}

function pickDriveNodeType(value: unknown): WorkspaceDriveNodeType {
  return value === 'folder' || value === 'shortcut' || value === 'virtual_reference'
    ? value
    : 'file';
}

function pickDocumentKind(value: unknown): WorkspaceDocumentKind | undefined {
  const values: WorkspaceDocumentKind[] = [
    'folder',
    'shortcut',
    'pdf',
    'word',
    'spreadsheet',
    'presentation',
    'image',
    'video',
    'audio',
    'text',
    'archive',
    'document',
    'binary',
    'unknown',
  ];
  return values.includes(value as WorkspaceDocumentKind)
    ? value as WorkspaceDocumentKind
    : undefined;
}

function normalizeFileExtension(value: unknown, name: string): string | undefined {
  const declared = pickString(value)?.replace(/^\.+/u, '').toLowerCase();
  if (declared) {
    return declared;
  }
  const dotIndex = name.lastIndexOf('.');
  return dotIndex >= 0 && dotIndex < name.length - 1
    ? name.slice(dotIndex + 1).toLowerCase()
    : undefined;
}

function resolveDocumentKind(
  nodeType: WorkspaceDriveNodeType,
  fileExtension: string | undefined,
  contentType: string | undefined,
  contentTypeGroup: string | undefined,
): WorkspaceDocumentKind {
  if (nodeType === 'folder') return 'folder';
  if (nodeType === 'shortcut' || nodeType === 'virtual_reference') return 'shortcut';

  const extension = fileExtension?.toLowerCase();
  const mime = contentType?.toLowerCase();
  if (extension === 'pdf' || mime === 'application/pdf') return 'pdf';
  if (
    ['doc', 'docx', 'odt', 'rtf'].includes(extension ?? '')
    || mime?.includes('word')
    || mime?.includes('opendocument.text')
  ) return 'word';
  if (
    ['xls', 'xlsx', 'ods', 'csv'].includes(extension ?? '')
    || mime?.includes('spreadsheet')
    || mime?.includes('excel')
    || mime === 'text/csv'
  ) return 'spreadsheet';
  if (
    ['ppt', 'pptx', 'odp'].includes(extension ?? '')
    || mime?.includes('presentation')
    || mime?.includes('powerpoint')
  ) return 'presentation';
  if (contentTypeGroup === 'image' || mime?.startsWith('image/')) return 'image';
  if (contentTypeGroup === 'video' || mime?.startsWith('video/')) return 'video';
  if (contentTypeGroup === 'audio' || mime?.startsWith('audio/')) return 'audio';
  if (contentTypeGroup === 'archive') return 'archive';
  if (contentTypeGroup === 'text' || mime?.startsWith('text/')) return 'text';
  if (contentTypeGroup === 'document') return 'document';
  if (contentTypeGroup === 'binary') return 'binary';
  return 'unknown';
}

function legacyDocumentType(kind: WorkspaceDocumentKind): string {
  return kind === 'spreadsheet' ? 'excel' : kind;
}

function normalizeEpochMilliseconds(value: number): number {
  return Math.abs(value) < 100_000_000_000 ? value * 1000 : value;
}

function parseActivityTimestamp(value: unknown): { occurredAt: string; timestamp: number } | undefined {
  const numeric = pickFiniteNumber(value);
  const timestamp = numeric !== undefined
    ? normalizeEpochMilliseconds(numeric)
    : Date.parse(pickString(value) ?? '');
  if (!Number.isFinite(timestamp)) {
    return undefined;
  }
  return {
    occurredAt: new Date(timestamp).toISOString(),
    timestamp,
  };
}

function resolveDocumentActivity(node: RecordLike): WorkspaceDocumentActivity {
  const candidates: Array<[WorkspaceDocumentActivityKind, unknown]> = [
    ['last-accessed', node.lastAccessedAt ?? node.last_accessed_at ?? node.accessedAt ?? node.accessed_at ?? node.recentAt ?? node.recent_at],
    ['last-modified', node.updatedAt ?? node.updated_at ?? node.modifiedAt ?? node.modified_at],
    ['created', node.createdAt ?? node.created_at],
  ];
  for (const [kind, value] of candidates) {
    const activity = parseActivityTimestamp(value);
    if (activity) {
      return { kind, ...activity };
    }
  }
  return {
    kind: 'unknown',
    occurredAt: null,
    timestamp: null,
  };
}

function createDefaultPageInfo(itemCount: number): WorkspacePageInfo {
  return {
    mode: 'cursor',
    pageSize: itemCount,
    nextCursor: null,
    hasMore: false,
  };
}

function normalizePageInfo(value: unknown, itemCount: number): WorkspacePageInfo {
  if (!isRecord(value)) {
    return createDefaultPageInfo(itemCount);
  }
  const nextCursor = value.nextCursor === null
    ? null
    : pickString(value.nextCursor ?? value.next_cursor);
  const totalItemsValue = value.totalItems ?? value.total_items;
  const totalItems = pickString(totalItemsValue)
    ?? (pickFiniteNumber(totalItemsValue) !== undefined ? String(pickFiniteNumber(totalItemsValue)) : undefined);
  return {
    mode: value.mode === 'offset' ? 'offset' : 'cursor',
    page: pickNonNegativeInteger(value.page),
    pageSize: pickNonNegativeInteger(value.pageSize ?? value.page_size) ?? itemCount,
    totalItems,
    totalPages: pickNonNegativeInteger(value.totalPages ?? value.total_pages),
    nextCursor: nextCursor ?? null,
    hasMore: pickBoolean(value.hasMore ?? value.has_more) ?? Boolean(nextCursor),
    incompletePage: pickBoolean(value.incompletePage ?? value.incomplete_page),
  };
}

function normalizeDriveNodeMetadata(node: RecordLike, id: string, name: string): WorkspaceDriveNodeMetadata {
  const nodeType = pickDriveNodeType(node.nodeType ?? node.node_type);
  const fileExtension = normalizeFileExtension(node.fileExtension ?? node.file_extension, name);
  const versionValue = node.version;
  const contentLengthValue = node.contentLength ?? node.content_length;
  return {
    nodeId: id,
    spaceId: pickString(node.spaceId ?? node.space_id),
    parentNodeId: pickString(node.parentNodeId ?? node.parent_node_id),
    nodeType,
    shortcutTargetNodeId: pickString(node.shortcutTargetNodeId ?? node.shortcut_target_node_id),
    lifecycleStatus: pickString(node.lifecycleStatus ?? node.lifecycle_status),
    version: pickString(versionValue)
      ?? (pickFiniteNumber(versionValue) !== undefined ? String(pickFiniteNumber(versionValue)) : undefined),
    spaceType: pickString(node.spaceType ?? node.space_type),
    contentState: pickString(node.contentState ?? node.content_state),
    fileExtension,
    contentType: pickString(node.contentType ?? node.content_type),
    contentTypeGroup: pickString(node.contentTypeGroup ?? node.content_type_group),
    contentLength: pickString(contentLengthValue)
      ?? (pickFiniteNumber(contentLengthValue) !== undefined ? String(pickFiniteNumber(contentLengthValue)) : undefined),
  };
}

function createDocumentOpenTarget(node: WorkspaceDriveNodeMetadata): WorkspaceDocumentOpenTarget {
  return {
    kind: 'drive-node',
    appId: 'drive',
    resourceType: 'drive-node',
    resourceId: node.nodeId,
    spaceId: node.spaceId,
    section: 'recent',
    intent: 'preview',
  };
}

function normalizeStoredDocumentItem(value: unknown): DocumentItem | undefined {
  if (!isRecord(value)) {
    return undefined;
  }
  const id = pickString(value.id) ?? pickString(value.nodeId) ?? pickString(value.node_id);
  const name = pickString(value.name) ?? pickString(value.nodeName) ?? pickString(value.node_name);
  if (!id || !name) {
    return undefined;
  }
  const nodeRecord = isRecord(value.node)
    ? { ...value.node, id, nodeName: name }
    : { ...value, id, nodeName: name };
  const node = normalizeDriveNodeMetadata(nodeRecord, id, name);
  const storedActivity = isRecord(value.activity) ? value.activity : undefined;
  const storedTimestamp = parseActivityTimestamp(storedActivity?.timestamp ?? value.timestamp);
  const activityKind = storedActivity?.kind === 'last-accessed'
    || storedActivity?.kind === 'last-modified'
    || storedActivity?.kind === 'created'
    ? storedActivity.kind
    : 'unknown';
  const activity: WorkspaceDocumentActivity = storedTimestamp
    ? {
        kind: activityKind,
        occurredAt: pickString(storedActivity?.occurredAt) ?? storedTimestamp.occurredAt,
        timestamp: storedTimestamp.timestamp,
      }
    : {
        kind: 'unknown',
        occurredAt: null,
        timestamp: null,
      };
  const kind = pickDocumentKind(value.kind)
    ?? resolveDocumentKind(node.nodeType, node.fileExtension, node.contentType, node.contentTypeGroup);
  return {
    id,
    name,
    nameKey: pickString(value.nameKey) ?? name,
    timestamp: activity.timestamp ?? Number.NaN,
    type: pickString(value.type) ?? legacyDocumentType(kind),
    kind,
    node,
    activity,
    openTarget: createDocumentOpenTarget(node),
  };
}

function extractDriveRecentPage(value: unknown): { nodes: RecordLike[]; pageInfo: WorkspacePageInfo } | undefined {
  if (!isRecord(value) || !Array.isArray(value.items) || !isRecord(value.pageInfo)) {
    return undefined;
  }
  const nodes = value.items.filter(isRecord);
  const pageInfo = normalizePageInfo(value.pageInfo, nodes.length);
  const incompletePage = pickBoolean(value.incompletePage ?? value.incomplete_page);
  return {
    nodes,
    pageInfo: incompletePage === undefined
      ? pageInfo
      : { ...pageInfo, incompletePage },
  };
}

function mapDriveNodeToDocument(node: RecordLike): DocumentItem | undefined {
  const id = pickString(node.id) ?? pickString(node.nodeId) ?? pickString(node.node_id);
  const name = pickString(node.nodeName) ?? pickString(node.node_name) ?? pickString(node.name);
  if (!id || !name) {
    return undefined;
  }
  const metadata = normalizeDriveNodeMetadata(node, id, name);
  const kind = resolveDocumentKind(
    metadata.nodeType,
    metadata.fileExtension,
    metadata.contentType,
    metadata.contentTypeGroup,
  );
  const activity = resolveDocumentActivity(node);
  return {
    id,
    name,
    nameKey: name,
    timestamp: activity.timestamp ?? Number.NaN,
    type: legacyDocumentType(kind),
    kind,
    node: metadata,
    activity,
    openTarget: createDocumentOpenTarget(metadata),
  };
}

class SdkworkWorkspaceService implements WorkspaceService {
  constructor(
    private readonly getClient: () => SdkworkImAppClient = getAppSdkClientWithSession,
    private readonly getDriveClient: () => SdkworkDriveAppClient = getDriveAppSdkClientWithSession,
  ) {}

  private async loadApps(): Promise<WorkspaceCollection<AppItem>> {
    let enabledModules: string[] = [];
    try {
      enabledModules = collectEnabledModules(await retrievePortalHome(this.getClient()));
      return {
        items: applyPinnedPreferences(mergeApps(buildCatalogApps(enabledModules), readStoredApps())),
        source: 'remote',
      };
    } catch (error) {
      return {
        items: applyPinnedPreferences(mergeApps(buildCatalogApps(enabledModules), readStoredApps())),
        source: classifyWorkspaceDataError(error),
      };
    }
  }

  private async loadRecentDocuments(): Promise<WorkspaceCollection<DocumentItem>> {
    const storedDocuments = readStoredRecentDocuments();
    try {
      const response = await this.getDriveClient().drive.recent.list({
        pageSize: DRIVE_RECENT_PAGE_SIZE,
      });
      const page = extractDriveRecentPage(response);
      if (!page) {
        throw new Error('Drive recent list returned an invalid page payload.');
      }
      const driveDocs = page.nodes
        .map(mapDriveNodeToDocument)
        .filter((doc): doc is DocumentItem => Boolean(doc));
      writeStoredRecentDocuments({
        items: driveDocs,
        pageInfo: page.pageInfo,
      });
      return {
        items: driveDocs,
        source: 'remote',
        pageInfo: page.pageInfo,
      };
    } catch (error) {
      return {
        items: storedDocuments.items,
        source: classifyWorkspaceDataError(error),
        pageInfo: storedDocuments.pageInfo,
      };
    }
  }

  async getApps(): Promise<AppItem[]> {
    return (await this.loadApps()).items;
  }

  async getRecentDocuments(): Promise<DocumentItem[]> {
    return (await this.loadRecentDocuments()).items;
  }

  async getWorkspaceData(): Promise<WorkspaceData> {
    const [apps, documents] = await Promise.all([
      this.loadApps(),
      this.loadRecentDocuments(),
    ]);
    return { apps, documents };
  }

  async searchApps(query: string): Promise<AppItem[]> {
    const lowered = query.trim().toLowerCase();
    const apps = await this.getApps();
    if (!lowered) {
      return apps;
    }
    return apps.filter(
      (app) => app.id.includes(lowered) || app.nameKey.toLowerCase().includes(lowered),
    );
  }

  async savePinnedAppIds(ids: string[]): Promise<void> {
    writePinnedAppIds(ids);
  }

  async addRecentDocument(doc: DocumentItem): Promise<void> {
    const storedDocuments = readStoredRecentDocuments();
    const normalizedDocument = normalizeStoredDocumentItem(doc);
    if (!normalizedDocument) {
      return;
    }
    const docs = storedDocuments.items.filter((item) => item.id !== normalizedDocument.id);
    docs.unshift(normalizedDocument);
    writeStoredRecentDocuments({
      items: docs,
      pageInfo: {
        ...storedDocuments.pageInfo,
        pageSize: Math.min(docs.length, WORKSPACE_RECENT_DOCS_CACHE_LIMIT),
      },
    });
  }

  async deleteRecentDocument(id: string): Promise<void> {
    const storedDocuments = readStoredRecentDocuments();
    const items = storedDocuments.items.filter((doc) => doc.id !== id);
    writeStoredRecentDocuments({
      items,
      pageInfo: {
        ...storedDocuments.pageInfo,
        pageSize: items.length,
      },
    });
  }

  async addApp(app: AppItem): Promise<void> {
    if (REQUIRED_WORKSPACE_APP_IDS.has(app.id)) {
      return;
    }
    const storedApps = readStoredApps().filter((item) => item.id !== app.id);
    storedApps.push({
      id: app.id,
      nameKey: app.nameKey,
      iconName: app.iconName,
      color: app.color,
    });
    writeStoredApps(storedApps);
  }

  async removeApp(id: string): Promise<void> {
    if (REQUIRED_WORKSPACE_APP_IDS.has(id)) {
      return;
    }
    writeStoredApps(readStoredApps().filter((app) => app.id !== id));
  }
}

export function createSdkworkWorkspaceService(
  getClient?: () => SdkworkImAppClient,
  getDriveClient?: () => SdkworkDriveAppClient,
): WorkspaceService {
  return new SdkworkWorkspaceService(getClient, getDriveClient);
}

export const workspaceService = createSdkworkWorkspaceService();
