import React, { useCallback, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { Chat, User } from '@sdkwork/im-pc-types';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import { groupService } from '../services/GroupService';
import { ContactMemberPickerPanel } from './ContactMemberPickerPanel';
import { ModalWrapper } from './ModalWrapper';

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

  useEffect(() => {
    if (isOpen) {
      setLoading(true);
      contactService.listContactsPage()
        .then((page) => {
          setContacts(page.items);
          setContactsCursor(page.nextCursor);
          setHasMoreContacts(page.hasMore);
        })
        .catch(() => {
          setContacts([]);
          setContactsCursor(undefined);
          setHasMoreContacts(false);
          toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
        })
        .finally(() => setLoading(false));
    } else {
      setContacts([]);
      setContactsCursor(undefined);
      setHasMoreContacts(false);
      setSelected(new Set());
      setSearchQuery('');
      setCreating(false);
      setLoadingMoreContacts(false);
    }
  }, [isOpen, t]);

  const loadMoreContacts = useCallback(() => {
    if (!hasMoreContacts || loadingMoreContacts) {
      return;
    }
    setLoadingMoreContacts(true);
    void contactService.listContactsPage({ cursor: contactsCursor })
      .then((page) => {
        setContacts((previousContacts) => [...previousContacts, ...page.items]);
        setContactsCursor(page.nextCursor);
        setHasMoreContacts(page.hasMore);
      })
      .catch(() => {
        toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
      })
      .finally(() => setLoadingMoreContacts(false));
  }, [contactsCursor, hasMoreContacts, loadingMoreContacts, t]);

  const toggleSelect = (id: string) => {
    setSelected((previousSelected) => {
      const nextSelected = new Set(previousSelected);
      if (nextSelected.has(id)) {
        nextSelected.delete(id);
      } else {
        nextSelected.add(id);
      }
      return nextSelected;
    });
  };

  const handleCreate = async () => {
    if (selected.size === 0 || creating) {
      return;
    }

    setCreating(true);
    try {
      const selectedCount = selected.size;
      const group = await groupService.createGroup('', Array.from(selected));
      await onCreated?.(group);
      toast(t('chat.modal.toast.groupCreated', { count: selectedCount }), 'success');
      onClose();
    } catch {
      toast(t('chat.modal.toast.createGroupFailed'), 'error');
    } finally {
      setCreating(false);
    }
  };

  return (
    <ModalWrapper
      isOpen={isOpen}
      onClose={onClose}
      title={t('chat.modal.title.createGroup')}
      width="w-[760px]"
      height="h-[700px]"
      footer={
        <>
          <button onClick={onClose} className="rounded bg-white/5 px-4 py-2 text-sm text-gray-300 transition-colors hover:bg-white/10">
            {t('chat.modal.actions.cancel')}
          </button>
          <button
            disabled={selected.size === 0 || creating}
            className="rounded bg-[#00b42a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#009a24] disabled:cursor-not-allowed disabled:bg-[#00b42a]/50"
            onClick={() => void handleCreate()}
          >
            {creating
              ? t('chat.modal.actions.creating')
              : t('chat.modal.actions.createWithCount', { count: selected.size })}
          </button>
        </>
      }
    >
      <ContactMemberPickerPanel
        contacts={contacts}
        emptyText={t('chat.modal.state.noContactsToCreate')}
        hasMoreContacts={hasMoreContacts}
        isLoading={loading}
        isLoadingMoreContacts={loadingMoreContacts}
        onLoadMoreContacts={loadMoreContacts}
        searchPlaceholder={t('chat.modal.placeholder.memberSearch')}
        searchQuery={searchQuery}
        onSearchQueryChange={setSearchQuery}
        selectedIds={selected}
        onToggleContact={toggleSelect}
      />
    </ModalWrapper>
  );
};
