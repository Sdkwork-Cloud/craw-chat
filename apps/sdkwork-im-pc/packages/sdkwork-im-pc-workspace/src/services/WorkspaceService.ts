import {
  getAppSdkClientWithSession,
  type SdkworkImAppClient,
} from '@sdkwork/im-pc-core/sdk/appSdkClient';
import {
  getDriveAppSdkClientWithSession,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-pc-core/sdk/driveAppSdkClient';
import {
  readAppSdkSessionTokens,
} from '@sdkwork/im-pc-core/sdk/session';
import { WORKSPACE_APP_TAB_MAP, isCommercialRuntimeModule } from '@sdkwork/im-pc-shell';

export interface AppItem {
  id: string;
  name: string;
  nameKey: string;
  iconName: string;
  color: string;
}

export interface DocumentItem {
  id: string;
  name: string;
  nameKey: string;
  timestamp: number;
  type: string;
}

export interface WorkspaceService {
  getApps(): Promise<AppItem[]>;
  getRecentDocuments(): Promise<DocumentItem[]>;
  searchApps(query: string): Promise<AppItem[]>;
  addRecentDocument(doc: DocumentItem): Promise<void>;
  deleteRecentDocument(id: string): Promise<void>;
  addApp(app: AppItem): Promise<void>;
  removeApp(id: string): Promise<void>;
}

const REQUIRED_WORKSPACE_APP_IDS = new Set(['notary']);
const WORKSPACE_APPS_STORAGE_KEY = 'sdkwork-im-pc:workspace-apps';
const WORKSPACE_RECENT_DOCS_STORAGE_KEY = 'sdkwork-im-pc:workspace-recent-docs';
const DRIVE_RECENT_PAGE_SIZE = '12';

type RecordLike = Record<string, unknown>;

const workspaceAppCatalog: AppItem[] = [
  { id: 'notary', name: '公证业务', nameKey: 'apps.notary', iconName: 'ShieldCheck', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'calendar', name: '日程安排', nameKey: 'apps.calendar', iconName: 'Calendar', color: 'bg-blue-500/20 text-blue-400' },
  { id: 'approval', name: '审批', nameKey: 'apps.approval', iconName: 'CheckSquare', color: 'bg-green-500/20 text-green-400' },
  { id: 'report', name: '汇报', nameKey: 'apps.report', iconName: 'FileText', color: 'bg-orange-500/20 text-orange-400' },
  { id: 'mail', name: '企业邮箱', nameKey: 'apps.mail', iconName: 'Mail', color: 'bg-purple-500/20 text-purple-400' },
  { id: 'dashboard', name: '数据看板', nameKey: 'apps.dashboard', iconName: 'PieChart', color: 'bg-pink-500/20 text-pink-400' },
  { id: 'attendance', name: '考勤打卡', nameKey: 'apps.attendance', iconName: 'Clock', color: 'bg-yellow-500/20 text-yellow-400' },
  { id: 'drive', name: '云盘', nameKey: 'apps.drive', iconName: 'Cloud', color: 'bg-cyan-500/20 text-cyan-400' },
  { id: 'devices', name: '智能硬件', nameKey: 'apps.devices', iconName: 'Server', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'videogen', name: 'AI视频生成', nameKey: 'apps.videogen', iconName: 'Video', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'imagegen', name: 'AI图片生成', nameKey: 'apps.imagegen', iconName: 'ImageIcon', color: 'bg-blue-500/20 text-blue-400' },
  { id: 'voicegen', name: 'AI语音合成', nameKey: 'apps.voicegen', iconName: 'Mic', color: 'bg-green-500/20 text-green-400' },
  { id: 'musicgen', name: 'AI音乐生成', nameKey: 'apps.musicgen', iconName: 'Music', color: 'bg-purple-500/20 text-purple-400' },
  { id: 'writing', name: 'AI智能写作', nameKey: 'apps.writing', iconName: 'PenTool', color: 'bg-pink-500/20 text-pink-400' },
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

function readStoredApps(): AppItem[] {
  const storage = getLocalStorage();
  if (!storage) {
    return [];
  }
  try {
    const parsed = JSON.parse(storage.getItem(WORKSPACE_APPS_STORAGE_KEY) ?? '[]') as unknown;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .filter((item): item is AppItem => isRecord(item) && typeof item.id === 'string')
      .map((item) => ({
        id: item.id,
        name: pickString(item.name) ?? item.id,
        nameKey: pickString(item.nameKey) ?? item.id,
        iconName: pickString(item.iconName) ?? 'FileText',
        color: pickString(item.color) ?? 'bg-indigo-500/20 text-indigo-400',
      }));
  } catch {
    return [];
  }
}

function writeStoredApps(apps: AppItem[]): void {
  const storage = getLocalStorage();
  if (!storage) {
    return;
  }
  storage.setItem(WORKSPACE_APPS_STORAGE_KEY, JSON.stringify(apps));
}

function readStoredRecentDocuments(): DocumentItem[] {
  const storage = getLocalStorage();
  if (!storage) {
    return [];
  }
  try {
    const parsed = JSON.parse(storage.getItem(WORKSPACE_RECENT_DOCS_STORAGE_KEY) ?? '[]') as unknown;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .filter((item): item is DocumentItem => isRecord(item) && typeof item.id === 'string')
      .map((item) => ({
        id: item.id,
        name: pickString(item.name) ?? item.id,
        nameKey: pickString(item.nameKey) ?? pickString(item.name) ?? item.id,
        timestamp: typeof item.timestamp === 'number' ? item.timestamp : Date.now(),
        type: pickString(item.type) ?? 'unknown',
      }));
  } catch {
    return [];
  }
}

function writeStoredRecentDocuments(docs: DocumentItem[]): void {
  const storage = getLocalStorage();
  if (!storage) {
    return;
  }
  storage.setItem(WORKSPACE_RECENT_DOCS_STORAGE_KEY, JSON.stringify(docs));
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
  if (enabledModules.length === 0) {
    return true;
  }
  const moduleId = resolveWorkspaceModuleId(appId);
  return enabledModules.includes(moduleId) || enabledModules.includes(appId);
}

function buildCatalogApps(enabledModules: string[]): AppItem[] {
  return workspaceAppCatalog
    .filter((app) => isCommercialWorkspaceApp(app.id))
    .filter((app) => isWorkspaceAppEnabled(app.id, enabledModules));
}

function mergeApps(catalogApps: AppItem[], storedApps: AppItem[]): AppItem[] {
  const merged = new Map<string, AppItem>();
  for (const app of catalogApps) {
    merged.set(app.id, app);
  }
  for (const app of storedApps) {
    if (REQUIRED_WORKSPACE_APP_IDS.has(app.id)) {
      continue;
    }
    merged.set(app.id, app);
  }
  return Array.from(merged.values());
}

function parseTimestamp(value: unknown): number {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }
  const text = pickString(value);
  if (!text) {
    return Date.now();
  }
  const parsed = Date.parse(text);
  return Number.isFinite(parsed) ? parsed : Date.now();
}

function fileTypeFromName(name: string): string {
  const lowered = name.toLowerCase();
  if (lowered.endsWith('.pdf')) return 'pdf';
  if (lowered.endsWith('.doc') || lowered.endsWith('.docx')) return 'word';
  if (lowered.endsWith('.xls') || lowered.endsWith('.xlsx')) return 'excel';
  if (/\.(png|jpe?g|gif|webp|bmp|svg)$/u.test(lowered)) return 'image';
  return 'unknown';
}

function extractDriveNodes(value: unknown): RecordLike[] {
  if (Array.isArray(value)) {
    return value.filter(isRecord);
  }
  if (!isRecord(value)) {
    return [];
  }
  for (const key of ['items', 'nodes', 'records', 'data']) {
    const nodes = value[key];
    if (Array.isArray(nodes)) {
      return nodes.filter(isRecord);
    }
  }
  return [];
}

function mapDriveNodeToDocument(node: RecordLike): DocumentItem | undefined {
  const id = pickString(node.id) ?? pickString(node.nodeId) ?? pickString(node.node_id);
  const name = pickString(node.nodeName) ?? pickString(node.node_name) ?? pickString(node.name);
  if (!id || !name) {
    return undefined;
  }
  const timestamp = parseTimestamp(
    node.updatedAt ?? node.updated_at ?? node.lastAccessedAt ?? node.last_accessed_at,
  );
  return {
    id,
    name,
    nameKey: name,
    timestamp,
    type: fileTypeFromName(name),
  };
}

class SdkworkWorkspaceService implements WorkspaceService {
  constructor(
    private readonly getClient: () => SdkworkImAppClient = getAppSdkClientWithSession,
    private readonly getDriveClient: () => SdkworkDriveAppClient = getDriveAppSdkClientWithSession,
  ) {}

  async getApps(): Promise<AppItem[]> {
    let enabledModules: string[] = [];
    try {
      enabledModules = collectEnabledModules(await this.getClient().portal.home.retrieve());
    } catch (error) {
      console.warn('Failed to load workspace app catalog from app SDK portal.home', error);
    }
    return mergeApps(buildCatalogApps(enabledModules), readStoredApps());
  }

  async getRecentDocuments(): Promise<DocumentItem[]> {
    const storedDocs = readStoredRecentDocuments();
    try {
      const response = await this.getDriveClient().drive.recent.list({
        pageSize: DRIVE_RECENT_PAGE_SIZE,
      });
      const driveDocs = extractDriveNodes(response)
        .map(mapDriveNodeToDocument)
        .filter((doc): doc is DocumentItem => Boolean(doc));
      if (driveDocs.length > 0) {
        return driveDocs;
      }
    } catch (error) {
      console.warn('Failed to load workspace recent documents from drive SDK', error);
    }
    return storedDocs;
  }

  async searchApps(query: string): Promise<AppItem[]> {
    const lowered = query.trim().toLowerCase();
    const apps = await this.getApps();
    if (!lowered) {
      return apps;
    }
    return apps.filter(
      (app) => app.name.includes(query) || app.name.toLowerCase().includes(lowered) || app.id.includes(lowered),
    );
  }

  async addRecentDocument(doc: DocumentItem): Promise<void> {
    const docs = readStoredRecentDocuments().filter((item) => item.id !== doc.id);
    docs.unshift(doc);
    writeStoredRecentDocuments(docs.slice(0, 20));
  }

  async deleteRecentDocument(id: string): Promise<void> {
    writeStoredRecentDocuments(readStoredRecentDocuments().filter((doc) => doc.id !== id));
  }

  async addApp(app: AppItem): Promise<void> {
    if (REQUIRED_WORKSPACE_APP_IDS.has(app.id)) {
      return;
    }
    const storedApps = readStoredApps().filter((item) => item.id !== app.id);
    storedApps.push(app);
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
