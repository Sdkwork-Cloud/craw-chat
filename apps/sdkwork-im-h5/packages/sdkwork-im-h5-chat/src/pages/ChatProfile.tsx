import { useNavigate } from "react-router";
import { useParams } from "react-router";
import React, { useState, useEffect } from "react";
import {
  ChevronLeft,
  Search,
  Bell,
  Pin,
  Image as ImageIcon,
  Trash2,
  Settings2,
  EyeOff,
  UserMinus,
  LogOut,
} from "lucide-react";
import {
  IconButton,
  showConfirm,
  showPrompt,
  ListItem,
  Switch,
  showToast,
  ActionSheet,
} from "@sdkwork/im-h5-commons";
import { ChatService } from "../services/ChatService";
import { ContactService } from "@sdkwork/im-h5-contacts";
import { useAppStore } from "@sdkwork/im-h5-core";
import type { Chat, Message, User } from "@sdkwork/im-h5-types";
import { SearchHistoryOverlay } from "../components/Chat/SearchHistoryOverlay";
import { useTranslation } from "react-i18next";

import { ChatProfileMembers } from "../components/Chat/ChatProfileMembers";

export const ChatProfile: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const navigate = useNavigate();
  const [chat, setChat] = useState<Chat | null>(null);
  const [isMuted, setIsMuted] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
  const [showAvatar, setShowAvatar] = useState(true);
  const [cleanMode, setCleanMode] = useState(false);
  const [remark, setRemark] = useState("");
  const [isStarred, setIsStarred] = useState(false);
  const [canManageMembers, setCanManageMembers] = useState(false);
  const [selectedMember, setSelectedMember] = useState<User | null>(null);

  const directPeer = chat?.type === "direct" ? chat.participants[0] : undefined;
  const sessionUserId = useAppStore((state) => state.currentUser)?.id;

  useEffect(() => {
    if (directPeer) {
      void ContactService.getContactPreferences(directPeer.id)
        .then((preferences) => {
          setRemark(preferences.remark);
          setIsStarred(preferences.isStarred);
        })
        .catch((error) => console.error("Unable to load contact preferences", error));
    } else {
      setRemark("");
      setIsStarred(false);
    }
  }, [directPeer]);

  useEffect(() => {
    if (!id || chat?.type !== "group") {
      setCanManageMembers(false);
      return;
    }
    void ChatService.getMyConversationRole(id)
      .then((role) => setCanManageMembers(role === "owner" || role === "admin"))
      .catch((error) => console.error("Unable to load conversation role", error));
  }, [id, chat?.type]);

  // Search state
  const [showSearch, setShowSearch] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<Message[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  useEffect(() => {
    if (id) {
      void ChatService.getChatById(id).then((c) => {
        if (!c) return;
        setChat(c);
        setIsMuted(c.settings?.isMuted ?? false);
        setIsPinned(c.isPinned ?? false);
        setShowAvatar(c.settings?.showAvatar ?? true);
        setCleanMode(c.settings?.cleanMode ?? false);
      }).catch((error) => {
        console.error(error);
        showToast(t("chat.profile.load_failed", "Unable to load conversation settings"));
      });
    }
  }, [id]);

  useEffect(() => {
    if (showSearch && searchQuery.trim() && id) {
      setIsSearching(true);
      const timer = setTimeout(async () => {
        try {
          const results = await ChatService.searchChatHistory(id, searchQuery);
          setSearchResults(results);
        } catch (error) {
          console.error(error);
          setSearchResults([]);
          showToast(t("chat.profile.search_unavailable", "Message search is unavailable"));
        } finally {
          setIsSearching(false);
        }
      }, 300);
      return () => clearTimeout(timer);
    } else {
      setSearchResults([]);
    }
    return undefined;
  }, [searchQuery, showSearch, id]);

  const handleUpdateSettings = async (updates: Partial<NonNullable<Chat["settings"]>>) => {
    if (!id) return;
    try {
      const preferences: { isMuted?: boolean; isPinned?: boolean } = {};
      if (updates.isMuted !== undefined) preferences.isMuted = updates.isMuted;
      if (updates.isPinned !== undefined) preferences.isPinned = updates.isPinned;
      await ChatService.updateChatSettings(id, preferences);
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.update_failed", "Unable to update conversation settings"));
    }
  };

  const handleUpdateProfile = async (body: { displayName?: string; notice?: string }) => {
    if (!id) return;
    try {
      await ChatService.updateChatProfile(id, body);
      if (body.displayName) {
        setChat((value) => (value ? { ...value, name: body.displayName } : value));
      }
      if (body.notice !== undefined) {
        setChat((value) => (value ? { ...value, notice: body.notice } : value));
      }
      showToast(t("chat.profile.profile_updated", "Profile updated"));
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.profile_update_failed", "Unable to update profile"));
    }
  };

  const handleEditGroupName = () => {
    void showPrompt(t("chat.profile.group_name_hint", "Enter group name"), chat?.name ?? "").then((value) => {
      if (value === null) return;
      const normalized = value.trim();
      if (!normalized || normalized === chat?.name) return;
      void handleUpdateProfile({ displayName: normalized });
    });
  };

  const handleEditGroupNotice = () => {
    void showPrompt(t("chat.profile.group_notice_hint", "Enter group notice"), chat?.notice ?? "").then((value) => {
      if (value === null) return;
      const normalized = value.trim();
      if (!normalized || normalized === chat?.notice) return;
      void handleUpdateProfile({ notice: normalized });
    });
  };

  const handleEditRemark = () => {
    if (!directPeer) return;
    void showPrompt(t("chat.profile.remark_hint", "Enter a remark for this contact"), remark).then(async (value) => {
      if (value === null) return;
      const normalized = value.trim();
      try {
        const preferences = await ContactService.updateContactPreferences(directPeer.id, { remark: normalized });
        setRemark(preferences.remark);
        showToast(t("chat.profile.profile_updated", "Profile updated"));
      } catch (error) {
        console.error(error);
        showToast(t("chat.profile.profile_update_failed", "Unable to update profile"));
      }
    });
  };

  const handleToggleStar = async (value: boolean) => {
    if (!directPeer) return;
    try {
      const preferences = await ContactService.updateContactPreferences(directPeer.id, { isStarred: value });
      setIsStarred(preferences.isStarred);
      showToast(t("chat.profile.profile_updated", "Profile updated"));
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.profile_update_failed", "Unable to update profile"));
    }
  };

  const handleRemoveMember = async (member: User) => {
    if (!id || member.id === sessionUserId) return;
    setSelectedMember(null);
    try {
      await ChatService.removeGroupMember(id, member.id);
      setChat((value) => value
        ? { ...value, participants: value.participants.filter((participant) => participant.id !== member.id) }
        : value);
      showToast(t("chat.profile.member_removed", "Member removed"));
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.member_remove_failed", "Unable to remove member"));
    }
  };

  const handleLeaveGroup = async () => {
    if (!id) return;
    if (!(await showConfirm(t("chat.profile.leave_group_confirm", "Are you sure you want to leave this group?")))) {
      return;
    }
    try {
      await ChatService.leaveGroupChat(id);
      navigate("/", { replace: true });
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.leave_group_failed", "Unable to leave the group"));
    }
  };

  const handleBlockContact = async () => {
    if (!directPeer) return;
    if (!(await showConfirm(t("chat.profile.block_contact_confirm", "Block this contact? They will no longer be able to message you.")))) {
      return;
    }
    try {
      await ContactService.blockContact(directPeer.id);
      showToast(t("chat.profile.blocked", "Contact blocked"));
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.action_failed", "Action failed"));
    }
  };

  const handleRemoveFriend = async () => {
    if (!directPeer) return;
    if (!(await showConfirm(t("chat.profile.remove_friend_confirm", "Remove this contact? The conversation will also be deleted.")))) {
      return;
    }
    try {
      await ContactService.removeFriend(directPeer.id);
      if (id) await ChatService.deleteChat(id);
      navigate("/", { replace: true });
    } catch (error) {
      console.error(error);
      showToast(t("chat.profile.action_failed", "Action failed"));
    }
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
          <h2 className="text-[17px] font-medium text-text-main">{t('chat.profile.title')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col px-0 sm:px-4 pb-8">
        {/* Members */}
        <ChatProfileMembers
          chat={chat}
          onAddMember={() => navigate(`/create-group?chatId=${id}`)}
          onMemberClick={chat?.type === "group" && canManageMembers
            ? (member) => setSelectedMember(member)
            : undefined}
        />

        {/* Profile Group (group name / notice or contact remark) */}
        {chat?.type === "group" ? (
          <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
            <ListItem
              icon={Settings2}
              label={t('chat.profile.group_name')}
              value={chat.name}
              onClick={handleEditGroupName}
            />
            <ListItem
              icon={Settings2}
              label={t('chat.profile.group_notice')}
              value={chat.notice}
              onClick={handleEditGroupNotice}
            />
          </div>
        ) : directPeer ? (
          <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
            <ListItem
              icon={Settings2}
              label={t('chat.profile.remark')}
              value={remark || undefined}
              onClick={handleEditRemark}
            />
            <ListItem
              icon={Settings2}
              label={t('chat.profile.star_contact')}
              rightElement={<Switch checked={isStarred} onChange={(value) => void handleToggleStar(value)} />}
            />
          </div>
        ) : null}

        {/* Settings Group 1 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Search}
            label={t('chat.profile.search_history')}
            onClick={() => { setSearchQuery(""); setShowSearch(true); }}
          />
        </div>

        {/* Settings Group 2 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Bell}
            label={t('chat.profile.mute')}
            rightElement={<Switch checked={isMuted} onChange={(value) => { setIsMuted(value); void handleUpdateSettings({ isMuted: value }); }} />}
          />
          <ListItem
            icon={Pin}
            label={t('chat.profile.pin')}
            rightElement={<Switch checked={isPinned} onChange={(value) => { setIsPinned(value); void handleUpdateSettings({ isPinned: value }); }} />}
          />
        </div>

        {/* Settings Group 3 (Display Settings) */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Settings2}
            label={t('chat.profile.show_avatar')}
            rightElement={
              <Switch
                checked={showAvatar}
                onChange={(val: boolean) => {
                  setShowAvatar(val);
                  showToast(t("chat.profile.display_setting_local", "Display settings are local to this client"));
                }}
              />
            }
          />
          <ListItem
            icon={EyeOff}
            label={t('chat.profile.clean_mode')}
            rightElement={
              <Switch
                checked={cleanMode}
                onChange={(val: boolean) => {
                  setCleanMode(val);
                  showToast(t("chat.profile.display_setting_local", "Display settings are local to this client"));
                }}
              />
            }
          />
        </div>

        {/* Settings Group 4 */}
        <div className="mb-6 sm:mb-8 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={ImageIcon}
            label={t('chat.profile.set_background')}
            onClick={() => showToast(t("chat.profile.set_background_unavailable", "Chat background is unavailable"))}
          />
        </div>

        {/* Danger Zone */}
        <div className="sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Trash2}
            label={t('chat.profile.clear_history')}
            danger={true}
            rightElement={<div />}
            onClick={async () => {
              if (await showConfirm(t('chat.profile.clear_history_confirm'))) {
                showToast(t("chat.profile.clear_history_unavailable", "Clear history is unavailable"));
              }
            }}
          />
          {chat?.type === "group" && (
            <ListItem
              icon={LogOut}
              label={t('chat.profile.leave_group')}
              danger={true}
              rightElement={<div />}
              onClick={() => void handleLeaveGroup()}
            />
          )}
          {chat?.type === "direct" && directPeer && (
            <>
              <ListItem
                icon={UserMinus}
                label={t('chat.profile.block_contact')}
                danger={true}
                rightElement={<div />}
                onClick={() => void handleBlockContact()}
              />
              <ListItem
                icon={Trash2}
                label={t('chat.profile.remove_friend')}
                danger={true}
                rightElement={<div />}
                onClick={() => void handleRemoveFriend()}
              />
            </>
          )}
        </div>
      </div>

      <ActionSheet
        isOpen={selectedMember !== null}
        onClose={() => setSelectedMember(null)}
        title={selectedMember?.name}
        options={[
          {
            label: t("chat.profile.remove_member", "Remove member"),
            danger: true,
            onClick: () => {
              if (selectedMember) void handleRemoveMember(selectedMember);
            },
          },
        ]}
      />

      <SearchHistoryOverlay
        id={id as string}
        chat={chat}
        showSearch={showSearch}
        setShowSearch={setShowSearch}
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        isSearching={isSearching}
        searchResults={searchResults}
      />
    </div>
  );
};
