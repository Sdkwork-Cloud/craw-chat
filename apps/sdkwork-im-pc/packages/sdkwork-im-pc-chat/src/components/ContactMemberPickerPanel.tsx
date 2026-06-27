import React, { useMemo, useRef } from 'react';
import { Check, Loader2, Search, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { User } from '@sdkwork/im-pc-types';

export interface ContactMemberPickerPanelProps {
  contacts: User[];
  disabledContactIds?: Set<string>;
  disabledReason?: string;
  emptyText: string;
  isLoading: boolean;
  onSearchQueryChange: (query: string) => void;
  onToggleContact: (contactId: string) => void;
  searchPlaceholder: string;
  searchQuery: string;
  selectedIds: Set<string>;
}

function createContactSearchText(contact: User): string {
  return [
    contact.name,
    contact.chatId,
    contact.email,
    contact.phone,
    contact.company,
    contact.position,
    contact.py,
  ]
    .filter((value): value is string => Boolean(value?.trim()))
    .join(' ')
    .toLowerCase();
}

function createContactIndexKey(contact: User): string {
  const source = (contact.py || contact.name || '#').trim().charAt(0).toUpperCase();
  return /[A-Z]/u.test(source) ? source : '#';
}

function groupContactsByIndex(contacts: User[]): Array<{ contacts: User[]; key: string }> {
  const grouped = new Map<string, User[]>();
  for (const contact of contacts) {
    const key = createContactIndexKey(contact);
    grouped.set(key, [...(grouped.get(key) ?? []), contact]);
  }

  const groups = Array.from(grouped.entries()).map(([key, items]) => ({
    key,
    contacts: items.sort((left, right) => (left.py || left.name).localeCompare(right.py || right.name)),
  }));
  groups.sort((left, right) => {
    if (left.key === '#') {
      return 1;
    }
    if (right.key === '#') {
      return -1;
    }
    return left.key.localeCompare(right.key);
  });
  return groups;
}

function getContactSubtitle(contact: User): string | undefined {
  return contact.chatId
    ?? contact.email
    ?? contact.phone
    ?? contact.position
    ?? contact.company;
}

export const ContactMemberPickerPanel: React.FC<ContactMemberPickerPanelProps> = ({
  contacts,
  disabledContactIds = new Set<string>(),
  disabledReason,
  emptyText,
  isLoading,
  onSearchQueryChange,
  onToggleContact,
  searchPlaceholder,
  searchQuery,
  selectedIds,
}) => {
  const { t } = useTranslation();
  const groupRefs = useRef<Record<string, HTMLDivElement | null>>({});
  const normalizedQuery = searchQuery.trim().toLowerCase();

  const filteredContacts = useMemo(() => (
    contacts.filter((contact) => (
      !normalizedQuery || createContactSearchText(contact).includes(normalizedQuery)
    ))
  ), [contacts, normalizedQuery]);

  const groupedContacts = useMemo(() => groupContactsByIndex(filteredContacts), [filteredContacts]);
  const renderedIndexKeys = useMemo(() => (
    groupedContacts.map((group) => group.key)
  ), [groupedContacts]);
  const selectedContacts = useMemo(() => (
    contacts.filter((contact) => selectedIds.has(contact.id) && !disabledContactIds.has(contact.id))
  ), [contacts, disabledContactIds, selectedIds]);

  const scrollToIndexGroup = (indexKey: string) => {
    groupRefs.current[indexKey]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };

  return (
    <div className="grid h-full min-h-0 grid-cols-2 gap-4">
      <section className="flex min-h-0 flex-col overflow-hidden rounded-lg border border-white/5 bg-[#181818]">
        <div className="border-b border-white/5 p-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
            <input
              type="text"
              placeholder={searchPlaceholder}
              value={searchQuery}
              onChange={(event) => onSearchQueryChange(event.target.value)}
              className="h-9 w-full rounded-md border border-white/5 bg-[#111] py-2 pl-9 pr-3 text-sm text-gray-200 outline-none transition-colors focus:border-white/20"
            />
          </div>
        </div>

        <div className="min-h-0 flex-1">
          <div className="grid h-full min-h-0 grid-cols-[minmax(0,1fr)_24px]">
            <div className="h-full min-h-0 overflow-y-auto custom-scrollbar">
              {isLoading ? (
                <div className="flex items-center justify-center gap-2 py-12 text-sm text-gray-500">
                  <Loader2 size={16} className="animate-spin" />
                  <span>{t('chat.modal.state.loadingContacts')}</span>
                </div>
              ) : groupedContacts.length > 0 ? (
                groupedContacts.map((group) => (
                  <div
                    key={group.key}
                    ref={(node) => {
                      groupRefs.current[group.key] = node;
                    }}
                  >
                    <div className="sticky top-0 z-10 border-y border-white/5 bg-[#202020] px-4 py-1.5 text-xs font-medium text-gray-500">
                      {group.key}
                    </div>
                    <div className="py-1">
                      {group.contacts.map((contact) => {
                        const disabled = disabledContactIds.has(contact.id);
                        const checked = !disabled && selectedIds.has(contact.id);
                        const subtitle = getContactSubtitle(contact);

                        return (
                          <button
                            key={contact.id}
                            type="button"
                            disabled={disabled}
                            className={`flex w-full items-center gap-3 px-4 py-2.5 text-left transition-colors ${disabled ? 'cursor-not-allowed opacity-60' : 'hover:bg-white/5'}`}
                            onClick={() => {
                              if (!disabled) {
                                onToggleContact(contact.id);
                              }
                            }}
                          >
                            <span className={`flex h-5 w-5 shrink-0 items-center justify-center rounded-full border transition-colors ${checked ? 'border-[#00b42a] bg-[#00b42a]' : 'border-gray-500'}`}>
                              {checked && <Check size={12} className="text-white" />}
                            </span>
                            <Avatar src={contact.avatar} alt={contact.name} className="h-9 w-9 shrink-0 rounded bg-[#2b2b2d]" />
                            <span className="min-w-0 flex-1">
                              <span className="block truncate text-sm text-gray-200">{contact.name}</span>
                              {subtitle && (
                                <span className="mt-0.5 block truncate text-xs text-gray-500">{subtitle}</span>
                              )}
                              {disabled && disabledReason && (
                                <span className="mt-1 inline-flex max-w-full rounded border border-white/10 bg-white/5 px-1.5 py-0.5 text-xs text-gray-400">
                                  {disabledReason}
                                </span>
                              )}
                            </span>
                          </button>
                        );
                      })}
                    </div>
                  </div>
                ))
              ) : (
                <div className="flex h-full items-center justify-center px-6 text-center text-sm text-gray-500">
                  {normalizedQuery ? t('chat.modal.selection.noSearchResults') : emptyText}
                </div>
              )}
            </div>

            <div className="flex min-h-0 items-center justify-center border-l border-white/5 bg-[#141414]">
              <div className="flex max-h-full flex-col items-center justify-center gap-1 overflow-hidden py-2">
                {renderedIndexKeys.map((indexKey) => (
                  <button
                    key={indexKey}
                    type="button"
                    onClick={() => scrollToIndexGroup(indexKey)}
                    className="flex h-5 w-5 items-center justify-center rounded text-[10px] text-gray-400 transition-colors hover:bg-[#00b42a] hover:text-white"
                    aria-label={t('chat.modal.selection.indexAria', { index: indexKey })}
                  >
                    {indexKey}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </section>

      <aside className="flex min-h-0 flex-col rounded-lg border border-white/5 bg-[#181818]">
        <div className="border-b border-white/5 px-4 py-3">
          <div className="text-sm font-medium text-gray-200">{t('chat.modal.selection.selectedTitle')}</div>
          <div className="mt-0.5 text-xs text-gray-500">
            {t('chat.modal.selection.selectedCount', { count: selectedContacts.length })}
          </div>
        </div>

        <div className="min-h-0 flex-1 overflow-y-auto p-2 custom-scrollbar">
          {selectedContacts.length > 0 ? (
            selectedContacts.map((contact) => {
              const subtitle = getContactSubtitle(contact);
              return (
                <div key={contact.id} className="flex items-center gap-2 rounded-lg p-2">
                  <Avatar src={contact.avatar} alt={contact.name} className="h-8 w-8 shrink-0 rounded bg-[#2b2b2d]" />
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-sm text-gray-200">{contact.name}</div>
                    {subtitle && <div className="mt-0.5 truncate text-xs text-gray-500">{subtitle}</div>}
                  </div>
                  <button
                    type="button"
                    onClick={() => onToggleContact(contact.id)}
                    className="flex h-7 w-7 shrink-0 items-center justify-center rounded text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-200"
                    aria-label={t('chat.modal.selection.removeSelected', { name: contact.name })}
                  >
                    <X size={14} />
                  </button>
                </div>
              );
            })
          ) : (
            <div className="flex h-full items-center justify-center px-4 text-center text-sm text-gray-500">
              {t('chat.modal.selection.emptySelected')}
            </div>
          )}
        </div>
      </aside>
    </div>
  );
};
