import { getAppSdkClientWithSession } from '@sdkwork/im-pc-core';

type AppSdkClient = ReturnType<typeof getAppSdkClientWithSession>;
type PortalDashboardSnapshot = Awaited<ReturnType<AppSdkClient['portal']['dashboard']['retrieve']>>;
type PortalConversationSnapshot = Awaited<ReturnType<AppSdkClient['portal']['conversationSnapshot']['retrieve']>>;

export type DashboardMetricKey =
  | 'clientRouteWindows'
  | 'pendingRealtimeEvents'
  | 'laggingConversationScopes'
  | 'maxConversationLag'
  | 'pendingOutboxEvents'
  | 'failedOutboxAttempts';

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

function resolveState(
  dashboard: PortalDashboardSnapshot,
  conversations: PortalConversationSnapshot,
): DashboardViewModel['state'] {
  const states = [dashboard.availability.state, conversations.availability.state];
  if (states.every((state) => state === 'unavailable')) {
    return 'unavailable';
  }
  if (states.some((state) => state !== 'available')) {
    return 'partial';
  }
  return 'available';
}

function toDashboardView(
  dashboard: PortalDashboardSnapshot,
  conversations: PortalConversationSnapshot,
): DashboardViewModel {
  const realtimeMetrics = dashboard.metrics;
  const conversationMetrics = conversations.metrics;
  const reasons = [dashboard.availability.reason, conversations.availability.reason]
    .filter((reason): reason is string => Boolean(reason));
  const sources = [...new Set([dashboard.availability.source, conversations.availability.source])];
  const opsStatuses = [...new Set([dashboard.meta.opsStatus, conversations.meta.opsStatus])];
  const metrics: DashboardMetric[] = [];

  if (realtimeMetrics) {
    metrics.push(
      {
        key: 'clientRouteWindows',
        label: '客户端路由窗口',
        value: formatInt64Count(realtimeMetrics.clientRouteWindowCount),
      },
      {
        key: 'pendingRealtimeEvents',
        label: '待投递实时事件',
        value: formatInt64Count(realtimeMetrics.pendingRealtimeEventCount),
      },
    );
  }

  if (conversationMetrics) {
    metrics.push(
      {
        key: 'laggingConversationScopes',
        label: '存在延迟的会话范围',
        value: formatInt64Count(conversationMetrics.laggingScopeCount),
      },
      {
        key: 'maxConversationLag',
        label: '最大运行延迟',
        value: formatInt64Count(conversationMetrics.maxOperationalLag),
      },
      {
        key: 'pendingOutboxEvents',
        label: '待投递事务事件',
        value: formatInt64Count(conversationMetrics.pendingOutboxEventCount),
      },
      {
        key: 'failedOutboxAttempts',
        label: '事务事件失败尝试',
        value: formatInt64Count(conversationMetrics.failedOutboxAttemptCount),
      },
    );
  }

  return {
    state: resolveState(dashboard, conversations),
    source: sources.join(', '),
    complete: dashboard.availability.complete && conversations.availability.complete,
    reason: reasons.length > 0 ? reasons.join('; ') : undefined,
    generatedAt: dashboard.meta.generatedAt,
    opsStatus: opsStatuses.join(', '),
    metrics,
  };
}

class DashboardService {
  async retrieve(): Promise<DashboardViewModel> {
    const client = getAppSdkClientWithSession();
    const [dashboard, conversations] = await Promise.all([
      client.portal.dashboard.retrieve(),
      client.portal.conversationSnapshot.retrieve(),
    ]);
    return toDashboardView(dashboard, conversations);
  }
}

export const dashboardService = new DashboardService();
