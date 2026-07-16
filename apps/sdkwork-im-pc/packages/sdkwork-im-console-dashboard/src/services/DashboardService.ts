import { getAppSdkClientWithSession } from '@sdkwork/im-pc-core';

type AppSdkClient = ReturnType<typeof getAppSdkClientWithSession>;
type PortalDashboardSnapshot = Awaited<ReturnType<AppSdkClient['portal']['dashboard']['retrieve']>>;

export type DashboardMetricKey =
  | 'clientRouteWindows'
  | 'pendingRealtimeEvents'
  | 'persistedConversationSnapshots'
  | 'failedConversationSnapshotPersists'
  | 'projectionReplayBacklog'
  | 'projectionReplayedEvents';

export interface DashboardMetric {
  key: DashboardMetricKey;
  label: string;
  value: string;
}

export interface DashboardViewModel {
  state: PortalDashboardSnapshot['availability']['state'];
  source: string;
  complete: boolean;
  reason?: string;
  generatedAt: string;
  opsStatus: string;
  metrics: DashboardMetric[];
}

function formatInt64Count(value: string): string {
  if (!/^[0-9]+$/u.test(value)) {
    throw new Error('Portal dashboard returned an invalid int64 count.');
  }

  return value.replace(/\B(?=(\d{3})+(?!\d))/gu, ',');
}

function toDashboardView(snapshot: PortalDashboardSnapshot): DashboardViewModel {
  const metrics = snapshot.metrics;

  return {
    state: snapshot.availability.state,
    source: snapshot.availability.source,
    complete: snapshot.availability.complete,
    reason: snapshot.availability.reason,
    generatedAt: snapshot.meta.generatedAt,
    opsStatus: snapshot.meta.opsStatus,
    metrics: metrics ? [
      {
        key: 'clientRouteWindows',
        label: '客户端路由窗口',
        value: formatInt64Count(metrics.clientRouteWindowCount),
      },
      {
        key: 'pendingRealtimeEvents',
        label: '待投递实时事件',
        value: formatInt64Count(metrics.pendingRealtimeEventCount),
      },
      {
        key: 'persistedConversationSnapshots',
        label: '会话快照持久化成功',
        value: formatInt64Count(metrics.conversationSnapshotPersistSuccessCount),
      },
      {
        key: 'failedConversationSnapshotPersists',
        label: '会话快照持久化失败',
        value: formatInt64Count(metrics.conversationSnapshotPersistFailureCount),
      },
      {
        key: 'projectionReplayBacklog',
        label: '投影重放积压',
        value: formatInt64Count(metrics.projectionReplayBacklogSize),
      },
      {
        key: 'projectionReplayedEvents',
        label: '已重放投影事件',
        value: formatInt64Count(metrics.projectionReplayedEventCount),
      },
    ] : [],
  };
}

class DashboardService {
  async retrieve(): Promise<DashboardViewModel> {
    const snapshot = await getAppSdkClientWithSession().portal.dashboard.retrieve();
    return toDashboardView(snapshot);
  }
}

export const dashboardService = new DashboardService();
