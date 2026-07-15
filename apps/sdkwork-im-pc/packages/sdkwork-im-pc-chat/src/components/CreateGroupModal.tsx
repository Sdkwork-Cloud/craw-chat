import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { BookOpen, Bot, X } from 'lucide-react';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { Chat, User } from '@sdkwork/im-pc-types';
import type { AgentConfig } from '@sdkwork/agents-pc-agents';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import {
  createGroupClientRequestKey,
  groupService,
  MAX_GROUP_INITIAL_MEMBERS,
} from '../services/GroupService';
import { ContactMemberPickerPanel } from './ContactMemberPickerPanel';
import { ModalWrapper } from './ModalWrapper';
import { AgentPickerPanel } from './AgentPickerPanel';
import { listAvailableAgents } from '../services/AgentCatalogService';
import type { GroupAgentAssignment } from '../services/GroupService';

export const CreateGroupModal: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  onCreated?: (group: Chat) => void | Promise<void>;
}> = ({ isOpen, onClose, onCreated }) => {
  const { t } = useTranslation();
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = useState('');
  const [contacts, setContacts] = useState<User[]>([]);
  const [contactsCursor, setContactsCursor] = useState<string | undefined>();
  const [hasMoreContacts, setHasMoreContacts] = useState(false);
  const [loading, setLoading] = useState(false);
  const [loadingMoreContacts, setLoadingMoreContacts] = useState(false);
  const [creating, setCreating] = useState(false);
  const [groupName, setGroupName] = useState('');
  const [initializeKnowledgebase, setInitializeKnowledgebase] = useState(false);
  const [agents, setAgents] = useState<AgentConfig[]>([]);
  const [selectedAgentIds, setSelectedAgentIds] = useState<Set<string>>(new Set());
  const [selectedAgents, setSelectedAgents] = useState<Map<string, GroupAgentAssignment>>(new Map());
  const [agentsCursorPage, setAgentsCursorPage] = useState(1);
  const [hasMoreAgents, setHasMoreAgents] = useState(false);
  const [loadingAgents, setLoadingAgents] = useState(false);
  const [loadingMoreAgents, setLoadingMoreAgents] = useState(false);
  const [agentSearchQuery, setAgentSearchQuery] = useState('');
  const [agentLoadError, setAgentLoadError] = useState(false);
  const contactRequestSequenceRef = useRef(0);
  const agentRequestSequenceRef = useRef(0);
  const createAttemptRef = useRef<{ fingerprint: string; requestKey: string } | undefined>(undefined);
  const modalSessionRef = useRef(0);
  const previousOpenRef = useRef(false);
  const MAX_GROUP_AGENTS = 10;

  const closeModal = useCallback((): void => {
    modalSessionRef.current += 1;
    contactRequestSequenceRef.current += 1;
    agentRequestSequenceRef.current += 1;
    onClose();
  }, [onClose]);

  useEffect(() => {
    if (previousOpenRef.current !== isOpen) {
      modalSessionRef.current += 1;
      previousOpenRef.current = isOpen;
    }
    if (isOpen) {
      const requestId = ++contactRequestSequenceRef.current;
      setLoading(true);
      contactService.listContactsPage()
        .then((page) => {
          if (contactRequestSequenceRef.current !== requestId) {
            return;
          }
          setContacts(page.items);
          setContactsCursor(page.nextCursor);
          setHasMoreContacts(page.hasMore);
        })
        .catch(() => {
          if (contactRequestSequenceRef.current !== requestId) {
            return;
          }
          setContacts([]);
          setContactsCursor(undefined);
          setHasMoreContacts(false);
          toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
        })
        .finally(() => {
          if (contactRequestSequenceRef.current === requestId) {
            setLoading(false);
          }
        });
    } else {
      contactRequestSequenceRef.current += 1;
      agentRequestSequenceRef.current += 1;
      setContacts([]);
      setContactsCursor(undefined);
      setHasMoreContacts(false);
      setSelected(new Set());
      setSearchQuery('');
      setCreating(false);
      setGroupName('');
      setInitializeKnowledgebase(false);
      setLoadingMoreContacts(false);
      setAgents([]);
      setSelectedAgentIds(new Set());
      setSelectedAgents(new Map());
      setAgentsCursorPage(1);
      setHasMoreAgents(false);
      setLoadingAgents(false);
      setLoadingMoreAgents(false);
      setAgentSearchQuery('');
      setAgentLoadError(false);
      createAttemptRef.current = undefined;
    }
  }, [isOpen, t]);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }
    // Invalidate the previous query immediately; a stale response must not
    // update the list while the new query is waiting for its debounce timer.
    agentRequestSequenceRef.current += 1;
    setLoadingMoreAgents(false);
    const timer = window.setTimeout(() => {
      setLoadingAgents(true);
      setAgentLoadError(false);
      const requestId = ++agentRequestSequenceRef.current;
      void listAvailableAgents({ q: agentSearchQuery })
        .then((page) => {
          if (agentRequestSequenceRef.current !== requestId) {
            return;
          }
          setAgents(page.items);
          setAgentsCursorPage(page.page);
          setHasMoreAgents(page.hasMore);
        })
        .catch(() => {
          if (agentRequestSequenceRef.current === requestId) {
            setAgents([]);
            setHasMoreAgents(false);
            setAgentLoadError(true);
          }
        })
        .finally(() => {
          if (agentRequestSequenceRef.current === requestId) {
            setLoadingAgents(false);
          }
        });
    }, agentSearchQuery.trim() ? 250 : 0);
    return () => window.clearTimeout(timer);
  }, [agentSearchQuery, isOpen]);

  const loadMoreContacts = useCallback(() => {
    if (!hasMoreContacts || loadingMoreContacts) {
      return;
    }
    setLoadingMoreContacts(true);
    const requestId = ++contactRequestSequenceRef.current;
    void contactService.listContactsPage({ cursor: contactsCursor })
      .then((page) => {
        if (contactRequestSequenceRef.current !== requestId) {
          return;
        }
        setContacts((previousContacts) => [...previousContacts, ...page.items]);
        setContactsCursor(page.nextCursor);
        setHasMoreContacts(page.hasMore);
      })
      .catch(() => {
        if (contactRequestSequenceRef.current === requestId) {
          toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
        }
      })
      .finally(() => {
        if (contactRequestSequenceRef.current === requestId) {
          setLoadingMoreContacts(false);
        }
      });
  }, [contactsCursor, hasMoreContacts, loadingMoreContacts, t]);

  const toggleSelect = (id: string) => {
    setSelected((previousSelected) => {
      const nextSelected = new Set(previousSelected);
      if (nextSelected.has(id)) {
        nextSelected.delete(id);
      } else if (nextSelected.size < MAX_GROUP_INITIAL_MEMBERS) {
        nextSelected.add(id);
      }
      return nextSelected;
    });
  };

  const toggleAgent = (agent: AgentConfig): void => {
    const id = agent.id?.trim();
    if (!id) {
      return;
    }
    setSelectedAgentIds((previous) => {
      const next = new Set(previous);
      if (next.has(id)) {
        next.delete(id);
        setSelectedAgents((current) => {
          const copy = new Map(current);
          copy.delete(id);
          return copy;
        });
      } else if (next.size < MAX_GROUP_AGENTS) {
        next.add(id);
        setSelectedAgents((current) => new Map(current).set(id, {
          agentId: id,
          ...(agent.name ? { name: agent.name } : {}),
          ...(agent.avatar ? { avatar: agent.avatar } : {}),
        }));
      }
      return next;
    });
  };

  const removeSelectedAgent = (agentId: string): void => {
    setSelectedAgentIds((previous) => {
      if (!previous.has(agentId)) {
        return previous;
      }
      const next = new Set(previous);
      next.delete(agentId);
      return next;
    });
    setSelectedAgents((previous) => {
      if (!previous.has(agentId)) {
        return previous;
      }
      const next = new Map(previous);
      next.delete(agentId);
      return next;
    });
  };

  const loadMoreAgents = (): void => {
    if (!hasMoreAgents || loadingMoreAgents) {
      return;
    }
    setLoadingMoreAgents(true);
    setAgentLoadError(false);
    const requestId = ++agentRequestSequenceRef.current;
    void listAvailableAgents({ page: agentsCursorPage + 1, q: agentSearchQuery })
      .then((page) => {
        if (agentRequestSequenceRef.current !== requestId) {
          return;
        }
        setAgents((previous) => {
          const byId = new Map(previous.map((agent) => [agent.id, agent]));
          for (const agent of page.items) {
            if (agent.id && !byId.has(agent.id)) {
              byId.set(agent.id, agent);
            }
          }
          return [...byId.values()];
        });
        setAgentsCursorPage(page.page);
        setHasMoreAgents(page.hasMore);
      })
      .catch(() => {
        if (agentRequestSequenceRef.current === requestId) {
          setAgentLoadError(true);
          toast(t('chat.agentPicker.loadFailed'), 'error');
        }
      })
      .finally(() => {
        if (agentRequestSequenceRef.current === requestId) {
          setLoadingMoreAgents(false);
        }
      });
  };

  const handleCreate = async () => {
    if ((selected.size === 0 && selectedAgentIds.size === 0) || creating) {
      return;
    }

    const sessionId = modalSessionRef.current;
    setCreating(true);
    try {
      const shouldInitializeKnowledgebase = initializeKnowledgebase;
      const selectedCount = selected.size;
      const memberIds = [...selected].sort();
      const resolvedGroupName = groupName.trim() || t('chat.fallback.groupName');
      // Omit the optional field when no agent was explicitly selected so the
      // server can apply the configured default assignment. An empty array is
      // an explicit invalid replacement, not the default-selection signal.
      const assignments = selectedAgentIds.size > 0
        ? [...selectedAgentIds].map((id) => selectedAgents.get(id) ?? { agentId: id })
        : undefined;
      const fingerprint = JSON.stringify({
        agentAssignments: assignments?.map(({ agentId, revisionId }) => ({ agentId, revisionId })) ?? null,
        groupName: resolvedGroupName,
        initializeKnowledgebase: shouldInitializeKnowledgebase,
        memberIds,
      });
      if (createAttemptRef.current?.fingerprint !== fingerprint) {
        createAttemptRef.current = {
          fingerprint,
          requestKey: createGroupClientRequestKey(),
        };
      }
      const group = await groupService.createGroup(
        resolvedGroupName,
        memberIds,
        assignments,
        {
          clientRequestKey: createAttemptRef.current.requestKey,
          initializeKnowledgebase: shouldInitializeKnowledgebase,
        },
      );
      if (modalSessionRef.current !== sessionId) {
        return;
      }
      let openFailed = false;
      try {
        await onCreated?.(group);
      } catch {
        openFailed = true;
      }
      if (modalSessionRef.current !== sessionId) {
        return;
      }
      createAttemptRef.current = undefined;
      const knowledgebaseInitializationToast = shouldInitializeKnowledgebase
        ? group.knowledgebaseInitialization === 'active'
          ? { message: t('chat.modal.toast.groupAndKnowledgebaseCreated'), type: 'success' as const }
          : group.knowledgebaseInitialization === 'provisioning'
            ? { message: t('chat.modal.toast.groupCreatedKnowledgebaseProvisioning'), type: 'info' as const }
            : { message: t('chat.modal.toast.groupCreatedKnowledgebaseFailed'), type: 'error' as const }
        : undefined;
      toast(
        openFailed
          ? t('chat.messageList.toast.openGroupFailed')
          : knowledgebaseInitializationToast?.message
            ?? t('chat.agentPicker.groupCreated', { memberCount: selectedCount, agentCount: selectedAgentIds.size }),
        openFailed ? 'error' : knowledgebaseInitializationToast?.type ?? 'success',
      );
      closeModal();
    } catch {
      if (modalSessionRef.current === sessionId) {
        toast(t('chat.modal.toast.createGroupFailed'), 'error');
      }
    } finally {
      if (modalSessionRef.current === sessionId) {
        setCreating(false);
      }
    }
  };

  const retryAgentCatalog = useCallback((): void => {
    if (!isOpen || loadingAgents) {
      return;
    }
    setLoadingAgents(true);
    setLoadingMoreAgents(false);
    setAgentLoadError(false);
    setAgents([]);
    setHasMoreAgents(false);
    const requestId = ++agentRequestSequenceRef.current;
    void listAvailableAgents({ page: 1, q: agentSearchQuery })
      .then((page) => {
        if (agentRequestSequenceRef.current !== requestId) {
          return;
        }
        setAgents(page.items);
        setAgentsCursorPage(page.page);
        setHasMoreAgents(page.hasMore);
      })
      .catch(() => {
        if (agentRequestSequenceRef.current === requestId) {
          setAgents([]);
          setHasMoreAgents(false);
          setAgentLoadError(true);
        }
      })
      .finally(() => {
        if (agentRequestSequenceRef.current === requestId) {
          setLoadingAgents(false);
        }
      });
  }, [agentSearchQuery, isOpen, loadingAgents]);

  const selectedAgentList = [...selectedAgentIds].map((agentId) => (
    selectedAgents.get(agentId) ?? { agentId }
  ));
  return (
    <ModalWrapper
      isOpen={isOpen}
      onClose={closeModal}
      title={t('chat.modal.title.createGroup')}
      width="w-[1040px]"
      height="h-[740px]"
      footer={
        <>
          <button onClick={closeModal} className="rounded bg-white/5 px-4 py-2 text-sm text-gray-300 transition-colors hover:bg-white/10">
            {t('chat.modal.actions.cancel')}
          </button>
          <button
            disabled={(selected.size === 0 && selectedAgentIds.size === 0) || creating}
            className="rounded bg-[#00b42a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#009a24] disabled:cursor-not-allowed disabled:bg-[#00b42a]/50"
            onClick={() => void handleCreate()}
          >
            {creating
              ? t('chat.modal.actions.creating')
              : t('chat.modal.actions.createWithCount', { count: selected.size + selectedAgentIds.size })}
          </button>
        </>
      }
    >
      <div className="flex h-full min-h-0 flex-col">
        <div className="grid shrink-0 grid-cols-[minmax(0,1fr)_auto] gap-4 border-b border-white/5 bg-[#242426] px-4 py-3">
          <label className="min-w-0">
            <span className="mb-1.5 block text-xs font-medium text-gray-300">
              {t('chat.rightPanel.fields.groupName')}
            </span>
            <input
              type="text"
              value={groupName}
              onChange={(event) => setGroupName(event.target.value)}
              placeholder={t('chat.modal.placeholder.groupName')}
              aria-label={t('chat.rightPanel.fields.groupName')}
              disabled={creating}
              maxLength={256}
              className="h-9 w-full rounded-md border border-white/10 bg-[#171719] px-3 text-sm text-gray-100 outline-none transition-colors placeholder:text-gray-600 focus:border-[#00b42a]/70 disabled:cursor-not-allowed disabled:opacity-60"
            />
          </label>
          <div className="max-w-[280px] self-end">
            <label
              className="flex h-9 cursor-pointer items-center gap-2 rounded-md border border-white/10 bg-[#1b1b1d] px-3 text-xs text-gray-300 transition-colors hover:border-[#00b42a]/40 hover:bg-white/[0.03] has-[:checked]:border-[#00b42a]/55 has-[:checked]:bg-[#00b42a]/10 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-60"
            >
              <input
                type="checkbox"
                checked={initializeKnowledgebase}
                onChange={(event) => setInitializeKnowledgebase(event.target.checked)}
                disabled={creating}
                className="h-4 w-4 accent-[#00b42a]"
              />
              <BookOpen size={15} className="shrink-0 text-[#6be38b]" aria-hidden="true" />
              <span>{t('chat.header.knowledgebaseInitialize')}</span>
            </label>
          </div>
        </div>
        <div className="flex min-h-0 flex-1 gap-4">
          <div className="min-h-0 min-w-0 flex-[1.15]">
            <ContactMemberPickerPanel
              contacts={contacts}
              disabled={creating}
              emptyText={t('chat.modal.state.noContactsToCreate')}
              hasMoreContacts={hasMoreContacts}
              isLoading={loading}
              isLoadingMoreContacts={loadingMoreContacts}
              maxSelectable={MAX_GROUP_INITIAL_MEMBERS}
              onLoadMoreContacts={loadMoreContacts}
              searchPlaceholder={t('chat.modal.placeholder.memberSearch')}
              searchQuery={searchQuery}
              onSearchQueryChange={setSearchQuery}
              selectedIds={selected}
              onToggleContact={toggleSelect}
            />
          </div>
          <div className="flex min-h-0 min-w-0 flex-1 overflow-hidden rounded-lg border border-white/5 bg-[#1b1b1d] [&>section]:min-w-0">
            <AgentPickerPanel
              agents={agents}
              disabled={creating}
              selectedIds={selectedAgentIds}
              onToggle={toggleAgent}
              searchQuery={agentSearchQuery}
              onSearchQueryChange={setAgentSearchQuery}
              isLoading={loadingAgents}
              isLoadingMore={loadingMoreAgents}
              hasMore={hasMoreAgents}
              onLoadMore={loadMoreAgents}
              maxSelected={MAX_GROUP_AGENTS}
              emptyText={t('chat.agentPicker.empty')}
              errorText={agentLoadError && agents.length === 0 ? t('chat.agentPicker.loadFailed') : undefined}
              onRetry={retryAgentCatalog}
              retryText={t('chat.agentPicker.retry')}
            />
            {selectedAgentList.length > 0 && (
              <aside
                aria-label={t('chat.agentPicker.selectedTitle')}
                className="flex w-[200px] shrink-0 flex-col border-l border-white/5 bg-[#171719] p-4 max-[1100px]:hidden"
              >
                <div className="mb-3 flex items-center justify-between gap-2">
                  <div className="flex min-w-0 items-center gap-2">
                    <Bot size={15} className="shrink-0 text-indigo-400" aria-hidden="true" />
                    <span className="truncate text-xs font-medium text-gray-300">
                      {t('chat.agentPicker.selectedTitle')}
                    </span>
                  </div>
                  <span className="shrink-0 text-[11px] text-gray-500">
                    {selectedAgentIds.size}/{MAX_GROUP_AGENTS}
                  </span>
                </div>
                <div className="min-h-0 flex-1 space-y-1 overflow-y-auto custom-scrollbar">
                  {selectedAgentList.map((agent) => {
                    const label = agent.name?.trim() || agent.agentId;
                    return (
                      <div key={agent.agentId} className="flex items-center gap-2 rounded-md bg-white/[0.035] px-2 py-2">
                        <Avatar
                          src={agent.avatar}
                          alt={label}
                          className="h-7 w-7 shrink-0 rounded-md bg-[#2b2b2d]"
                        />
                        <span className="min-w-0 flex-1 truncate text-xs text-gray-300" title={label}>
                          {label}
                        </span>
                        <button
                          type="button"
                          onClick={() => removeSelectedAgent(agent.agentId)}
                          disabled={creating}
                          aria-label={t('chat.agentPicker.remove', { name: label })}
                          title={t('chat.agentPicker.remove', { name: label })}
                          className="flex h-6 w-6 shrink-0 items-center justify-center rounded text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-100 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                          <X size={13} />
                        </button>
                      </div>
                    );
                  })}
                </div>
              </aside>
            )}
          </div>
        </div>
      </div>
    </ModalWrapper>
  );
};
