import assert from 'node:assert/strict';

import {
  createSdkworkWorkspaceService,
  type AppItem,
  type DocumentItem,
} from '../packages/sdkwork-im-pc-workspace/src/services/WorkspaceService';

class MemoryStorage implements Storage {
  private readonly values = new Map<string, string>();

  get length(): number {
    return this.values.size;
  }

  clear(): void {
    this.values.clear();
  }

  getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.values.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.values.delete(key);
  }

  setItem(key: string, value: string): void {
    this.values.set(key, value);
  }
}

const localStorage = new MemoryStorage();
Object.defineProperty(globalThis, 'localStorage', {
  configurable: true,
  value: localStorage,
});

type WorkspaceTestGlobal = typeof globalThis & {
  __workspaceTestSessionScope?: {
    tenantId: string;
    userId: string;
  };
};

const testGlobal = globalThis as WorkspaceTestGlobal;

function setSessionScope(tenantId: string, userId: string): void {
  testGlobal.__workspaceTestSessionScope = { tenantId, userId };
}

function recentDocumentsStorageKey(tenantId: string, userId: string): string {
  return `sdkwork-im-pc:workspace-recent-docs:${encodeURIComponent(tenantId)}:${encodeURIComponent(userId)}`;
}

function createRecentPage(items: unknown[], overrides: Record<string, unknown> = {}) {
  return {
    items,
    pageInfo: {
      mode: 'cursor',
      pageSize: items.length,
      totalItems: String(items.length),
      nextCursor: null,
      hasMore: false,
      ...overrides,
    },
    incompletePage: false,
  };
}

setSessionScope('tenant-a', 'user-a');

const appClient = {
  portal: {
    home: {
      async retrieve() {
        return { enabledModules: ['notary', 'drive', 'knowledge'] };
      },
    },
  },
};

const driveNodes = [
  {
    id: 'doc-planning',
    spaceId: 'space personal',
    parentNodeId: 'folder-planning',
    nodeType: 'file',
    nodeName: 'Planning.docx',
    lifecycleStatus: 'active',
    version: '7',
    spaceType: 'personal',
    contentState: 'ready',
    fileExtension: 'docx',
    contentType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    contentTypeGroup: 'document',
    contentLength: '4096',
    lastAccessedAt: '2026-07-10T12:00:00.000Z',
    url: 'javascript:alert(1)',
  },
  {
    id: 'doc-architecture',
    spaceId: 'space-personal',
    nodeType: 'file',
    nodeName: 'Architecture',
    lifecycleStatus: 'active',
    version: 3,
    spaceType: 'personal',
    fileExtension: 'pdf',
    contentType: 'application/pdf',
    contentTypeGroup: 'document',
    updatedAt: '2026-07-09T08:15:00.000Z',
  },
  {
    id: 'doc-metrics',
    spaceId: 'space-personal',
    nodeType: 'file',
    nodeName: 'Metrics.xlsx',
    lifecycleStatus: 'active',
    version: 2,
    spaceType: 'personal',
    fileExtension: '.XLSX',
    contentTypeGroup: 'document',
    updatedAt: 1_783_610_100,
  },
  {
    id: 'doc-cover',
    spaceId: 'space-personal',
    nodeType: 'file',
    nodeName: 'Cover',
    lifecycleStatus: 'active',
    version: 1,
    spaceType: 'personal',
    contentType: 'image/png',
    contentTypeGroup: 'image',
  },
  {
    id: 'folder-projects',
    spaceId: 'space-personal',
    nodeType: 'folder',
    nodeName: 'Projects',
    lifecycleStatus: 'active',
    version: 4,
    spaceType: 'personal',
  },
  {
    id: 'shortcut-roadmap',
    spaceId: 'space-personal',
    nodeType: 'shortcut',
    nodeName: 'Roadmap shortcut',
    shortcutTargetNodeId: 'doc-roadmap',
    lifecycleStatus: 'active',
    version: 1,
    spaceType: 'personal',
    updatedAt: 'not-a-date',
  },
];

const driveClient = {
  drive: {
    recent: {
      async list(params: { pageSize: string }) {
        assert.equal(params.pageSize, '12');
        return createRecentPage(driveNodes, {
          pageSize: 12,
          totalItems: '6',
        });
      },
    },
  },
};

const service = createSdkworkWorkspaceService(
  () => appClient as never,
  () => driveClient as never,
);

assert.equal(typeof service.getWorkspaceData, 'function', 'workspace service must expose catalog and document source status');
const remoteWorkspaceData = await service.getWorkspaceData();
assert.equal(remoteWorkspaceData.apps.source, 'remote');
assert.equal(remoteWorkspaceData.documents.source, 'remote');
assert.deepEqual(remoteWorkspaceData.documents.pageInfo, {
  mode: 'cursor',
  page: undefined,
  pageSize: 12,
  totalItems: '6',
  totalPages: undefined,
  nextCursor: null,
  hasMore: false,
  incompletePage: false,
});

