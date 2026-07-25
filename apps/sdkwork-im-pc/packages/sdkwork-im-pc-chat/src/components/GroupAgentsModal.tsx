import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bot, Loader2, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import type { ChatAgentAssignment } from '@sdkwork/im-pc-types';
import type { AgentConfig } from '@sdkwork/agents-pc-agents';
import { ModalWrapper } from './ModalWrapper';
import { AgentPickerPanel } from './AgentPickerPanel';
import { toast } from './Toast';
import { groupService, type GroupAgentAssignment } from '../services/GroupService';
import { listAvailableAgents } from '../services/AgentCatalogService';
import { isStandardAgentId } from '../services/AgentCatalogService';
import { mentionLabelForAgent } from '../services/AgentMentionService';

export interface GroupAgentsModalProps {
  chat: { id: string; agentAssignments?: ChatAgentAssignment[]; agentAssignmentGeneration?: number } | null;
  isOpen: boolean;
  canManageAgents?: boolean;
  onClose: () => void;
  onSaved?: (assignments: GroupAgentAssignment[], generation: number) => void | Promise<void>;
}

const MAX_GROUP_AGENTS = 10;

function assignmentId(assignment: ChatAgentAssignment): string {
  return assignment.agentId.trim();
}

function toAssignment(agent: AgentConfig): GroupAgentAssignment {
  return {
    agentId: agent.id?.trim() ?? '',
    ...(agent.avatar ? { avatar: agent.avatar } : {}),
    ...(agent.name ? { name: agent.name } : {}),
  };
}

function readErrorStatus(error: unknown): number | undefined {
  if (!error || typeof error !== 'object') {
    return undefined;
  }
  const record = error as Record<string, unknown>;
  for (const candidate of [record.httpStatus, record.status, record.statusCode]) {
    if (typeof candidate === 'number') {
      return candidate;
    }
  }
  for (const nested of [record.response, record.raw, record.problem, record.details]) {
    if (nested && typeof nested === 'object') {
      const nestedRecord = nested as Record<string, unknown>;
      for (const candidate of [nestedRecord.httpStatus, nestedRecord.status, nestedRecord.statusCode]) {
        if (typeof candidate === 'number') {
          return candidate;
        }
      }
    }
  }
  return undefined;
}

function isPermissionDenied(error: unknown): boolean {
  if (readErrorStatus(error) === 403 || readErrorStatus(error) === 401) {
    return true;
  }
  const record = error && typeof error === 'object' ? error as Record<string, unknown> : undefined;
  const code = typeof record?.code === 'string' ? record.code : '';
  const message = error instanceof Error ? error.message : String(error ?? '');
  return /\b(?:401|403)\b|forbidden|unauthori[sz]ed/iu.test(`${code} ${message}`);
}

function isGenerationConflict(error: unknown): boolean {
  if (readErrorStatus(error) === 409) {
    return true;
  }
  const message = error instanceof Error ? error.message : String(error ?? '');
  return /generation|conflict|changed|\b409\b/iu.test(message);
}

