import React, { useEffect, useMemo, useState } from 'react';
import { Check, Loader2, Search, Send } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { Chat, User } from '@sdkwork/im-pc-types';
import { contactService } from '../services/ContactService';
import { groupService } from '../services/GroupService';
import { ContactMemberPickerPanel } from './ContactMemberPickerPanel';
import { ModalWrapper } from './ModalWrapper';
import { toast } from './Toast';

export interface AddGroupMembersModalProps {
  chat: Chat | null;
  isOpen: boolean;
  onAdded?: (count: number) => void | Promise<void>;
  onClose: () => void;
}

type InviteMemberTab = 'contacts' | 'strangers';

function isExistingGroupMember(existingMemberIds: Set<string>, contact: User): boolean {
  return existingMemberIds.has(contact.id)
    || (Boolean(contact.chatId) && existingMemberIds.has(contact.chatId ?? ''));
}

export const AddGroupMembersModal: React.FC<AddGroupMembersModalProps> = ({
  chat,
  isOpen,
  onAdded,
  onClose,
}) => {
  const { t } = useTranslation();
  const [contacts, setContacts] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSearchingNonContacts, setIsSearchingNonContacts] = useState(false);
  const [isInvitingNonContact, setIsInvitingNonContact] = useState(false);
  const [activeTab, setActiveTab] = useState<InviteMemberTab>('contacts');
  const [searchQuery, setSearchQuery] = useState('');
  const [nonContactSearchQuery, setNonContactSearchQuery] = useState('');
  const [nonContactSearchResults, setNonContactSearchResults] = useState<User[]>([]);
  const [selectedNonContactUser, setSelectedNonContactUser] = useState<User | null>(null);
  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!isOpen) {
      setContacts([]);
      setActiveTab('contacts');
      setSearchQuery('');
      setNonContactSearchQuery('');
      setNonContactSearchResults([]);
      setSelectedNonContactUser(null);
      setSelected(new Set());
      setIsSubmitting(false);
      setIsSearchingNonContacts(false);
      setIsInvitingNonContact(false);
      return;
    }

    if (activeTab !== 'contacts') {
      return;
    }

    setIsLoading(true);
    contactService.getContacts()
      .then((items) => {
        setContacts(items);
      })
      .catch(() => {
        setContacts([]);
        toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
      })
      .finally(() => setIsLoading(false));
  }, [activeTab, isOpen, t]);

  const existingMemberIds = useMemo(() => {
    if (!chat) {
      return new Set<string>();
    }
    return new Set(chat.members ?? []);
  }, [chat]);

  const disabledContactIds = useMemo(() => {
    const disabledIds = new Set<string>();
    for (const contact of contacts) {
      if (isExistingGroupMember(existingMemberIds, contact)) {
        disabledIds.add(contact.id);
      }
    }
    return disabledIds;
  }, [contacts, existingMemberIds]);

  const selectedInviteIds = useMemo(() => (
    Array.from(selected).filter((contactId) => !disabledContactIds.has(contactId))
  ), [disabledContactIds, selected]);

  const canInviteSelectedNonContact = Boolean(
    selectedNonContactUser && !isExistingGroupMember(existingMemberIds, selectedNonContactUser),
  );

  const toggleContact = (contactId: string) => {
    setSelected((previousSelected) => {
      if (disabledContactIds.has(contactId)) {
        return previousSelected;
      }

      const nextSelected = new Set(previousSelected);
      if (nextSelected.has(contactId)) {
        nextSelected.delete(contactId);
      } else {
        nextSelected.add(contactId);
      }
      return nextSelected;
    });
  };

  const handleSubmit = async () => {
    if (!chat || selectedInviteIds.length === 0 || isSubmitting) {
      return;
    }

    setIsSubmitting(true);
    try {
      const selectedCount = selectedInviteIds.length;
      await groupService.addMembers(chat.id, selectedInviteIds);
      await onAdded?.(selectedCount);
      toast(t('chat.modal.toast.invitedMembers', { count: selectedCount }), 'success');
      onClose();
    } catch {
      toast(t('chat.modal.toast.inviteFailed'), 'error');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleNonContactSearch = async () => {
    const query = nonContactSearchQuery.trim();
    if (activeTab !== 'strangers' || !query || isSearchingNonContacts) {
      return;
    }

    setIsSearchingNonContacts(true);
    setSelectedNonContactUser(null);
    try {
      const results = await contactService.searchContacts(nonContactSearchQuery);
      setNonContactSearchResults(results);
    } catch {
      setNonContactSearchResults([]);
      toast(t('chat.modal.toast.contactsLoadFailed'), 'error');
    } finally {
      setIsSearchingNonContacts(false);
    }
  };

  const handleInviteNonContact = async () => {
    if (
      !chat
      || !selectedNonContactUser
      || isExistingGroupMember(existingMemberIds, selectedNonContactUser)
      || isInvitingNonContact
    ) {
      return;
    }

    setIsInvitingNonContact(true);
    try {
      await groupService.inviteUserToGroup(chat, selectedNonContactUser);
      await onAdded?.(1);
      toast(t('chat.modal.toast.invitedMembers', { count: 1 }), 'success');
      onClose();
    } catch {
      toast(t('chat.modal.toast.inviteFailed'), 'error');
    } finally {
      setIsInvitingNonContact(false);
    }
  };

  const renderContactsTab = () => (
    <ContactMemberPickerPanel
      contacts={contacts}
      disabledContactIds={disabledContactIds}
      disabledReason={t('chat.modal.selection.alreadyInGroup')}
      emptyText={t('chat.modal.state.noContactsToInvite')}
      isLoading={isLoading}
      searchPlaceholder={t('chat.modal.placeholder.memberSearch')}
      searchQuery={searchQuery}
      onSearchQueryChange={setSearchQuery}
      selectedIds={selected}
      onToggleContact={toggleContact}
    />
  );

  const renderStrangersTab = () => (
    <>
      <div className="mb-3 flex gap-2">
        <div className="relative min-w-0 flex-1">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
          <input
            type="text"
            placeholder={t('chat.modal.placeholder.nonContactSearch')}
            value={nonContactSearchQuery}
            onChange={(event) => setNonContactSearchQuery(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === 'Enter') {
                void handleNonContactSearch();
              }
            }}
            className="w-full rounded-lg border border-white/5 bg-[#181818] py-2 pl-9 pr-4 text-sm text-gray-200 outline-none transition-colors focus:border-white/20"
          />
        </div>
        <button
          type="button"
          disabled={!nonContactSearchQuery.trim() || isSearchingNonContacts}
          onClick={() => void handleNonContactSearch()}
          className="flex h-9 items-center gap-1 rounded bg-white/10 px-3 text-sm text-gray-200 transition-colors hover:bg-white/15 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {isSearchingNonContacts ? <Loader2 size={14} className="animate-spin" /> : <Search size={14} />}
          {t('chat.modal.actions.search')}
        </button>
      </div>

      <div className="min-h-[220px] space-y-1 overflow-y-auto">
        {nonContactSearchResults.map((user) => {
          const isAlreadyInGroup = isExistingGroupMember(existingMemberIds, user);
          const selectedUser = !isAlreadyInGroup && selectedNonContactUser?.id === user.id;

          return (
            <button
              key={user.id}
              type="button"
              disabled={isAlreadyInGroup}
              className={`flex w-full items-center gap-3 rounded-lg p-2 text-left transition-colors ${isAlreadyInGroup ? 'cursor-not-allowed opacity-60' : 'hover:bg-white/5'}`}
              onClick={() => {
                if (!isAlreadyInGroup) {
                  setSelectedNonContactUser(user);
                }
              }}
            >
              <span className={`flex h-5 w-5 shrink-0 items-center justify-center rounded-full border transition-colors ${selectedUser ? 'border-[#00b42a] bg-[#00b42a]' : 'border-gray-500'}`}>
                {selectedUser && <Check size={12} className="text-white" />}
              </span>
              <Avatar src={user.avatar} alt={user.name} className="h-8 w-8 shrink-0 rounded bg-[#2b2b2d]" />
              <span className="min-w-0 flex-1">
                <span className="block truncate text-sm text-gray-200">{user.name}</span>
                {(user.chatId || user.email || user.phone || user.id) && (
                  <span className="mt-0.5 block truncate text-xs text-gray-500">
                    {user.chatId ?? user.email ?? user.phone ?? user.id}
                  </span>
                )}
                {isAlreadyInGroup && (
                  <span className="mt-1 inline-flex max-w-full rounded border border-white/10 bg-white/5 px-1.5 py-0.5 text-xs text-gray-400">
                    {t('chat.modal.selection.alreadyInGroup')}
                  </span>
                )}
              </span>
            </button>
          );
        })}
        {!isSearchingNonContacts && nonContactSearchQuery.trim() && nonContactSearchResults.length === 0 && (
          <div className="py-8 text-center text-sm text-gray-500">
            {t('chat.modal.state.noNonContactResults')}
          </div>
        )}
      </div>
    </>
  );

  return (
    <ModalWrapper
      isOpen={isOpen && Boolean(chat)}
      onClose={onClose}
      title={t('chat.modal.title.addMember')}
      width="w-[820px]"
      height="h-[740px]"
      footer={
        <>
          <button
            onClick={onClose}
            className="rounded bg-white/5 px-4 py-2 text-sm text-gray-300 transition-colors hover:bg-white/10"
          >
            {t('chat.modal.actions.cancel')}
          </button>
          {activeTab === 'contacts' ? (
            <button
              disabled={selectedInviteIds.length === 0 || isSubmitting}
              onClick={() => void handleSubmit()}
              className="rounded bg-[#00b42a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#009a24] disabled:cursor-not-allowed disabled:bg-[#00b42a]/50"
            >
              {isSubmitting
                ? t('chat.modal.actions.inviting')
                : t('chat.modal.actions.inviteWithCount', { count: selectedInviteIds.length })}
            </button>
          ) : (
            <button
              type="button"
              disabled={!canInviteSelectedNonContact || isInvitingNonContact}
              onClick={() => void handleInviteNonContact()}
              className="flex items-center gap-2 rounded bg-[#00b42a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#009a24] disabled:cursor-not-allowed disabled:bg-[#00b42a]/50"
            >
              {isInvitingNonContact ? <Loader2 size={14} className="animate-spin" /> : <Send size={14} />}
              {t('chat.modal.actions.sendGroupInvite')}
            </button>
          )}
        </>
      }
    >
      <div className="flex h-full min-h-0 flex-col">
        <div className="mb-4 grid grid-cols-2 rounded-lg bg-white/5 p-1" role="tablist" aria-label={t('chat.modal.title.addMember')}>
          {(['contacts', 'strangers'] as InviteMemberTab[]).map((tab) => (
            <button
              key={tab}
              type="button"
              role="tab"
              aria-selected={activeTab === tab}
              className={`h-8 rounded-md text-sm transition-colors ${activeTab === tab ? 'bg-[#00b42a] text-white' : 'text-gray-400 hover:bg-white/5 hover:text-gray-200'}`}
              onClick={() => setActiveTab(tab)}
            >
              {tab === 'contacts'
                ? t('chat.modal.tabs.contacts')
                : t('chat.modal.tabs.strangers')}
            </button>
          ))}
        </div>

        <div className="min-h-0 flex-1">
          {activeTab === 'contacts' ? renderContactsTab() : renderStrangersTab()}
        </div>
      </div>
    </ModalWrapper>
  );
};