const initialApps = await service.getApps();
assert.deepEqual(
  initialApps.map((app) => app.id),
  ['notary', 'drive', 'knowledge'],
  'enabled commercial modules should form the initial workbench catalog',
);
assert.ok(initialApps.every((app) => app.pinned === true), 'catalog apps should be pinned by default');
assert.equal(initialApps.find((app) => app.id === 'notary')?.required, true);
assert.equal(initialApps.find((app) => app.id === 'knowledge')?.iconName, 'BookOpen');

const recentDocuments = remoteWorkspaceData.documents.items;
assert.equal(recentDocuments.length, driveNodes.length);
const planning = recentDocuments.find((item) => item.id === 'doc-planning');
assert.equal(planning?.kind, 'word');
assert.equal(planning?.type, 'word');
assert.deepEqual(planning?.node, {
  nodeId: 'doc-planning',
  spaceId: 'space personal',
  parentNodeId: 'folder-planning',
  nodeType: 'file',
  shortcutTargetNodeId: undefined,
  lifecycleStatus: 'active',
  version: '7',
  spaceType: 'personal',
  contentState: 'ready',
  fileExtension: 'docx',
  contentType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  contentTypeGroup: 'document',
  contentLength: '4096',
});
assert.deepEqual(planning?.activity, {
  kind: 'last-accessed',
  occurredAt: '2026-07-10T12:00:00.000Z',
  timestamp: Date.parse('2026-07-10T12:00:00.000Z'),
});
assert.deepEqual(planning?.openTarget, {
  kind: 'drive-node',
  appId: 'drive',
  resourceType: 'drive-node',
  resourceId: 'doc-planning',
  spaceId: 'space personal',
  section: 'recent',
  intent: 'preview',
});
assert.equal('url' in (planning?.openTarget ?? {}), false, 'untrusted response URLs must not become open targets');

assert.equal(recentDocuments.find((item) => item.id === 'doc-architecture')?.kind, 'pdf');
assert.equal(recentDocuments.find((item) => item.id === 'doc-metrics')?.kind, 'spreadsheet');
assert.equal(recentDocuments.find((item) => item.id === 'doc-metrics')?.type, 'excel');
assert.equal(recentDocuments.find((item) => item.id === 'doc-cover')?.kind, 'image');
assert.equal(recentDocuments.find((item) => item.id === 'folder-projects')?.kind, 'folder');
const shortcut = recentDocuments.find((item) => item.id === 'shortcut-roadmap');
assert.equal(shortcut?.kind, 'shortcut');
assert.equal(shortcut?.node?.shortcutTargetNodeId, 'doc-roadmap');
assert.equal(shortcut?.openTarget?.resourceId, 'shortcut-roadmap');
assert.deepEqual(shortcut?.activity, {
  kind: 'unknown',
  occurredAt: null,
  timestamp: null,
});
assert.equal(Number.isNaN(shortcut?.timestamp), true, 'missing or invalid activity time must not be replaced with the current clock');

const scopedCacheKey = recentDocumentsStorageKey('tenant-a', 'user-a');
const cachedRemotePage = JSON.parse(localStorage.getItem(scopedCacheKey) ?? 'null') as {
  schemaVersion?: number;
  items?: unknown[];
  pageInfo?: Record<string, unknown>;
};
assert.equal(cachedRemotePage.schemaVersion, 1);
assert.equal(cachedRemotePage.items?.length, driveNodes.length, 'remote success must populate the bounded offline cache');
assert.equal(cachedRemotePage.pageInfo?.totalItems, '6');
assert.equal(localStorage.getItem('sdkwork-im-pc:workspace-recent-docs'), null, 'recent documents must not use an unscoped storage key');

await service.savePinnedAppIds(['drive']);
const customizedApps = await service.getApps();
assert.equal(customizedApps.find((app) => app.id === 'notary')?.pinned, true, 'required app cannot be hidden');
assert.equal(customizedApps.find((app) => app.id === 'drive')?.pinned, true);
assert.equal(customizedApps.find((app) => app.id === 'knowledge')?.pinned, false);
assert.equal(
  localStorage.getItem('sdkwork-im-pc:workspace-pinned-apps:v1'),
  null,
  'workspace preferences must not use an unscoped storage key',
);

setSessionScope('tenant-a', 'user-b');
const otherUserApps = await service.getApps();
assert.ok(
  otherUserApps.every((app) => app.pinned),
  'a different user must not inherit another user\'s shortcut preferences',
);

setSessionScope('tenant-a', 'user-a');
const restoredUserApps = await service.getApps();
assert.equal(restoredUserApps.find((app) => app.id === 'knowledge')?.pinned, false);

const appsByType: AppItem[] = await service.searchApps('drive');
assert.deepEqual(appsByType.map((app) => app.id), ['drive']);

