import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
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
      const selectedCount = selected.size;
      const memberIds = [...selected].sort();
      // Omit the optional field when no agent was explicitly selected so the
      // server can apply the configured default assignment. An empty array is
      // an explicit invalid replacement, not the default-selection signal.
      const assignments = selectedAgentIds.size > 0
        ? [...selectedAgentIds].map((id) => selectedAgents.get(id) ?? { agentId: id })
        : undefined;
      const fingerprint = JSON.stringify({
        agentAssignments: assignments?.map(({ agentId, revisionId }) => ({ agentId, revisionId })) ?? null,
        memberIds,
      });
      if (createAttemptRef.current?.fingerprint !== fingerprint) {
        createAttemptRef.current = {
          fingerprint,
          requestKey: createGroupClientRequestKey(),
        };
      }
      const group = await groupService.createGroup(
        t('chat.fallback.groupName'),
        memberIds,
        assignments,
        {
          clientRequestKey: createAttemptRef.current.requestKey,
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
      toast(
        openFailed
          ? t('chat.messageList.toast.openGroupFailed')
          : t('chat.agentPicker.groupCreated', { memberCount: selectedCount, agentCount: selectedAgentIds.size }),
        openFailed ? 'error' : 'success',
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

  return (
    <ModalWrapper
      isOpen={isOpen}
      onClose={closeModal}
      title={t('chat.modal.title.createGroup')}
      width="w-[760px]"
      height="h-[700px]"
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
      <div className="flex h-full min-h-0">
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
      </div>
    </ModalWrapper>
  );
};
