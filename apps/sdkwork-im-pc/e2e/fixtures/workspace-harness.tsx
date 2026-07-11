import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';

import { ModuleRenderHost } from '../../packages/sdkwork-im-pc-shell/src';
import {
  WorkspaceView,
} from '../../packages/sdkwork-im-pc-workspace/src';
import i18n from '../../packages/sdkwork-im-pc-workspace/src/i18n';
import type {
  AppItem,
  DocumentItem,
  WorkspaceData,
  WorkspaceService,
} from '../../packages/sdkwork-im-pc-workspace/src/services/WorkspaceService';
import '../../src/index.css';

declare global {
  interface Window {
    __workspaceHarnessLoadCalls: number;
    __workspaceHarnessLastAppId: string | null;
    __workspaceHarnessSavedPinnedIds: string[];
  }
}

const mode = new URLSearchParams(window.location.search).get('mode') ?? 'success';
const apps: AppItem[] = [
  {
    id: 'notary',
    nameKey: 'apps.notary',
    iconName: 'ShieldCheck',
    color: 'bg-indigo-500/20 text-indigo-400',
    pinned: true,
    required: true,
  },
  {
    id: 'drive',
    nameKey: 'apps.drive',
    iconName: 'Cloud',
    color: 'bg-cyan-500/20 text-cyan-400',
    pinned: true,
    required: false,
  },
  {
    id: 'knowledge',
    nameKey: 'apps.knowledge',
    iconName: 'BookOpen',
    color: 'bg-emerald-500/20 text-emerald-400',
    pinned: true,
    required: false,
  },
];
const documents: DocumentItem[] = [
  {
    id: 'doc-roadmap',
    name: 'Roadmap.docx',
    nameKey: 'Roadmap.docx',
    timestamp: Date.UTC(2026, 6, 10, 9, 30),
    type: 'word',
  },
  {
    id: 'doc-metrics',
    name: 'Metrics.xlsx',
    nameKey: 'Metrics.xlsx',
    timestamp: Date.UTC(2026, 6, 9, 16, 0),
    type: 'excel',
  },
  {
    id: 'doc-i18n-key-collision',
    name: 'loading',
    nameKey: 'loading',
    timestamp: Date.UTC(2026, 6, 8, 10, 0),
    type: 'unknown',
  },
];

window.__workspaceHarnessLoadCalls = 0;
window.__workspaceHarnessLastAppId = null;
window.__workspaceHarnessSavedPinnedIds = [];

function createWorkspaceData(): WorkspaceData {
  const source = mode === 'fallback'
    ? 'fallback'
    : mode === 'permission'
      ? 'permission-denied'
      : 'remote';
  return {
    apps: {
      items: mode === 'empty' ? [] : apps,
      source,
    },
    documents: {
      items: mode === 'empty' ? [] : documents,
      source,
    },
  };
}

function ThrowingModule(): never {
  throw new Error('sensitive-module-internal-detail');
}

function ModuleRecoveryHarness() {
  const [activeTab, setActiveTab] = useState('broken');
  return (
    <main className="flex h-screen min-h-0 bg-[#181818] text-white">
      <ModuleRenderHost
        activeTab={activeTab}
        capabilitySurface={activeTab === 'broken'
          ? <ThrowingModule />
          : <div role="status">Workspace recovered</div>}
        chatSurface={<div role="status">Chat recovered</div>}
        errorFallback={(
          <div role="alert">
            <h1>Module unavailable</h1>
            <button onClick={() => setActiveTab('workspace')} type="button">
              Return to workspace
            </button>
          </div>
        )}
      />
    </main>
  );
}

const service: WorkspaceService = {
  async getWorkspaceData() {
    window.__workspaceHarnessLoadCalls += 1;
    if (mode === 'retry' && window.__workspaceHarnessLoadCalls === 1) {
      throw new Error('initial load failed');
    }
    return createWorkspaceData();
  },
  async getApps() {
    return createWorkspaceData().apps.items;
  },
  async getRecentDocuments() {
    return createWorkspaceData().documents.items;
  },
  async searchApps(query) {
    const normalized = query.trim().toLowerCase();
    return apps.filter((app) => app.id.includes(normalized));
  },
  async savePinnedAppIds(ids) {
    window.__workspaceHarnessSavedPinnedIds = [...ids];
  },
  async addRecentDocument() {},
  async deleteRecentDocument() {},
  async addApp() {},
  async removeApp() {},
};

function WorkspaceHarness() {
  return (
    <main className="flex h-screen min-h-0 bg-[#181818] text-white">
      <WorkspaceView
        onAppSelect={(appId) => {
          window.__workspaceHarnessLastAppId = appId;
        }}
        service={service}
      />
    </main>
  );
}

function HarnessRoot() {
  return mode === 'module-error' ? <ModuleRecoveryHarness /> : <WorkspaceHarness />;
}

void i18n.changeLanguage('en-US').then(() => {
  const root = document.getElementById('root');
  if (!root) throw new Error('Workspace harness root is unavailable.');
  createRoot(root).render(<HarnessRoot />);
});