const fallbackService = createSdkworkWorkspaceService(
  () => ({
    portal: {
      home: {
        async retrieve() {
          throw new Error('offline');
        },
      },
    },
  }) as never,
  () => ({
    drive: {
      recent: {
        async list() {
          throw new Error('offline');
        },
      },
    },
  }) as never,
);
const fallbackWorkspaceData = await fallbackService.getWorkspaceData();
assert.equal(fallbackWorkspaceData.apps.source, 'offline');
assert.equal(fallbackWorkspaceData.documents.source, 'offline');
assert.deepEqual(
  fallbackWorkspaceData.apps.items.map((app) => app.id),
  ['notary'],
  'an unavailable portal catalog must not expose every commercial app',
);
assert.deepEqual(
  fallbackWorkspaceData.documents.items.map((item) => item.id),
  recentDocuments.map((item) => item.id),
  'offline reads must recover the last successful page for the same tenant and user',
);
assert.equal(fallbackWorkspaceData.documents.pageInfo?.totalItems, '6');

const permissionDeniedService = createSdkworkWorkspaceService(
  () => ({
    portal: {
      home: {
        async retrieve() {
          throw Object.assign(new Error('forbidden'), {
            code: 'FORBIDDEN',
            httpStatus: 403,
          });
        },
      },
    },
  }) as never,
  () => driveClient as never,
);
const permissionDeniedData = await permissionDeniedService.getWorkspaceData();
assert.equal(permissionDeniedData.apps.source, 'permission-denied');
assert.deepEqual(permissionDeniedData.apps.items.map((app) => app.id), ['notary']);

const unavailableService = createSdkworkWorkspaceService(
  () => ({
    portal: {
      home: {
        async retrieve() {
          throw Object.assign(new Error('server unavailable'), {
            code: 'SERVER_ERROR',
            httpStatus: 503,
          });
        },
      },
    },
  }) as never,
  () => driveClient as never,
);
assert.equal((await unavailableService.getWorkspaceData()).apps.source, 'unavailable');

const malformedResponseService = createSdkworkWorkspaceService(
  () => appClient as never,
  () => ({
    drive: {
      recent: {
        async list() {
          return {
            code: 0,
            data: createRecentPage([{ id: 'should-not-be-unwrapped-locally' }]),
            traceId: 'trace-raw-envelope',
          };
        },
      },
    },
  }) as never,
);
const malformedResponseData = await malformedResponseService.getWorkspaceData();
assert.equal(malformedResponseData.documents.source, 'unavailable');
assert.deepEqual(
  malformedResponseData.documents.items.map((item) => item.id),
  recentDocuments.map((item) => item.id),
  'a malformed or raw envelope must not be mistaken for a real empty page',
);

const emptyCatalogService = createSdkworkWorkspaceService(
  () => ({
    portal: {
      home: {
        async retrieve() {
          return { enabledModules: [] };
        },
      },
    },
  }) as never,
  () => driveClient as never,
);
assert.deepEqual(
  (await emptyCatalogService.getApps()).map((app) => app.id),
  ['notary'],
  'an explicitly empty catalog must not mean every app is enabled',
);

const emptyPageService = createSdkworkWorkspaceService(
  () => appClient as never,
  () => ({
    drive: {
      recent: {
        async list() {
          return createRecentPage([], {
            pageSize: 12,
            totalItems: '0',
          });
        },
      },
    },
  }) as never,
);
const emptyRemotePage = await emptyPageService.getWorkspaceData();
assert.equal(emptyRemotePage.documents.source, 'remote');
assert.deepEqual(emptyRemotePage.documents.items, []);
assert.equal(emptyRemotePage.documents.pageInfo?.totalItems, '0');
const offlineAfterEmpty = await fallbackService.getWorkspaceData();
assert.deepEqual(
  offlineAfterEmpty.documents.items,
  [],
  'a successful remote empty page must clear stale cached documents',
);
assert.equal(offlineAfterEmpty.documents.pageInfo?.totalItems, '0');

await service.getRecentDocuments();
setSessionScope('tenant-b', 'user-a');
const otherTenantFallback = await fallbackService.getRecentDocuments();
assert.deepEqual(
  otherTenantFallback,
  [],
  'recent document metadata must not cross tenant boundaries',
);

setSessionScope('tenant-a', 'user-a');
for (let index = 0; index < 25; index += 1) {
  const document: DocumentItem = {
    id: `local-doc-${index}`,
    name: `Local ${index}.txt`,
    nameKey: `Local ${index}.txt`,
    timestamp: Date.parse(`2026-07-${String((index % 9) + 1).padStart(2, '0')}T10:00:00.000Z`),
    type: 'text',
  };
  await service.addRecentDocument(document);
}
const boundedFallback = await fallbackService.getWorkspaceData();
assert.equal(boundedFallback.documents.items.length, 20, 'offline document cache must remain bounded');
assert.equal(boundedFallback.documents.items[0]?.id, 'local-doc-24');
assert.equal(boundedFallback.documents.pageInfo?.pageSize, 20);

console.log('sdkwork-im-pc workspace service contract passed');
