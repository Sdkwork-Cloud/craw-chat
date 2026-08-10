import React, { useState, useEffect, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft, Search, Check } from "lucide-react";
import { Avatar, IconButton, cn, showToast } from "@sdkwork/im-h5-commons";
import { ContactService, type Contact } from "@sdkwork/im-h5-contacts";
import { ChatService } from "../services/ChatService";
import type { User, Chat } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";
import { SelectedContactsHorizontalList } from "../components/SelectedContactsHorizontalList";
import { AlphabetIndex } from "../components/AlphabetIndex";
import { ContactSelectionRow } from "../components/ContactSelectionRow";

export const CreateGroupChat: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const baseChatId = searchParams.get("chatId");

  const [searchQuery, setSearchQuery] = useState("");
  const [groupName, setGroupName] = useState("");
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [contacts, setContacts] = useState<User[]>([]);
  const [contactsDict, setContactsDict] = useState<Record<string, User[]>>({});
  const [isCreating, setIsCreating] = useState(false);
  const [existingChat, setExistingChat] = useState<Chat | null>(null);
  const [nextCursor, setNextCursor] = useState<string>();
  const [hasMoreContacts, setHasMoreContacts] = useState(false);
  const [loadingMoreContacts, setLoadingMoreContacts] = useState(false);

  const [activeLetter, setActiveLetter] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const letterIndicatorTimeout = useRef<any>(null);

  useEffect(() => {
    void ContactService.listContactPage().then((page) => {
      setContacts(page.items as User[]);
      setContactsDict(groupContacts(page.items as User[]));
      setNextCursor(page.nextCursor);
      setHasMoreContacts(page.hasMore);
    }).catch((error) => {
      console.error(error);
      showToast(t("chat.create_group.contacts_failed", "Unable to load contacts"));
    });
  }, []);

  const loadMoreContacts = async () => {
    if (!nextCursor || !hasMoreContacts || loadingMoreContacts) return;
    setLoadingMoreContacts(true);
    try {
      const page = await ContactService.listContactPage(nextCursor);
      const merged = new Map(contacts.map((contact) => [contact.id, contact]));
      for (const contact of page.items) merged.set(contact.id, contact as User);
      const items = Array.from(merged.values());
      setContacts(items);
      setContactsDict(groupContacts(items));
      setNextCursor(page.nextCursor);
      setHasMoreContacts(page.hasMore);
    } catch (error) {
      console.error(error);
      showToast(t("chat.create_group.contacts_failed", "Unable to load contacts"));
    } finally {
      setLoadingMoreContacts(false);
    }
  };

  useEffect(() => {
    if (baseChatId) {
      ChatService.getChatById(baseChatId).then(chat => {
        if (chat) setExistingChat(chat);
      });
    }
  }, [baseChatId]);

  const handleCreate = async () => {
    if (selectedIds.size === 0 || isCreating) return;
    const name = groupName.trim() || t('chat.create_group.group_chat');
    setIsCreating(true);
    try {
      if (existingChat) {
        if (existingChat.type === "group") {
          await ChatService.addParticipants(existingChat.id, Array.from(selectedIds));
          showToast(t('chat.create_group.add_success'));
          navigate(-1); // Go back to chat profile
        } else {
          const chat = await ChatService.createGroupChat(
            name,
            [...existingChat.participants.map(p => p.id), ...Array.from(selectedIds)],
          );
          showToast(t('chat.create_group.create_success'));
          navigate(`/chat/${chat.id}`, { replace: true });
        }
      } else {
        const chat = await ChatService.createGroupChat(
          name,
          Array.from(selectedIds),
        );
        showToast(t('chat.create_group.create_success'));
        navigate(`/chat/${chat.id}`, { replace: true });
      }
    } catch (error) {
      console.error(error);
      showToast(t('chat.create_group.create_failed'));
      setIsCreating(false);
    }
  };

  const toggleSelection = (id: string, disabled: boolean) => {
  if (disabled) return;
    const newSet = new Set(selectedIds);
    if (newSet.has(id)) {
      newSet.delete(id);
    } else {
      newSet.add(id);
    }
    setSelectedIds(newSet);
  };

  const handleIndexClick = (letter: string) => {
  if (searchQuery) return; // Disable index when searching
    setActiveLetter(letter);
    if (letterIndicatorTimeout.current)
      clearTimeout(letterIndicatorTimeout.current);
    letterIndicatorTimeout.current = setTimeout(
      () => setActiveLetter(null),
      800,
    );

    if (letter === "↑") {
      scrollRef.current?.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }

    const section = document.getElementById(`contact-section-${letter}`);
    if (section && scrollRef.current) {
      scrollRef.current.scrollTo({ top: section.offsetTop, behavior: "smooth" });
    }
  };

  const filteredContacts = searchQuery
    ? contacts.filter((c) =>
        c.name.toLowerCase().includes(searchQuery.toLowerCase()),
      )
    : contacts;

  const renderContactRow = (contact: User) => {
    const isDisabled = existingChat?.participants.some(p => p.id === contact.id) ?? false;
    const isSelected = selectedIds.has(contact.id) || isDisabled;
    
    return (
      <ContactSelectionRow
        key={contact.id}
        contact={contact}
        isSelected={isSelected}
        isDisabled={isDisabled}
        onToggle={toggleSelection}
      />
    );
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute inset-x-0 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{t('chat.create_group.title')}</h2>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-3">
          <button
            onClick={handleCreate}
            disabled={selectedIds.size === 0 || isCreating}
            className={cn(
              "px-3 py-1.5 rounded-md text-[14px] font-medium transition-colors",
              selectedIds.size > 0 && !isCreating
                ? "bg-primary-blue text-white active:bg-blue-600"
                : "bg-black/5 dark:bg-white/5 text-text-sub cursor-not-allowed",
            )}
          >
            {isCreating
              ? t('chat.create_group.creating')
              : `${t('chat.create_group.complete')} ${selectedIds.size > 0 ? `(${selectedIds.size})` : ""}`}
          </button>
        </div>
      </header>

      {/* Selected Contacts Horizontal Scroll */}
      <SelectedContactsHorizontalList 
        selectedIds={selectedIds}
        contacts={contacts}
        toggleSelection={toggleSelection}
      />

      {/* Group Name */}
      {!existingChat && (
        <div className="px-4 py-2 bg-bg-color">
          <div className="flex items-center gap-2 bg-chat-other-bg rounded-xl px-3 py-2 border border-border-color focus-within:border-primary-blue transition-colors">
            <input
              type="text"
              placeholder={t('chat.create_group.group_name_placeholder')}
              className="flex-1 bg-transparent text-[15px] text-text-main focus:outline-none placeholder:text-text-sub"
              value={groupName}
              maxLength={64}
              onChange={(e) => setGroupName(e.target.value)}
            />
          </div>
        </div>
      )}

      {/* Search Bar */}
      <div className="px-4 py-2 bg-bg-color">
        <div className="flex items-center gap-2 bg-chat-other-bg rounded-xl px-3 py-2 border border-border-color focus-within:border-primary-blue transition-colors">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            type="text"
            placeholder={t('chat.create_group.search')}
            className="flex-1 bg-transparent text-[15px] text-text-main focus:outline-none placeholder:text-text-sub"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </div>

      {/* Contact List */}
      <div className="flex-1 overflow-y-auto relative no-scrollbar" ref={scrollRef}>
        <div className="flex flex-col pb-8">
          {searchQuery ? (
            filteredContacts.map(renderContactRow)
          ) : (
            Object.keys(contactsDict)
              .sort()
              .map((letter) => (
                <div key={letter} id={`contact-section-${letter}`}>
                  <div className="h-7 bg-hover-bg flex items-center pl-4 sticky top-0 z-10">
                    <span className="text-[13px] font-semibold text-text-sub">
                      {letter}
                    </span>
                  </div>
                  <div className="flex flex-col">
                    {contactsDict[letter].map(renderContactRow)}
                  </div>
                </div>
              ))
          )}

        </div>
      </div>

      <AlphabetIndex 
        searchQuery={searchQuery}
        activeLetter={activeLetter}
        handleIndexClick={handleIndexClick}
      />
    </div>
  );
};

function groupContacts(contacts: User[]): Record<string, User[]> {
  const grouped: Record<string, User[]> = {};
  for (const contact of contacts) {
    const first = contact.name.charAt(0).toUpperCase();
    const key = /^[A-Z]$/u.test(first) ? first : "#";
    (grouped[key] ??= []).push(contact);
  }
  for (const key of Object.keys(grouped)) {
    grouped[key].sort((left, right) => left.name.localeCompare(right.name));
  }
  return grouped;
}