export const GroupAgentsModal: React.FC<GroupAgentsModalProps> = ({
  chat,
  isOpen,
  canManageAgents = false,
  onClose,
  onSaved,
}) => {
  const { t } = useTranslation();
  const [agents, setAgents] = useState<AgentConfig[]>([]);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [selectedMetadata, setSelectedMetadata] = useState<Map<string, GroupAgentAssignment>>(new Map());
  const [generation, setGeneration] = useState<number | undefined>();
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [loadingMore, setLoadingMore] = useState(false);
  const [saving, setSaving] = useState(false);
  const [loadError, setLoadError] = useState(false);
  const [assignmentReady, setAssignmentReady] = useState(false);
  const [assignmentLoadError, setAssignmentLoadError] = useState(false);
  const requestSequenceRef = useRef(0);
  const assignmentRequestSequenceRef = useRef(0);
  const openedChatIdRef = useRef<string | undefined>(undefined);
  const modalSessionRef = useRef(0);

  const closeModal = useCallback((): void => {
    modalSessionRef.current += 1;
    requestSequenceRef.current += 1;
    assignmentRequestSequenceRef.current += 1;
    openedChatIdRef.current = undefined;
    onClose();
  }, [onClose]);

  const applyAssignmentSnapshot = useCallback((
    assignments: readonly GroupAgentAssignment[],
    nextGeneration: number,
  ): void => {
    const normalized = assignments.filter((assignment) => isStandardAgentId(assignment.agentId.trim()));
    setSelectedIds(new Set(normalized.map((assignment) => assignment.agentId.trim())));
    setSelectedMetadata((previous) => new Map(normalized.map((assignment) => {
      const agentId = assignment.agentId.trim();
      return [agentId, {
        ...previous.get(agentId),
        ...assignment,
        agentId,
      }];
    })));
    setGeneration(nextGeneration);
  }, []);

  const loadAssignmentSnapshot = useCallback((chatId: string): void => {
    setAssignmentReady(false);
    setAssignmentLoadError(false);
    const requestId = ++assignmentRequestSequenceRef.current;
    void groupService.getAgentAssignments(chatId)
      .then((latest) => {
        if (
          assignmentRequestSequenceRef.current === requestId
          && openedChatIdRef.current === chatId
        ) {
          applyAssignmentSnapshot(latest.agents, latest.generation);
          setAssignmentReady(true);
        }
      })
      .catch(() => {
        if (
          assignmentRequestSequenceRef.current === requestId
          && openedChatIdRef.current === chatId
        ) {
          setAssignmentReady(false);
          setAssignmentLoadError(true);
        }
      });
  }, [applyAssignmentSnapshot]);

  const load = useCallback(async (nextPage: number, append: boolean, query: string) => {
    const requestId = ++requestSequenceRef.current;
    if (append) {
      setLoadingMore(true);
    } else {
      setLoading(true);
      setLoadError(false);
    }
    try {
      const result = await listAvailableAgents({ page: nextPage, q: query });
      if (requestSequenceRef.current !== requestId) {
        return;
      }
      setAgents((previous) => {
        const next = append ? [...previous, ...result.items] : result.items;
        const byId = new Map<string, AgentConfig>();
        for (const item of next) {
          if (item.id && !byId.has(item.id)) {
            byId.set(item.id, item);
          }
        }
        return [...byId.values()];
      });
      setPage(result.page);
      setHasMore(result.hasMore);
    } catch {
      if (requestSequenceRef.current === requestId) {
        setLoadError(true);
        if (!append) {
          setAgents([]);
        }
      }
    } finally {
      if (requestSequenceRef.current === requestId) {
        setLoading(false);
        setLoadingMore(false);
      }
    }
  }, []);

  useEffect(() => {
    if (!isOpen || !chat) {
      if (!isOpen) {
        modalSessionRef.current += 1;
        requestSequenceRef.current += 1;
        assignmentRequestSequenceRef.current += 1;
        openedChatIdRef.current = undefined;
        setAgents([]);
        setSelectedIds(new Set());
        setSelectedMetadata(new Map());
        setSearchQuery('');
        setGeneration(undefined);
        setPage(1);
        setHasMore(false);
        setSaving(false);
        setLoadError(false);
        setAssignmentReady(false);
        setAssignmentLoadError(false);
      }
      return;
    }
    if (!canManageAgents) {
      closeModal();
      return;
    }
    if (openedChatIdRef.current === chat.id) {
      return;
    }
    modalSessionRef.current += 1;
    openedChatIdRef.current = chat.id;
    const existing = (chat.agentAssignments ?? []).filter((item) => isStandardAgentId(assignmentId(item)));
    setSelectedIds(new Set(existing.map(assignmentId)));
    setSelectedMetadata(new Map(existing.map((item) => [assignmentId(item), {
      agentId: assignmentId(item),
      ...(item.revisionId ? { revisionId: item.revisionId } : {}),
      ...(item.name ? { name: item.name } : {}),
      ...(item.avatar ? { avatar: item.avatar } : {}),
    }])));
    const currentGeneration = chat.agentAssignmentGeneration;
    setGeneration(
      Number.isSafeInteger(currentGeneration) && (currentGeneration ?? 0) >= 1
        ? currentGeneration
        : undefined,
    );

    loadAssignmentSnapshot(chat.id);
  }, [canManageAgents, chat, closeModal, isOpen, loadAssignmentSnapshot]);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }
    // Invalidate an in-flight request as soon as the query changes. This
    // prevents the previous result from winning during the debounce window.
    requestSequenceRef.current += 1;
    setLoadingMore(false);
    const timer = window.setTimeout(() => {
      void load(1, false, searchQuery);
    }, 250);
    return () => window.clearTimeout(timer);
  }, [isOpen, load, searchQuery]);

  const toggleAgent = (agent: AgentConfig): void => {
    const id = agent.id?.trim();
    if (!id) {
      return;
    }
    setSelectedIds((previous) => {
      const next = new Set(previous);
      if (next.has(id)) {
        next.delete(id);
        setSelectedMetadata((metadata) => {
          const copy = new Map(metadata);
          copy.delete(id);
          return copy;
        });
      } else if (next.size < MAX_GROUP_AGENTS) {
        next.add(id);
        setSelectedMetadata((metadata) => new Map(metadata).set(id, toAssignment(agent)));
      }
      return next;
    });
  };

  const selectedAssignmentList = useMemo(() => (
    [...selectedIds].map((id) => selectedMetadata.get(id) ?? { agentId: id })
  ), [selectedIds, selectedMetadata]);
  const selectedLabels = useMemo(() => new Map(
    selectedAssignmentList.map((assignment) => [
      assignment.agentId,
      mentionLabelForAgent(assignment, selectedAssignmentList),
    ]),
  ), [selectedAssignmentList]);

  const removeAssignment = (agentId: string): void => {
    setSelectedIds((previous) => {
      const next = new Set(previous);
      next.delete(agentId);
      return next;
    });
    setSelectedMetadata((previous) => {
      const next = new Map(previous);
      next.delete(agentId);
      return next;
    });
  };

  const save = async (): Promise<void> => {
    if (!chat || saving || !canManageAgents) {
      return;
    }
    if (!assignmentReady || typeof generation !== 'number' || !Number.isSafeInteger(generation) || generation < 1) {
      toast(t('chat.agentPicker.loadFailed'), 'error');
      return;
    }
    if (selectedIds.size < 1 || selectedIds.size > MAX_GROUP_AGENTS) {
      toast(t('chat.agentPicker.invalidCount', { min: 1, max: MAX_GROUP_AGENTS }), 'error');
      return;
    }
    const sessionId = modalSessionRef.current;
    const chatId = chat.id;
    setSaving(true);
    try {
      const stillAllowed = await groupService.canManageAgents(chatId);
      if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
        return;
      }
      if (!stillAllowed) {
        toast(t('chat.agentPicker.permissionDenied'), 'error');
        closeModal();
        return;
      }
      const assignments = [...selectedIds].map((id) => selectedMetadata.get(id) ?? { agentId: id });
      const result = await groupService.replaceAgentAssignments(chatId, generation, assignments);
      if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
        return;
      }
      setGeneration(result.generation);
      await onSaved?.(result.agents, result.generation);
      if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
        return;
      }
      toast(t('chat.agentPicker.saved'), 'success');
      closeModal();
    } catch (error) {
      if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
        return;
      }
      const isConflict = isGenerationConflict(error);
      if (isPermissionDenied(error)) {
        toast(t('chat.agentPicker.permissionDenied'), 'error');
        closeModal();
        return;
      }
      if (isConflict) {
        setAssignmentReady(false);
        setAssignmentLoadError(false);
        try {
          const latest = await groupService.getAgentAssignments(chatId);
          if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
            return;
          }
          applyAssignmentSnapshot(latest.agents, latest.generation);
          setAssignmentReady(true);
        } catch {
          if (modalSessionRef.current !== sessionId || openedChatIdRef.current !== chatId) {
            return;
          }
          setGeneration(undefined);
          setAssignmentReady(false);
          setAssignmentLoadError(true);
        }
      }
      const message = isConflict ? t('chat.agentPicker.conflict') : t('chat.agentPicker.saveFailed');
      toast(message, 'error');
    } finally {
      if (modalSessionRef.current === sessionId && openedChatIdRef.current === chatId) {
        setSaving(false);
      }
    }
  };

  return (
    <ModalWrapper
      isOpen={isOpen}
      onClose={closeModal}
      title={t('chat.agentPicker.manageTitle')}
      width="w-[820px]"
      height="h-[650px]"
      footer={(
        <>
          <button type="button" onClick={closeModal} disabled={saving} className="rounded bg-white/5 px-4 py-2 text-sm text-gray-300 transition-colors hover:bg-white/10 disabled:opacity-50">
            {t('chat.modal.actions.cancel')}
          </button>
          <button type="button" onClick={() => void save()} disabled={saving || !canManageAgents || !assignmentReady || generation === undefined || selectedIds.size < 1 || selectedIds.size > MAX_GROUP_AGENTS} className="flex items-center gap-2 rounded bg-indigo-600 px-4 py-2 text-sm text-white transition-colors hover:bg-indigo-500 disabled:cursor-not-allowed disabled:opacity-50">
            {saving && <Loader2 size={14} className="animate-spin" />}
            {saving ? t('chat.agentPicker.saving') : t('chat.agentPicker.save')}
          </button>
        </>
      )}
    >
      <div className="flex h-full min-h-0">
        <AgentPickerPanel
          agents={agents}
          disabled={!canManageAgents || !assignmentReady || saving}
          selectedIds={selectedIds}
          onToggle={toggleAgent}
          searchQuery={searchQuery}
          onSearchQueryChange={setSearchQuery}
          isLoading={loading}
          isLoadingMore={loadingMore}
          hasMore={hasMore}
          onLoadMore={() => void load(page + 1, true, searchQuery)}
          maxSelected={MAX_GROUP_AGENTS}
          emptyText={t('chat.agentPicker.empty')}
          errorText={loadError && agents.length === 0 ? t('chat.agentPicker.loadFailed') : undefined}
          onRetry={() => void load(1, false, searchQuery)}
          retryText={t('chat.agentPicker.retry')}
        />
        <aside className="flex w-[250px] shrink-0 flex-col border-l border-white/5 bg-[#171719] p-4">
          <div className="mb-3 flex items-center justify-between">
            <span className="text-xs font-medium text-gray-300">{t('chat.agentPicker.selectedTitle')}</span>
            <span className="text-[11px] text-gray-500">{selectedIds.size}/{MAX_GROUP_AGENTS}</span>
          </div>
          <div className="min-h-0 flex-1 space-y-1 overflow-y-auto custom-scrollbar">
            {!assignmentReady && !assignmentLoadError && (
              <div className="flex justify-center py-4" aria-label={t('chat.agentPicker.loading')}>
                <Loader2 size={16} className="animate-spin text-indigo-400" />
              </div>
            )}
            {assignmentLoadError && (
              <div className="rounded bg-red-500/10 px-2 py-3 text-center text-xs text-red-300">
                <p>{t('chat.agentPicker.loadFailed')}</p>
                <button
                  type="button"
                  onClick={() => chat && loadAssignmentSnapshot(chat.id)}
                  className="mt-2 rounded border border-red-400/30 px-2 py-1 text-[11px] text-red-200 hover:bg-red-500/10"
                >
                  {t('chat.agentPicker.retry')}
                </button>
              </div>
            )}
            {selectedAssignmentList.map((assignment) => (
              <div key={assignment.agentId} className="flex items-center gap-2 rounded-lg bg-white/[0.03] px-2 py-2">
                <Bot size={14} className="shrink-0 text-indigo-400" />
                <span className="min-w-0 flex-1 truncate text-xs text-gray-300" title={assignment.agentId}>
                  {selectedLabels.get(assignment.agentId) ?? assignment.name ?? assignment.agentId}
                </span>
                <button type="button" disabled={!canManageAgents || !assignmentReady || saving} onClick={() => removeAssignment(assignment.agentId)} className="flex h-6 w-6 items-center justify-center rounded text-gray-500 hover:bg-white/10 hover:text-gray-200 disabled:cursor-not-allowed disabled:opacity-50" aria-label={t('chat.agentPicker.remove', { name: selectedLabels.get(assignment.agentId) ?? assignment.name ?? assignment.agentId })}>
                  <X size={13} />
                </button>
              </div>
            ))}
            {selectedIds.size === 0 && <p className="py-6 text-center text-xs text-gray-600">{t('chat.agentPicker.noneSelected')}</p>}
          </div>
          <p className="mt-3 text-[11px] leading-relaxed text-gray-600">{t('chat.agentPicker.mentionHint')}</p>
        </aside>
      </div>
    </ModalWrapper>
  );
};
