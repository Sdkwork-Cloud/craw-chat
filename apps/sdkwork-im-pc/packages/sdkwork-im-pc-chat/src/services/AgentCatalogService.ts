import {
  agentService,
  type AgentConfig,
} from '@sdkwork/agents-pc-agents';

export interface AgentCatalogPage {
  items: AgentConfig[];
  page: number;
  hasMore: boolean;
}

const CATALOG_PAGE_SIZE = 50;

export function isStandardAgentId(value: unknown): value is string {
  return typeof value === 'string'
    && /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(value.trim());
}

/**
 * The IM client can invite agents from both the private workspace catalog and
 * the published market. The two scopes are merged client-side only for this
 * bounded picker page; assignment authorization remains server-side.
 */
export async function listAvailableAgents(params: {
  page?: number;
  q?: string;
} = {}): Promise<AgentCatalogPage> {
  const page = Number.isSafeInteger(params.page) && (params.page ?? 1) > 0 ? params.page ?? 1 : 1;
  const query = params.q?.trim();
  const [mine, market] = await Promise.all([
    agentService.listAgentsPage({
      page,
      pageSize: CATALOG_PAGE_SIZE,
      scope: 'mine',
      ...(query ? { q: query } : {}),
    }),
    agentService.listAgentsPage({
      page,
      pageSize: CATALOG_PAGE_SIZE,
      scope: 'market',
      ...(query ? { q: query } : {}),
    }),
  ]);
  const byId = new Map<string, AgentConfig>();
  for (const agent of [...mine.items, ...market.items]) {
    const id = typeof agent.id === 'string' ? agent.id.trim() : '';
    if (isStandardAgentId(id) && !byId.has(id)) {
      byId.set(id, agent);
    }
  }
  return {
    items: [...byId.values()],
    page,
    hasMore: mine.pageInfo.hasMore || market.pageInfo.hasMore,
  };
}
