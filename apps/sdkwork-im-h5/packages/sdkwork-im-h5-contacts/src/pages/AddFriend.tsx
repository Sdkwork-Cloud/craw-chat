import React, { useState } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Search,
  QrCode,
  UserPlus,
  Smartphone,
  ChevronRight,
} from "lucide-react";
import {
  IconButton,
  Avatar,
  cn,
  showToast,
} from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import {
  ContactService,
  classifyFriendRequestSubmitError,
  type ContactSearchResult,
} from "../services/ContactService";

export const AddFriend: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [searchQuery, setSearchQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);
  const [searchResults, setSearchResults] = useState<ContactSearchResult[]>([]);
  const [selectedResult, setSelectedResult] = useState<ContactSearchResult | null>(null);
  const [isAdding, setIsAdding] = useState(false);

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;
    setIsSearching(true);
    try {
      const results = await ContactService.searchFriends(searchQuery);
      setSearchResults(results);
      setSelectedResult(results.length === 1 ? results[0] : null);
    } catch (error) {
      console.error(error);
      setSearchResults([]);
      setSelectedResult(null);
      showToast(t('contacts.search_failed', 'Search failed'));
    } finally {
      setIsSearching(false);
    }
  };

  const handleAddFriend = async () => {
    if (!selectedResult || isAdding) return;
    setIsAdding(true);
    try {
      await ContactService.addFriend(selectedResult.id);
      navigate("/workspace/contacts", { replace: true });
    } catch (e) {
      console.error(e);
      const conflict = classifyFriendRequestSubmitError(e);
      if (conflict === "already_friend") {
        showToast(t('contacts.add_failed_already_friend', 'Already friends'));
      } else if (conflict === "pending") {
        showToast(t('contacts.add_failed_pending', 'A friend request is already pending'));
      } else if (conflict === "blocked") {
        showToast(t('contacts.add_failed_blocked', 'Unable to add: the user has blocked you or restricted adds'));
      } else {
        showToast(t('contacts.add_failed'));
      }
      setIsAdding(false);
    }
  };

  const ListItem = ({
    icon: Icon,
    title,
    subtitle,
    colorClass,
    onClick,
  }: {
    icon: React.ElementType;
    title: string;
    subtitle?: string;
    colorClass?: string;
    onClick?: () => void;
  }) => {
  const { t } = useTranslation();
  return (
    <div
      className={`flex items-center gap-4 px-4 py-3.5 bg-chat-other-bg border-b border-border-color last:border-none ${onClick ? "active:bg-active-bg transition-colors cursor-pointer" : ""}`}
      onClick={onClick}
    >
      <div
        className={cn(
          "w-10 h-10 rounded-lg flex items-center justify-center shrink-0",
          colorClass,
        )}
      >
        <Icon className="w-6 h-6 text-white" />
      </div>
      <div className="flex flex-col flex-1 min-w-0">
        <span className="text-[16px] font-medium text-text-main">{title}</span>
        <span className="text-[13px] text-text-sub truncate">{subtitle}</span>
      </div>
      <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
    </div>
  );
};


  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute inset-x-0 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{t('contacts.add_friend')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col px-0 sm:px-4 pb-8">
        {/* Search Bar */}
        <div className="px-4 py-3 sm:px-0">
          <div className="flex items-center gap-2 bg-chat-other-bg rounded-xl px-3 py-2.5 border border-border-color focus-within:border-primary-blue transition-colors">
            <Search className="w-5 h-5 text-text-sub shrink-0" />
            <input
              type="text"
              placeholder={t('contacts.add_friend_placeholder')}
              className="flex-1 bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
              value={searchQuery}
              onChange={(e) => {
                setSearchQuery(e.target.value);
                setSearchResults([]);
                setSelectedResult(null);
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleSearch();
                }
              }}
            />
            {searchQuery && (
              <button
                onClick={handleSearch}
                className="text-primary-blue text-[14px] font-medium px-2"
              >
                {t('contacts.search_button')}
              </button>
            )}
          </div>

          <div className="flex items-center justify-center gap-2 mt-4 text-[14px] text-text-sub">
            <span>{t('contacts.my_wechat_id', { id: "wxid_123456789" })}</span>
            <QrCode className="w-4 h-4 text-primary-blue cursor-pointer active:opacity-70" />
          </div>
        </div>

        {/* Search Result */}
        {isSearching && (
          <div className="px-4 py-8 flex justify-center">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin" />
          </div>
        )}

        {searchResults.length > 0 && !isSearching && (
          <div className="mt-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color bg-chat-other-bg flex flex-col">
            {searchResults.map((result) => (
              <button
                key={result.id}
                type="button"
                onClick={() => setSelectedResult(result)}
                className={cn(
                  "flex items-center gap-4 px-4 py-4 border-b border-border-color text-left",
                  selectedResult?.id === result.id && "bg-active-bg",
                )}
              >
                <Avatar src={result.avatar} size="lg" />
                <div className="flex flex-col flex-1 min-w-0">
                  <span className="text-[16px] font-medium text-text-main">
                    {result.name}
                  </span>
                  <span className="text-[13px] text-text-sub truncate">
                    {result.email ?? result.phone ?? result.id}
                  </span>
                </div>
              </button>
            ))}
            <div
              onClick={handleAddFriend}
              className={cn(
                "px-4 py-3.5 flex items-center justify-center font-medium text-[16px] transition-colors",
                selectedResult
                  ? "text-primary-blue active:bg-active-bg cursor-pointer"
                  : "text-text-sub cursor-not-allowed",
              )}
            >
              {isAdding ? t('contacts.adding') : t('contacts.add_to_contacts')}
            </div>
          </div>
        )}

        {/* Options */}
        {searchResults.length === 0 && !isSearching && (
          <div className="mt-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
            <ListItem
              icon={QrCode}
              title={t('contacts.scan_qr')}
              subtitle={t('contacts.scan_qr_desc')}
              colorClass="bg-[#2B5CE7]"
              onClick={() => navigate("/scan")}
            />
            <ListItem
              icon={Smartphone}
              title={t('contacts.mobile_contacts')}
              subtitle={t('contacts.mobile_contacts_desc')}
              colorClass="bg-accent-green"
            />
            <ListItem
              icon={UserPlus}
              title={t('contacts.enterprise_contacts')}
              subtitle={t('contacts.enterprise_contacts_desc')}
              colorClass="bg-[#FF7D00]"
            />
          </div>
        )}
      </div>
    </div>
  );
};
