import React, { useMemo } from 'react';
import { Check, Bot, Loader2 } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { AgentConfig } from '@sdkwork/agents-pc-agents';
import { isStandardAgentId } from '../services/AgentCatalogService';
import { mentionLabelForAgent } from '../services/AgentMentionService';

export interface AgentPickerPanelProps {
  agents: AgentConfig[];
  disabled?: boolean;
  selectedIds: Set<string>;
  onToggle: (agent: AgentConfig) => void;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  isLoading?: boolean;
  isLoadingMore?: boolean;
  hasMore?: boolean;
  onLoadMore?: () => void;
  maxSelected?: number;
  emptyText?: string;
  errorText?: string;
  onRetry?: () => void;
  retryText?: string;
}

function agentId(agent: AgentConfig): string {
  return typeof agent.id === 'string' ? agent.id.trim() : '';
}

function agentLabel(agent: AgentConfig): string {
  return agent.name.trim() || agentId(agent);
}

export const AgentPickerPanel: React.FC<AgentPickerPanelProps> = ({
  agents,
  disabled: pickerDisabled = false,
  selectedIds,
  onToggle,
  searchQuery,
  onSearchQueryChange,
  isLoading = false,
  isLoadingMore = false,
  hasMore = false,
  onLoadMore,
  maxSelected = 10,
  emptyText,
  errorText,
  onRetry,
  retryText,
}) => {
  const { t } = useTranslation();
  const mentionAgents = useMemo(() => {
    const seen = new Set<string>();
    return agents
      .map((agent) => {
        const id = agentId(agent);
        if (!isStandardAgentId(id) || seen.has(id)) {
          return undefined;
        }
        seen.add(id);
        return {
          agentId: id,
          name: agentLabel(agent),
        };
      })
      .filter((agent): agent is { agentId: string; name: string } => Boolean(agent));
  }, [agents]);
  const filteredAgents = useMemo(() => {
    const query = searchQuery.trim().toLocaleLowerCase();
    if (!query) {
      return agents;
    }
    return agents.filter((agent) => isStandardAgentId(agent.id) && [agent.name, agent.description, agent.id]
      .filter((value): value is string => typeof value === 'string')
      .some((value) => value.toLocaleLowerCase().includes(query)));
  }, [agents, searchQuery]);

  return (
    <section
      className="flex min-h-0 flex-1 flex-col border-l border-white/5 bg-[#1b1b1d]"
      aria-label={t('chat.agentPicker.title')}
    >
      <div className="shrink-0 border-b border-white/5 px-4 py-3">
        <div className="mb-2 flex items-center justify-between gap-2">
          <div>
            <h3 className="text-sm font-medium text-gray-200">{t('chat.agentPicker.title')}</h3>
            <p className="mt-0.5 text-[11px] text-gray-500">
              {t('chat.agentPicker.selectedCount', { count: selectedIds.size, max: maxSelected })}
            </p>
          </div>
          <Bot size={16} className="text-indigo-400" aria-hidden="true" />
        </div>
        <input
          type="search"
          value={searchQuery}
          onChange={(event) => onSearchQueryChange(event.target.value)}
          placeholder={t('chat.agentPicker.searchPlaceholder')}
          aria-label={t('chat.agentPicker.searchPlaceholder')}
          disabled={pickerDisabled}
          className="w-full rounded-lg border border-white/10 bg-[#121214] px-3 py-2 text-xs text-gray-200 outline-none transition-colors placeholder:text-gray-600 focus:border-indigo-500/60"
        />
      </div>
      <div className="min-h-0 flex-1 overflow-y-auto custom-scrollbar p-3">
        {isLoading ? (
          <div className="flex items-center justify-center gap-2 py-10 text-xs text-gray-500">
            <Loader2 size={14} className="animate-spin" />
            {t('chat.agentPicker.loading')}
          </div>
        ) : errorText ? (
          <div
            role="alert"
            className="rounded-lg border border-red-400/20 bg-red-500/10 px-3 py-6 text-center text-xs text-red-200"
          >
            <p>{errorText}</p>
            {onRetry && (
              <button
                type="button"
                onClick={onRetry}
                disabled={pickerDisabled}
                className="mt-3 rounded border border-red-300/30 px-3 py-1.5 text-[11px] text-red-100 transition-colors hover:bg-red-500/15 disabled:cursor-not-allowed disabled:opacity-50"
              >
                {retryText ?? t('chat.agentPicker.retry')}
              </button>
            )}
          </div>
        ) : filteredAgents.length === 0 ? (
          <div className="rounded-lg border border-dashed border-white/10 px-3 py-8 text-center text-xs text-gray-500">
            {emptyText ?? t('chat.agentPicker.empty')}
          </div>
        ) : (
          <div className="space-y-1">
            {filteredAgents.map((agent) => {
              const id = agentId(agent);
              if (!isStandardAgentId(id)) {
                return null;
              }
              const checked = selectedIds.has(id);
              const disabled = pickerDisabled || (!checked && selectedIds.size >= maxSelected);
              const displayLabel = mentionLabelForAgent(
                { agentId: id, name: agentLabel(agent) },
                mentionAgents,
              );
              return (
                <button
                  key={id}
                  type="button"
                  disabled={disabled}
                  onClick={() => onToggle(agent)}
                  className="flex w-full items-center gap-3 rounded-lg px-2.5 py-2 text-left transition-colors hover:bg-white/5 disabled:cursor-not-allowed disabled:opacity-45"
                  aria-pressed={checked}
                  title={disabled ? t('chat.agentPicker.maxReached', { max: maxSelected }) : undefined}
                >
                  <Avatar
                    src={agent.avatar}
                    alt={displayLabel}
                    className="h-8 w-8 shrink-0 rounded-lg bg-[#2b2b2d]"
                  />
                  <span className="min-w-0 flex-1">
                    <span className="block truncate text-xs font-medium text-gray-200">{displayLabel}</span>
                    <span className="block truncate text-[11px] text-gray-500">{agent.description || id}</span>
                  </span>
                  <span
                    className={`flex h-5 w-5 shrink-0 items-center justify-center rounded border ${checked ? 'border-indigo-400 bg-indigo-500 text-white' : 'border-white/20 text-transparent'}`}
                    aria-hidden="true"
                  >
                    <Check size={13} />
                  </span>
                </button>
              );
            })}
          </div>
        )}
        {hasMore && !isLoading && (
          <button
            type="button"
            onClick={onLoadMore}
            disabled={pickerDisabled || isLoadingMore}
            className="mt-3 flex w-full items-center justify-center gap-2 rounded-lg border border-white/10 px-3 py-2 text-xs text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-200 disabled:opacity-50"
          >
            {isLoadingMore && <Loader2 size={13} className="animate-spin" />}
            {isLoadingMore ? t('chat.agentPicker.loadingMore') : t('chat.agentPicker.loadMore')}
          </button>
        )}
      </div>
    </section>
  );
};
