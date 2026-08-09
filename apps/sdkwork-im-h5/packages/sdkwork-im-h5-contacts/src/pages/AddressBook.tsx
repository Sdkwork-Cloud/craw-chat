import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import {
  Search,
  UserPlus,
  Users,
  Tags,
  Building2,
  Network,
  Plus,
  ChevronLeft,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { ContactService, type Contact } from "../services/ContactService";
import { TopFunctionRow } from "../components/TopFunctionRow";
import { ContactRow } from "../components/ContactRow";
import { AlphabetIndexBar } from "../components/AlphabetIndexBar";

const INDEX_ALPHABET = [
  "↑",
  "☆",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "#",
];

export const AddressBook: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [activeLetter, setActiveLetter] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const letterIndicatorTimeout = useRef<any>(null);
  const [contactsData, setContactsData] = useState<Record<string, Contact[]>>({});
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [hasMore, setHasMore] = useState(false);
  const [loadError, setLoadError] = useState(false);

  useEffect(() => {
    const loadData = async () => {
      try {
        const page = await ContactService.listContactPage();
        setContactsData(groupContacts(page.items));
        setNextCursor(page.nextCursor);
        setHasMore(page.hasMore);
        setLoadError(false);
      } catch (error) {
        console.error(error);
        setLoadError(true);
      } finally {
        setLoading(false);
      }
    };
    void loadData();
  }, []);

  const loadMore = async () => {
    if (!hasMore || !nextCursor || loadingMore) {
      return;
    }
    setLoadingMore(true);
    try {
      const page = await ContactService.listContactPage(nextCursor);
      const current = Object.values(contactsData).flat();
      const byId = new Map(current.map((contact) => [contact.id, contact]));
      for (const contact of page.items) {
        byId.set(contact.id, contact);
      }
      setContactsData(groupContacts(Array.from(byId.values())));
      setNextCursor(page.nextCursor);
      setHasMore(page.hasMore);
      setLoadError(false);
    } catch (error) {
      console.error(error);
      setLoadError(true);
    } finally {
      setLoadingMore(false);
    }
  };

  const handleIndexClick = (letter: string) => {
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

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[52px] flex items-center justify-between px-2 bg-bg-color/90 backdrop-blur-md sticky top-0 z-20 shrink-0 pt-safe">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={
              <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 font-semibold text-[17px] text-text-main pointer-events-none">
          {t('contacts.title')}
        </div>
        <div className="flex items-center justify-end z-10 w-[80px] gap-1 pr-2">
          <IconButton
            icon={<Search className="w-5 h-5 text-text-main" />}
            onClick={() => navigate("/search")}
          />
          <IconButton
            icon={<Plus className="w-6 h-6 text-text-main" />}
            onClick={() => navigate("/add-friend")}
          />
        </div>
      </header>

      {/* Main Content Area */}
      <div
        className="flex-1 overflow-y-auto no-scrollbar relative pb-[84px]"
        ref={scrollRef}
        onScroll={(event) => {
          if (loadingMore || !hasMore || !nextCursor) {
            return;
          }
          const element = event.currentTarget;
          if (element.scrollTop + element.clientHeight >= element.scrollHeight - 120) {
            void loadMore();
          }
        }}
      >
        {/* Search Bar Placeholder */}
        <div className="px-3 py-2 bg-bg-color">
          <div
            className="h-9 w-full bg-chat-other-bg rounded-lg flex items-center justify-center gap-1.5 cursor-pointer active:opacity-70"
            onClick={() => navigate("/search")}
          >
            <Search className="w-4 h-4 text-text-sub" />
            <span className="text-[15px] text-text-sub">{t('contacts.search')}</span>
          </div>
        </div>

        {/* Function Rows */}
        <div className="flex flex-col mb-1">
          <TopFunctionRow
            icon={UserPlus}
            title={t('contacts.new_friends')}
            bgColor="bg-[#FA9D3B]"
            onClick={() => navigate("/contacts/friend-requests")}
          />
          <TopFunctionRow
            icon={Users}
            title={t('contacts.group_chats')}
            bgColor="bg-[#07C160]"
            onClick={() => showToast(t('contacts.capability_unavailable', 'Not available'))}
          />
          <TopFunctionRow
            icon={Tags}
            title={t('contacts.tags')}
            bgColor="bg-[#10aeff]"
            onClick={() => showToast(t('contacts.capability_unavailable', 'Not available'))}
          />
          <TopFunctionRow
            icon={Network}
            title={t('contacts.org_structure')}
            bgColor="bg-[#4395F5]"
            onClick={() => navigate("/contacts/org")}
          />
          <TopFunctionRow
            icon={Building2}
            title={t('contacts.official_accounts')}
            bgColor="bg-[#10aeff]"
            onClick={() => showToast(t('contacts.capability_unavailable', 'Not available'))}
          />
          <div className="pl-4 bg-bg-color">
            <div className="border-b border-border-color/50 w-full" />
          </div>
        </div>

        {/* Contact List Sections */}
        {Object.keys(contactsData)
          .sort()
          .map((letter) => (
            <div key={letter} id={`contact-section-${letter}`}>
              <div className="h-7 bg-[#EDEDED] dark:bg-[#1A1A1A] flex items-center pl-4 sticky top-0 z-10">
                <span className="text-[13px] font-semibold text-text-sub">
                  {letter}
                </span>
              </div>
              <div className="flex flex-col">
                {contactsData[letter].map((contact, index) => (
                  <ContactRow
                    key={contact.id}
                    contact={contact}
                    isLast={index === contactsData[letter].length - 1}
                  />
                ))}
              </div>
            </div>
          ))}



        {/* Footer padding */}
        <div className="h-[40px] flex items-center justify-center pb-safe mb-4">
          <span className="text-[14px] text-text-sub">
            {loading
              ? t('common.loading', 'Loading...')
              : loadError
                ? t('contacts.load_failed', 'Unable to load contacts')
                : t('contacts.contacts_count', { count: Object.values(contactsData).flat().length })}
          </span>
        </div>
      </div>

      {/* Right Alphabet Index */}
      <AlphabetIndexBar onIndexClick={handleIndexClick} />

      {/* Center Letter Indicator (Pop-up) */}
      <AnimatePresence>
        {activeLetter && (
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.8 }}
            className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-16 h-16 bg-black/60 backdrop-blur-md rounded-xl flex items-center justify-center z-50 shadow-2xl pointer-events-none"
          >
            <span className="text-white text-3xl font-bold">
              {activeLetter}
            </span>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

function groupContacts(contacts: Contact[]): Record<string, Contact[]> {
  const result: Record<string, Contact[]> = {};
  for (const contact of contacts) {
    const firstCharacter = contact.name.charAt(0).toUpperCase();
    const group = /^[A-Z]$/u.test(firstCharacter) ? firstCharacter : "#";
    (result[group] ??= []).push(contact);
  }
  for (const group of Object.keys(result)) {
    result[group].sort((left, right) => left.name.localeCompare(right.name));
  }
  return result;
}
