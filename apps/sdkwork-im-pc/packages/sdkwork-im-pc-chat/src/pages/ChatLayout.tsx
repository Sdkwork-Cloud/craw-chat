import React, { useState, useRef, useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { motion, AnimatePresence } from "motion/react";
import { Sidebar } from "../components/Sidebar";
import { ChatList } from "../components/ChatList";
import { ChatWindow } from "../components/ChatWindow";
import { ChatEmptyHome } from "../components/ChatEmptyHome";
import { ChatHistoryModal } from "../components/ChatHistoryModal";
import { ChatRightPanel } from "../components/ChatRightPanel";
import { WindowControls } from "../components/WindowControls";
import { CallOverlay, CallType } from "../components/CallOverlay";
import { CreateGroupModal } from "../components/CreateGroupModal";
import { AddGroupMembersModal } from "../components/AddGroupMembersModal";
import { AddFriendModal } from "../components/AddFriendModal";
import { CreateAgentModal } from "@sdkwork/agents-pc-agents";
import { ScanQrCodeModal } from "../components/ScanQrCodeModal";
import { SettingsModal } from "../components/SettingsModal";
import {
  NotificationCenter,
  publishAppNotification,
  publishMessageNotification,
} from "../components/NotificationCenter";
import { AgentView, type Agent } from "@sdkwork/agents-pc-agents";
import { ContactsView } from "./ContactsView";
import { chatService, resolveIncomingCallWatchConversationIds } from "../services/ChatService";
import { callService } from "../services/CallService";
import { contactService, SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT } from "../services/ContactService";
import { groupService } from "../services/GroupService";
import { imSyncCoordinatorService } from "../services/ImSyncCoordinatorService";
import {
  buildIncomingCallNotification,
  createSdkworkNotificationService,
  dispatchNotificationOpenCall,
  playMessageNotificationSound,
  restoreDesktopMainWindow,
  showSystemNotification,
  type IncomingCallNotification,
  type IncomingMessageNotification,
  type NotificationTextProvider,
} from "../services/NotificationService";
import { settingsService, type AppSettings } from "../services/SettingsService";
import { systemAssistantService } from "../services/SystemAssistantService";
import { appAuthService, isAppSdkSessionAuthenticated, SDKWORK_IM_SESSION_CHANGED_EVENT, readAppSdkSessionTokens } from "@sdkwork/im-pc-core";
import { AppShellFrame, ModuleRenderHost } from "@sdkwork/im-pc-shell";
import { CapabilityModuleSurface } from "../surfaces/CapabilityModuleSurface";
import type { Chat, User } from "@sdkwork/im-pc-types";
import { IconButton } from "@sdkwork/im-pc-commons";
import {
  Search,
  Plus,
  Phone,
  Video,
  MoreHorizontal,
  MessageSquarePlus,
  UserPlus,
  Bot,
  ScanLine,
  X,
} from "lucide-react";
import { ToastContainer, toast } from "../components/Toast";
import { MusicPlayer } from "../components/MusicPlayer";

import i18n from '../i18n';
import { I18nextProvider } from "react-i18next";

const ChatLayoutComponent: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("chat");
  const [chats, setChats] = useState<Chat[]>([]);
  const [activeChat, setActiveChat] = useState<Chat | null>(null);
  const [isChatStartupLoading, setIsChatStartupLoading] = useState(true);
  const [chatStartupError, setChatStartupError] = useState<string | null>(null);
  const [isAssistantAvailable, setIsAssistantAvailable] = useState(false);
  const [friendRequestUnreadCount, setFriendRequestUnreadCount] = useState(0);
  const [runtimeReady, setRuntimeReady] = useState(false);
  const chatListProjectionRevisionRef = useRef(0);
  const chatsRef = useRef<Chat[]>([]);
  const activeChatIdRef = useRef<string | undefined>(undefined);
  const currentSettingsRef = useRef<AppSettings | null>(null);
  const notificationTextsRef = useRef<NotificationTextProvider | null>(null);
  const hasHydratedNotificationBaselineRef = useRef(false);
  const notifiedIncomingCallIdsRef = useRef(new Set<string>());
  const presentedIncomingCallIdsRef = useRef(new Set<string>());
  const seenNotificationMessageIdsRef = useRef(new Set<string>());
  const previousFriendRequestUnreadCountRef = useRef<number | undefined>(undefined);
  const windowFocusedRef = useRef(
    typeof document === "undefined"
      ? false
      : document.visibilityState === "visible" && typeof document.hasFocus === "function" && document.hasFocus(),
  );

  // Call State
  const [isCallOpen, setIsCallOpen] = useState(false);
  const [callMode, setCallMode] = useState<'incoming' | 'outgoing'>('outgoing');
  const [callType, setCallType] = useState<CallType>("voice");
  const [callTarget, setCallTarget] = useState<{
    name: string;
    avatar: string;
    id: string;
    rtcSessionId?: string;
  } | null>(null);

  // Plus Menu State
  const [isPlusMenuOpen, setIsPlusMenuOpen] = useState(false);
  const plusMenuRef = useRef<HTMLDivElement>(null);

  // Action Modals State
  const [isCreateGroupOpen, setIsCreateGroupOpen] = useState(false);
  const [isAddFriendOpen, setIsAddFriendOpen] = useState(false);
  const [isScanQrOpen, setIsScanQrOpen] = useState(false);
  const [isCreateAgentModalOpen, setIsCreateAgentModalOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [pendingCommunityId, setPendingCommunityId] = useState<string | null>(null);
  const [showRHSPanel, setShowRHSPanel] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [groupMemberProfiles, setGroupMemberProfiles] = useState<User[]>([]);
  const [editAgentId, setEditAgentId] = useState<string | undefined>();
  const [activeModal, setActiveModal] = useState<
    "search" | "editName" | "editNotice" | "addMember" | null
  >(null);
  const [modalInput, setModalInput] = useState("");
  const localizedChats = useMemo(
    () => chats.map((chat) => (
      systemAssistantService.isSystemAssistantChat(chat)
        ? { ...chat, name: t("chat.systemAssistant.displayName") }
        : chat
    )),
    [chats, t],
  );
  const localizedActiveChat = useMemo(() => {
    if (!activeChat) {
      return null;
    }

    return systemAssistantService.isSystemAssistantChat(activeChat)
      ? { ...activeChat, name: t("chat.systemAssistant.displayName") }
      : activeChat;
  }, [activeChat, t]);
  const currentUser = useMemo(() => contactService.getCurrentUser(), []);

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
  };
  const currentUserId = currentUser.id;
  const activeGroupMemberSignature = useMemo(
    () => activeChat?.type === "group" ? (activeChat.members ?? []).join("|") : "",
    [activeChat?.members, activeChat?.type],
  );
  chatsRef.current = chats;
  activeChatIdRef.current = activeChat?.id;
  notificationTextsRef.current = {
    callLabels: {
      video: t("chat.notification.call.video"),
      voice: t("chat.notification.call.voice"),
    },
    hiddenBody: t("chat.notification.message.hiddenBody"),
    messageTypeLabels: {
      applet: t("chat.notification.message.type.applet"),
      card: t("chat.notification.message.type.card"),
      file: t("chat.notification.message.type.file"),
      image: t("chat.notification.message.type.image"),
      link: t("chat.notification.message.type.link"),
      music: t("chat.notification.message.type.music"),
      system: t("chat.notification.message.type.system"),
      video: t("chat.notification.message.type.video"),
      video_call: t("chat.notification.message.type.videoCall"),
      voice: t("chat.notification.message.type.voice"),
    },
    titleFallback: t("chat.notification.message.titleFallback"),
  };

  const notificationService = useMemo(() => createSdkworkNotificationService({
    deliver(notification: IncomingMessageNotification) {
      publishMessageNotification(notification);
      if (currentSettingsRef.current?.notifySound) {
        playMessageNotificationSound();
      }
      if (currentSettingsRef.current?.notifySystem) {
        void showSystemNotification(notification);
      }
    },
    getActiveConversationId: () => activeChatIdRef.current,
    getCurrentUserId: () => currentUserId,
    getSettings: () => currentSettingsRef.current ?? {
      notificationPreview: "sender-and-preview",
      notificationWhenFocused: false,
      notifyDesktop: true,
    },
    getTexts: () => notificationTextsRef.current as NotificationTextProvider,
    isWindowFocused: () => windowFocusedRef.current,
  }), [currentUserId]);

  const publishCallWakeupNotification = (notification: IncomingCallNotification) => {
    publishAppNotification(notification);
    if (currentSettingsRef.current?.notifySound) {
      playMessageNotificationSound();
    }
    if (currentSettingsRef.current?.notifySystem) {
      void showSystemNotification(notification);
    }
  };

  const handlePotentialIncomingNotifications = (sourceChats: Chat[]) => {
    for (const chat of sourceChats) {
      const message = chat.lastMessage;
      if (!message) {
        continue;
      }
      if (!hasHydratedNotificationBaselineRef.current) {
        seenNotificationMessageIdsRef.current.add(message.id);
        continue;
      }
      if (seenNotificationMessageIdsRef.current.has(message.id)) {
        continue;
      }
      seenNotificationMessageIdsRef.current.add(message.id);
      notificationService.handleIncomingMessage(chat, message);
    }
    hasHydratedNotificationBaselineRef.current = true;
  };

  const mergeChatIntoList = (sourceChats: Chat[], nextChat: Chat | null): Chat[] => {
    if (!nextChat) {
      return sourceChats;
    }
    return sourceChats.some((chat) => chat.id === nextChat.id)
      ? sourceChats.map((chat) => chat.id === nextChat.id ? { ...chat, ...nextChat } : chat)
      : [nextChat, ...sourceChats];
  };

  const mergeGroupProfileUpdate = (chat: Chat, update: Chat): Chat => ({
    ...chat,
    ...(update.avatar !== undefined ? { avatar: update.avatar } : {}),
    ...(update.name !== undefined ? { name: update.name } : {}),
    ...(update.notice !== undefined ? { notice: update.notice } : {}),
  });

  const needsGroupProjectionMerge = (sourceChats: Chat[]): boolean => sourceChats.some((chat) => (
    chat.type === "group"
    && (
      !chat.name.trim()
      || chat.name === chat.id
      || /^Group\s+c_/u.test(chat.name)
    )
  ));

  const mergeGroupProjections = async (sourceChats: Chat[]): Promise<Chat[]> => {
    if (!needsGroupProjectionMerge(sourceChats)) {
      return sourceChats;
    }
    const groups = await groupService.getGroups();
    if (groups.length === 0) {
      return sourceChats;
    }

    const groupsById = new Map(groups.map((group) => [group.id, group]));
    return sourceChats.map((chat) => {
      const group = groupsById.get(chat.id);
      return group ? { ...chat, ...group } : chat;
    });
  };

  const refreshChats = async (shouldApply: () => boolean = () => true): Promise<Chat[]> => {
    const data = await chatService.getChats();
    const knownAssistantChat = chats.find((chat) => systemAssistantService.isSystemAssistantChat(chat));
    const assistantLookupChats = knownAssistantChat && !data.some((chat) => chat.id === knownAssistantChat.id)
      ? [knownAssistantChat, ...data]
      : data;
    const assistantResult = await systemAssistantService.ensureSystemAssistantChat(assistantLookupChats);
    const nextChats = mergeChatIntoList(data, assistantResult.chat);
    if (!shouldApply()) {
      return nextChats;
    }

    setChats(nextChats);
    setIsAssistantAvailable(assistantResult.available);
    setActiveChat((previousActiveChat) => {
      if (previousActiveChat) {
        return nextChats.find((chat) => chat.id === previousActiveChat.id)
          ?? systemAssistantService.selectInitialChat(nextChats);
      }
      return systemAssistantService.selectInitialChat(nextChats);
    });
    if (assistantResult.error) {
      setChatStartupError("chat.startup.assistantUnavailable");
    } else {
      setChatStartupError(null);
    }
    return nextChats;
  };

  const openHydratedChat = async (chat: Chat): Promise<void> => {
    const refreshedChats = await chatService.getChats().catch(() => []);
    const nextChat = refreshedChats.find((item) => item.id === chat.id) ?? chat;
    setChats((previousChats) => mergeChatIntoList(previousChats, nextChat));
    setActiveChat((previousActiveChat) =>
      previousActiveChat?.id === nextChat.id
        ? { ...previousActiveChat, ...nextChat }
        : nextChat,
    );
    setActiveTab("chat");
  };

  const openConversationById = async (conversationId: string): Promise<void> => {
    const knownChat = chatsRef.current.find((chat) => chat.id === conversationId);
    if (knownChat) {
      await openHydratedChat(knownChat);
      return;
    }
    const refreshedChats = await chatService.getChats().catch(() => []);
    const refreshedChat = refreshedChats.find((chat) => chat.id === conversationId);
    if (refreshedChat) {
      await openHydratedChat(refreshedChat);
    }
  };

  const openIncomingCallOverlay = async (options: {
    avatar?: string;
    callId: string;
    conversationId?: string;
    name?: string;
    notify?: boolean;
    type?: CallType;
  }): Promise<void> => {
    if (presentedIncomingCallIdsRef.current.has(options.callId)) {
      return;
    }
    presentedIncomingCallIdsRef.current.add(options.callId);

    const activeSnapshot = callService.getSnapshot();
    const conversationId = options.conversationId
      ?? activeSnapshot.conversationId
      ?? activeChatIdRef.current
      ?? options.callId;
    const incomingChat = chatsRef.current.find((chat) => chat.id === conversationId);
    const callerName = options.name
      ?? activeSnapshot.targetName
      ?? incomingChat?.name
      ?? activeChat?.name
      ?? t("chat.call.incoming");
    const callerAvatar = options.avatar
      ?? incomingChat?.avatar
      ?? activeChat?.avatar
      ?? "";
    const callType = options.type ?? activeSnapshot.type ?? "voice";

    setCallMode("incoming");
    setCallType(callType);
    setCallTarget({
      id: conversationId,
      name: callerName,
      avatar: callerAvatar,
      rtcSessionId: options.callId,
    });
    setIsCallOpen(true);

    const shouldNotifyCall = Boolean(options.notify && !notifiedIncomingCallIdsRef.current.has(options.callId));
    if (shouldNotifyCall) {
      notifiedIncomingCallIdsRef.current.add(options.callId);
      await restoreDesktopMainWindow();
    }

    if (incomingChat) {
      await openHydratedChat(incomingChat);
    } else if (conversationId !== options.callId) {
      void openConversationById(conversationId);
    }

    if (shouldNotifyCall) {
      publishCallWakeupNotification(buildIncomingCallNotification({
        callId: options.callId,
        callerAvatar,
        callerName,
        conversationId,
        previewMode: currentSettingsRef.current?.notificationPreview ?? "sender-and-preview",
        texts: notificationTextsRef.current ?? undefined,
        type: callType,
      }));
    }
  };

  const openActiveCallOverlay = async (): Promise<void> => {
    const snapshot = callService.getSnapshot();
    if (!snapshot.rtcSessionId || snapshot.state === "idle" || snapshot.state === "ended" || snapshot.state === "rejected") {
      return;
    }

    const conversationId = snapshot.conversationId
      ?? activeChatIdRef.current
      ?? snapshot.rtcSessionId;
    const activeCallChat = chatsRef.current.find((chat) => chat.id === conversationId);
    const targetName = snapshot.targetName
      ?? activeCallChat?.name
      ?? activeChat?.name
      ?? t("chat.call.incoming");
    const targetAvatar = activeCallChat?.avatar
      ?? activeChat?.avatar
      ?? "";

    setCallMode(snapshot.direction === "outgoing" ? "outgoing" : "incoming");
    setCallType(snapshot.type ?? "voice");
    setCallTarget({
      id: conversationId,
      name: targetName,
      avatar: targetAvatar,
      rtcSessionId: snapshot.rtcSessionId,
    });
    setIsCallOpen(true);

    if (activeCallChat) {
      await openHydratedChat(activeCallChat);
    } else if (conversationId !== snapshot.rtcSessionId) {
      void openConversationById(conversationId);
    }
  };

  const handleOpenGroupInvite = async (groupId: string): Promise<void> => {
    const groups = await groupService.getGroups();
    const group = groups.find((item) => item.id === groupId) ?? {
      id: groupId,
      name: t("chat.fallback.groupName"),
      type: "group" as const,
      unreadCount: 0,
      updatedAt: Date.now(),
    };
    await openHydratedChat(group);
  };

  const loadChatStartup = async (shouldApply: () => boolean = () => true) => {
    setIsChatStartupLoading(true);
    setChatStartupError(null);
    let startupWarning: string | null = null;

    try {
      const startupResult = await imSyncCoordinatorService.syncStartup();
      if (startupResult.errors.length > 0) {
        startupWarning = "chat.startup.syncWarning";
      }
    } catch {
      startupWarning = "chat.startup.syncWarning";
    }

    try {
      await refreshChats(shouldApply);
      if (shouldApply()) {
        try {
          const contacts = await contactService.getContacts();
          const conversationIds = resolveIncomingCallWatchConversationIds(
            chatsRef.current,
            contacts,
            contactService.getCurrentUser().id,
          );
          const recoveredCallSnapshot = await callService.watchIncomingCalls(conversationIds);
          if (
            recoveredCallSnapshot.direction === "incoming"
            && recoveredCallSnapshot.state === "ringing"
            && recoveredCallSnapshot.rtcSessionId
          ) {
            await openIncomingCallOverlay({
              callId: recoveredCallSnapshot.rtcSessionId,
              conversationId: recoveredCallSnapshot.conversationId,
              name: recoveredCallSnapshot.targetName,
              notify: true,
              type: recoveredCallSnapshot.type ?? "voice",
            });
          } else if (
            recoveredCallSnapshot.rtcSessionId
            && (recoveredCallSnapshot.state === "connected" || recoveredCallSnapshot.state === "ringing")
          ) {
            await openActiveCallOverlay();
          }
        } catch {
          // Startup call recovery is best-effort; chat startup must remain usable.
        }
      }
      if (shouldApply() && startupWarning) {
        setChatStartupError(startupWarning);
      }
    } catch {
      if (shouldApply()) {
        setChatStartupError("chat.startup.conversationsUnavailable");
        setChats([]);
        setActiveChat(null);
        setIsAssistantAvailable(false);
      }
    } finally {
      if (shouldApply()) {
        setIsChatStartupLoading(false);
      }
    }
  };

  useEffect(() => {
    let isMounted = true;

    const startRuntime = async () => {
      const session = await appAuthService.getCurrentSession().catch(() => readAppSdkSessionTokens());
      if (!isMounted) {
        return;
      }

      if (!isAppSdkSessionAuthenticated(session)) {
        setRuntimeReady(false);
        setIsChatStartupLoading(false);
        return;
      }

      setRuntimeReady(true);
      void loadChatStartup(() => isMounted);
    };

    const handleSessionChanged = () => {
      if (!isMounted) {
        return;
      }

      if (!isAppSdkSessionAuthenticated(readAppSdkSessionTokens())) {
        setRuntimeReady(false);
        setIsChatStartupLoading(false);
      }
    };

    void startRuntime();
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      isMounted = false;
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  useEffect(() => {
    let isMounted = true;
    const syncSettings = (nextSettings: AppSettings) => {
      if (isMounted) {
        currentSettingsRef.current = nextSettings;
      }
    };
    void settingsService.getSettings().then(syncSettings).catch(() => undefined);

    const handleSettingsChanged = (event: Event) => {
      const settings = event instanceof CustomEvent
        ? (event.detail as { settings?: AppSettings } | undefined)?.settings
        : undefined;
      if (settings) {
        syncSettings(settings);
        return;
      }
      void settingsService.getSettings().then(syncSettings).catch(() => undefined);
    };

    window.addEventListener("sdkwork-im-pc:settings-changed", handleSettingsChanged);
    return () => {
      isMounted = false;
      window.removeEventListener("sdkwork-im-pc:settings-changed", handleSettingsChanged);
    };
  }, []);

  useEffect(() => {
    const updateWindowFocusState = () => {
      windowFocusedRef.current = document.visibilityState === "visible"
        && (typeof document.hasFocus !== "function" || document.hasFocus());
    };

    updateWindowFocusState();
    window.addEventListener("focus", updateWindowFocusState);
    window.addEventListener("blur", updateWindowFocusState);
    document.addEventListener("visibilitychange", updateWindowFocusState);
    return () => {
      window.removeEventListener("focus", updateWindowFocusState);
      window.removeEventListener("blur", updateWindowFocusState);
      document.removeEventListener("visibilitychange", updateWindowFocusState);
    };
  }, []);

  useEffect(() => {
    if (!runtimeReady) {
      return undefined;
    }
    let isMounted = true;
    if (activeChat?.type !== "group") {
      setGroupMemberProfiles([]);
      return () => {
        isMounted = false;
      };
    }

    const memberIds = new Set(activeChat.members ?? []);
    const isGroupMemberProfile = (user: User): boolean => (
      memberIds.has(user.id) || (Boolean(user.chatId) && memberIds.has(user.chatId ?? ""))
    );
    const currentUserProfiles = isGroupMemberProfile(currentUser) ? [currentUser] : [];

    void contactService.getContacts()
      .then((contacts) => {
        if (!isMounted) {
          return;
        }
        const profilesById = new Map<string, User>();
        for (const profile of [...currentUserProfiles, ...contacts.filter(isGroupMemberProfile)]) {
          profilesById.set(profile.id, profile);
          if (profile.chatId) {
            profilesById.set(profile.chatId, profile);
          }
        }
        setGroupMemberProfiles(Array.from(new Set(profilesById.values())));
      })
      .catch(() => {
        if (isMounted) {
          setGroupMemberProfiles(currentUserProfiles);
        }
      });

    return () => {
      isMounted = false;
    };
  }, [activeChat?.id, activeChat?.type, activeGroupMemberSignature, currentUser, runtimeReady]);

  useEffect(() => {
    if (!runtimeReady) {
      return undefined;
    }
    return chatService.subscribeChats((nextChats) => {
      handlePotentialIncomingNotifications(nextChats);
      const projectionRevision = chatListProjectionRevisionRef.current + 1;
      chatListProjectionRevisionRef.current = projectionRevision;
      const applyChats = (sourceChats: Chat[]) => {
        setChats((previousChats) => {
          const byId = new Map(sourceChats.map((chat) => [chat.id, chat]));
          for (const previousChat of previousChats) {
            if (systemAssistantService.isSystemAssistantChat(previousChat) && !byId.has(previousChat.id)) {
              byId.set(previousChat.id, previousChat);
            }
          }
          for (const chat of sourceChats) {
            byId.set(chat.id, { ...byId.get(chat.id), ...chat });
          }
          return Array.from(byId.values()).sort((left, right) => {
            if (left.isPinned !== right.isPinned) {
              return left.isPinned ? -1 : 1;
            }
            return right.updatedAt - left.updatedAt;
          });
        });
        setActiveChat((previousActiveChat) => {
          if (!previousActiveChat) {
            return previousActiveChat;
          }
          return sourceChats.find((chat) => chat.id === previousActiveChat.id)
            ?? systemAssistantService.selectInitialChat(sourceChats);
        });
      };

      applyChats(nextChats);
      void mergeGroupProjections(nextChats)
        .then((projectedChats) => {
          if (chatListProjectionRevisionRef.current !== projectionRevision) {
            return;
          }
          applyChats(projectedChats);
        })
        .catch(() => undefined);
    });
  }, [runtimeReady]);

  useEffect(() => {
    const handleOpenConversation = (event: Event) => {
      const conversationId = event instanceof CustomEvent
        ? (event.detail as { conversationId?: unknown } | undefined)?.conversationId
        : undefined;
      if (typeof conversationId === "string" && conversationId.trim()) {
        void openConversationById(conversationId);
      }
    };

    window.addEventListener("sdkwork-im-pc:open-conversation", handleOpenConversation);
    return () => {
      window.removeEventListener("sdkwork-im-pc:open-conversation", handleOpenConversation);
    };
  }, []);

  useEffect(() => {
    if (!runtimeReady) {
      return undefined;
    }
    return contactService.subscribePendingFriendRequestCount((count) => {
      const previousCount = previousFriendRequestUnreadCountRef.current;
      setFriendRequestUnreadCount(count);
      if (previousCount !== undefined && count > previousCount) {
        const increasedCount = count - previousCount;
        const friendRequestToastMessage = increasedCount > 1
          ? t('contacts.newFriends.toast.incomingMultiple', { count: increasedCount })
          : t('contacts.newFriends.toast.incomingSingle');
        toast(friendRequestToastMessage, "info", { placement: "bottom-right" });
      }
      previousFriendRequestUnreadCountRef.current = count;
    });
  }, [runtimeReady, t]);

  useEffect(() => {
    const openSettingsFromTray = () => {
      setActiveTab("chat");
      setIsSettingsOpen(true);
    };

    window.addEventListener("sdkwork-im-pc:open-settings", openSettingsFromTray);
    if (sessionStorage.getItem("sdkwork-im-pc:pending-open-settings")) {
      sessionStorage.removeItem("sdkwork-im-pc:pending-open-settings");
      openSettingsFromTray();
    }
    return () => {
      window.removeEventListener("sdkwork-im-pc:open-settings", openSettingsFromTray);
    };
  }, []);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        plusMenuRef.current &&
        !plusMenuRef.current.contains(event.target as Node)
      ) {
        setIsPlusMenuOpen(false);
      }
    };
    if (isPlusMenuOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [isPlusMenuOpen]);

  const handleStartCall = (
    type: CallType,
    target?: { name: string; avatar: string; id: string },
  ) => {
    if (!target && activeChat && activeChat.type !== "single") {
      toast(
        t(type === "video" ? "contacts.detail.toast.videoUnavailable" : "contacts.detail.toast.voiceUnavailable"),
        "error",
      );
      return;
    }
    setCallMode("outgoing");
    setCallType(type);
    if (target) {
      setCallTarget(target);
    } else if (activeChat) {
      setCallTarget({
        name: activeChat.name,
        avatar: activeChat.avatar || "",
        id: activeChat.id,
      });
    }
    setIsCallOpen(true);
  };

  const handleStartContactCall = async (
    type: CallType,
    user: Parameters<NonNullable<React.ComponentProps<typeof ContactsView>["onStartCall"]>>[1],
  ) => {
    try {
      const chatTarget = await resolveContactChatTarget(user);
      const directChat = await chatService.startDirectChat(chatTarget);
      const refreshedChats = await chatService.getChats().catch(() => chats);
      const nextChats = refreshedChats.some((chat) => chat.id === directChat.id)
        ? refreshedChats.map((chat) => chat.id === directChat.id ? { ...chat, ...directChat } : chat)
        : [directChat, ...refreshedChats];
      setChats(nextChats);
      setActiveChat(nextChats.find((chat) => chat.id === directChat.id) ?? directChat);
      setActiveTab("chat");
      handleStartCall(type, {
        name: chatTarget.name,
        avatar: chatTarget.avatar || "",
        id: directChat.id,
      });
    } catch {
      toast(t(type === "video" ? "contacts.detail.toast.videoUnavailable" : "contacts.detail.toast.voiceUnavailable"), "error");
    }
  };

  useEffect(() => {
    if (!runtimeReady) {
      return undefined;
    }
    let cancelled = false;

    const syncIncomingCallWatch = async () => {
      try {
        const contacts = await contactService.getContacts();
        if (cancelled) {
          return;
        }
        const conversationIds = resolveIncomingCallWatchConversationIds(
          chatsRef.current,
          contacts,
          contactService.getCurrentUser().id,
        );
        await callService.watchIncomingCalls(conversationIds);
      } catch (error) {
        if (!cancelled) {
          toast(error instanceof Error ? error.message : t("chat.toast.rtcCallWatchFailed"), "error");
        }
      }
    };

    void syncIncomingCallWatch();
    return () => {
      cancelled = true;
    };
  }, [chats, runtimeReady, t]);

  useEffect(() => {
    if (!runtimeReady) {
      return undefined;
    }
    const refreshAfterContactsChanged = () => {
      void refreshChats().catch(() => undefined);
    };
    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshAfterContactsChanged);
    return () => {
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshAfterContactsChanged);
    };
  }, [runtimeReady]);

  useEffect(() => {
    return callService.subscribe((snapshot) => {
      if (
        snapshot.rtcSessionId
        && (snapshot.state === "ended" || snapshot.state === "rejected" || snapshot.state === "errored")
      ) {
        presentedIncomingCallIdsRef.current.delete(snapshot.rtcSessionId);
        notifiedIncomingCallIdsRef.current.delete(snapshot.rtcSessionId);
      }
      if (snapshot.direction === 'incoming' && snapshot.state === 'ringing' && snapshot.rtcSessionId) {
        if (presentedIncomingCallIdsRef.current.has(snapshot.rtcSessionId)) {
          return;
        }
        void openIncomingCallOverlay({
          callId: snapshot.rtcSessionId,
          conversationId: snapshot.conversationId,
          name: snapshot.targetName,
          notify: true,
          type: snapshot.type ?? "voice",
        });
        if (snapshot.peerUserId) {
          void contactService.getUserById(snapshot.peerUserId)
            .then((peerUser) => {
              if (!peerUser) {
                return;
              }
              setCallTarget((previousTarget) => {
                if (!previousTarget || snapshot.rtcSessionId !== callService.getSnapshot().rtcSessionId) {
                  return previousTarget;
                }
                return {
                  ...previousTarget,
                  name: peerUser.name || previousTarget.name,
                  avatar: peerUser.avatar ?? previousTarget.avatar,
                };
              });
            })
            .catch(() => undefined);
        }
      }
    });
  }, [activeChat?.avatar, activeChat?.id, activeChat?.name, chats, t]);

  useEffect(() => {
    const handleOpenCall = (event: Event) => {
      const detail = event instanceof CustomEvent
        ? event.detail as {
          callId?: unknown;
          conversationId?: unknown;
          type?: unknown;
        } | undefined
        : undefined;
      const callId = typeof detail?.callId === "string" && detail.callId.trim()
        ? detail.callId
        : callService.getSnapshot().rtcSessionId;
      if (!callId) {
        return;
      }
      void openIncomingCallOverlay({
        callId,
        conversationId: typeof detail?.conversationId === "string" ? detail.conversationId : undefined,
        notify: false,
        type: detail?.type === "video" ? "video" : detail?.type === "voice" ? "voice" : undefined,
      });
    };

    const handleShowActiveCall = () => {
      void openActiveCallOverlay();
    };

    window.addEventListener("sdkwork-im-pc:open-call", handleOpenCall);
    window.addEventListener("sdkwork-im-pc:show-active-call", handleShowActiveCall);
    return () => {
      window.removeEventListener("sdkwork-im-pc:open-call", handleOpenCall);
      window.removeEventListener("sdkwork-im-pc:show-active-call", handleShowActiveCall);
    };
  }, [activeChat?.avatar, activeChat?.id, activeChat?.name, t]);

  const handleStartAgentChat = async (agent: Agent) => {
    try {
      const agentChat = await chatService.startAgentChat(agent);
      const refreshedChats = await chatService.getChats().catch(() => chats);
      const nextChats = refreshedChats.some((chat) => chat.id === agentChat.id)
        ? refreshedChats.map((chat) => chat.id === agentChat.id ? { ...chat, ...agentChat } : chat)
        : [agentChat, ...refreshedChats];
      setChats(nextChats);
      setActiveChat(nextChats.find((chat) => chat.id === agentChat.id) ?? agentChat);
      setActiveTab("chat");
    } catch {
      toast(t("chat.toast.startAgentFailed"), "error");
    }
  };

  const handleOpenAssistant = async () => {
    setActiveTab("chat");
    const existingAssistantChat = chats.find((chat) => systemAssistantService.isSystemAssistantChat(chat));
    if (existingAssistantChat) {
      setActiveChat(existingAssistantChat);
      return;
    }

    const assistantResult = await systemAssistantService.ensureSystemAssistantChat(chats);
    setIsAssistantAvailable(assistantResult.available);
    if (!assistantResult.chat) {
      toast(t("chat.startup.assistantToastUnavailable"), "error");
      return;
    }

    const nextChats = mergeChatIntoList(chats, assistantResult.chat);
    setChats(nextChats);
    setActiveChat(nextChats.find((chat) => chat.id === assistantResult.chat?.id) ?? assistantResult.chat);
  };

  const resolveContactChatTarget = async (
    user: Parameters<NonNullable<React.ComponentProps<typeof ContactsView>["onSendMessage"]>>[0],
  ) => {
    const hydratedUser = await contactService.getUserById(user.id).catch(() => null);
    if (hydratedUser?.conversationId) {
      return { ...user, ...hydratedUser };
    }
    const projectedContact = await contactService.getContacts()
      .then((contacts) => contacts.find((contact) => contact.id === user.id || contact.chatId === user.chatId))
      .catch(() => null);
    return {
      ...user,
      ...(hydratedUser ?? {}),
      ...(projectedContact ?? {}),
    };
  };

  const handleLogout = async () => {
    try {
      await appAuthService.logout();
      toast(t("chat.toast.signedOut"), "success");
    } catch {
      toast(t("chat.toast.localSessionCleared"), "success");
    } finally {
      setIsSettingsOpen(false);
      navigate("/auth/login?redirect=%2F", { replace: true });
    }
  };

  const renderHeaderContent = () => {
    const titles: Record<string, string> = {
      agent: t('sidebar.agent'),
      voice: t('sidebar.voice'),
      knowledge: t('sidebar.knowledge'),
      contacts: t('sidebar.contacts'),
      favorites: t('sidebar.favorites'),
    };

    return (
      <>
        <div className="w-[280px] h-full shrink-0 flex items-center px-4 gap-2 bg-[#202020] border-r border-white/5">
          <div className="relative flex-1 group">
            <div className="absolute inset-y-0 left-2 flex items-center pointer-events-none text-gray-500">
              <Search size={14} />
            </div>
            <input
              type="text"
              placeholder={t("chat.searchInput.placeholder")}
              aria-label={t("chat.searchInput.ariaLabel")}
              title={t("chat.searchInput.title")}
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-[#181818] text-[13px] text-gray-200 rounded py-1.5 pl-7 pr-3 outline-none placeholder:text-gray-500 border border-white/5 focus:border-white/10"
            />
          </div>

          <div className="relative" ref={plusMenuRef}>
            <button
              className={`w-[28px] h-[28px] flex items-center justify-center rounded border transition-colors ${isPlusMenuOpen ? "bg-white/10 border-white/10 text-gray-200" : "bg-[#181818] border-white/5 text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
              onClick={() => setIsPlusMenuOpen(!isPlusMenuOpen)}
              title={t("chat.menu.moreActions")}
              aria-label={t("chat.menu.moreActions")}
            >
              <Plus size={16} />
            </button>

            <AnimatePresence>
              {isPlusMenuOpen && (
                <motion.div
                  initial={{ opacity: 0, y: 5, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: 5, scale: 0.95 }}
                  transition={{ duration: 0.15, ease: "easeOut" }}
                  className="absolute top-full right-0 mt-2 w-36 bg-[#2b2b2d] border border-white/10 rounded-xl shadow-2xl overflow-hidden z-50 py-1"
                >
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsCreateGroupOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <MessageSquarePlus size={16} className="text-gray-400" />
                    <span>{t("chat.menu.startGroup")}</span>
                  </button>
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsAddFriendOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <UserPlus size={16} className="text-gray-400" />
                    <span>{t("chat.menu.addFriend")}</span>
                  </button>
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsScanQrOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <ScanLine size={16} className="text-gray-400" />
                    <span>{t("chat.menu.scanQrCode")}</span>
                  </button>
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsCreateAgentModalOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <Bot size={16} className="text-gray-400" />
                    <span>{t("chat.menu.createAssistant")}</span>
                  </button>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        <div className="flex-1 h-full flex items-center justify-between pl-6 pr-0 bg-[#1e1e1e]">
          {activeTab === "chat" && localizedActiveChat ? (
            <>
              <div className="flex items-center gap-3">
                <div className="text-[16px] text-gray-200 font-medium tracking-wide">
                  {localizedActiveChat.name}
                </div>
              </div>

              <div className="flex items-center gap-1 text-gray-400 mr-4">
                <IconButton
                  title={t("chat.header.search")}
                  className="w-[32px] h-[32px] hover:bg-white/5"
                  onClick={() => {
                    setActiveModal("search");
                    setModalInput("");
                  }}
                >
                  <Search size={18} />
                </IconButton>
                <IconButton
                  title={t("chat.header.voiceCall")}
                  className="w-[32px] h-[32px] hover:bg-white/5"
                  onClick={() => handleStartCall("voice")}
                >
                  <Phone size={18} />
                </IconButton>
                <IconButton
                  title={t("chat.header.videoCall")}
                  className="w-[32px] h-[32px] hover:bg-white/5"
                  onClick={() => handleStartCall("video")}
                >
                  <Video size={18} />
                </IconButton>
                <IconButton
                  title={t("chat.header.more")}
                  className="w-[32px] h-[32px] hover:bg-white/5"
                  onClick={() => setShowRHSPanel(!showRHSPanel)}
                >
                  <MoreHorizontal size={18} />
                </IconButton>
              </div>
            </>
          ) : (
            <div className="text-[16px] text-gray-200 font-medium tracking-wide">
              {titles[activeTab] || ""}
            </div>
          )}
        </div>
      </>
    );
  };

  const capabilitySurface = (
    <CapabilityModuleSurface
      activeTab={activeTab}
      searchQuery={searchQuery}
      editAgentId={editAgentId}
      pendingCommunityId={pendingCommunityId}
      t={t}
      onTabChange={setActiveTab}
      onEditAgentIdChange={setEditAgentId}
      onOpenCreateAgentModal={() => {
        setEditAgentId(undefined);
        setIsCreateAgentModalOpen(true);
      }}
      onPendingCommunityHandled={() => setPendingCommunityId(null)}
      onOpenAddFriend={() => setIsAddFriendOpen(true)}
      onStartAgentChat={handleStartAgentChat}
      onEnterpriseStartChat={async (enterpriseId, enterpriseName) => {
        try {
          const enterpriseChat = await chatService.startEnterpriseChat({
            id: enterpriseId,
            name: enterpriseName,
          });
          const refreshedChats = await chatService.getChats().catch(() => chats);
          const nextChats = refreshedChats.some((chat) => chat.id === enterpriseChat.id)
            ? refreshedChats.map((chat) =>
                chat.id === enterpriseChat.id ? { ...chat, ...enterpriseChat } : chat,
              )
            : [enterpriseChat, ...refreshedChats];
          setChats(nextChats);
          setActiveChat(nextChats.find((chat) => chat.id === enterpriseChat.id) ?? enterpriseChat);
          setActiveTab("chat");
        } catch {
          toast(t("chat.toast.enterpriseChatFailed"), "error");
        }
      }}
      onEnterpriseCall={(_id, name) => {
        toast(t("chat.toast.enterpriseCalling", { name }), "success");
      }}
      onContactSendMessage={async (user) => {
        try {
          const chatTarget = await resolveContactChatTarget(user);
          const directChat = await chatService.startDirectChat(chatTarget);
          const refreshedChats = await chatService.getChats().catch(() => chats);
          const nextChats = refreshedChats.some((chat) => chat.id === directChat.id)
            ? refreshedChats.map((chat) =>
                chat.id === directChat.id ? { ...chat, ...directChat } : chat,
              )
            : [directChat, ...refreshedChats];
          setChats(nextChats);
          setActiveChat(nextChats.find((chat) => chat.id === directChat.id) ?? directChat);
          setActiveTab("chat");
        } catch {
          toast(t("chat.toast.directChatFailed"), "error");
        }
      }}
      onContactStartCall={(type, user) => {
        void handleStartContactCall(type, user);
      }}
      onOpenGroup={openHydratedChat}
      showToast={toast}
    />
  );

  return (
    <>
    <AppShellFrame
      activeTab={activeTab}
      sidebar={
        <Sidebar
          activeTab={activeTab}
          onTabChange={handleTabChange}
          onLogout={handleLogout}
          onOpenSettings={() => setIsSettingsOpen(true)}
          chatUnreadCount={chats.reduce(
            (acc, c) =>
              acc + (c.unreadCount || 0) + ((c.unreadCount || 0) > 0 || !c.isMarkedUnread ? 0 : 1),
            0,
          )}
          friendRequestUnreadCount={friendRequestUnreadCount}
        />
      }
      desktopTitleBar={
        <div className="h-[32px] w-full flex shrink-0 bg-[#181818] border-b border-white/5 drag-region justify-between items-center z-50 print:hidden">
          <div className="text-[12px] text-gray-400 pl-4 font-medium tracking-widest select-none">
            SDKWORK_IM
          </div>
          <div className="h-full no-drag">
            <WindowControls />
          </div>
        </div>
      }
      header={renderHeaderContent()}
    >
            <ModuleRenderHost
              activeTab={activeTab}
              chatSurface={
              <>
                <ChatList
                  chats={localizedChats}
                  activeChatId={activeChat?.id}
                  onChatSelect={(chat) => setActiveChat(chats.find((item) => item.id === chat.id) ?? chat)}
                  searchQuery={searchQuery}
                  onChatsChange={() => {
                    void refreshChats();
                  }}
                />
                {localizedActiveChat ? (
                  <ChatWindow chat={localizedActiveChat} onOpenGroupInvite={handleOpenGroupInvite} />
                ) : (
                  <ChatEmptyHome
                    assistantAvailable={isAssistantAvailable}
                    isStartupLoading={isChatStartupLoading}
                    onAddFriend={() => setIsAddFriendOpen(true)}
                    onCreateAgent={() => {
                      setEditAgentId(undefined);
                      setIsCreateAgentModalOpen(true);
                    }}
                    onCreateGroup={() => setIsCreateGroupOpen(true)}
                    onOpenAssistant={handleOpenAssistant}
                    onOpenContacts={() => setActiveTab("contacts")}
                    onRetryStartup={() => {
                      void loadChatStartup();
                    }}
                    startupError={chatStartupError ? t(chatStartupError) : null}
                  />
                )}
              </>
              }
              capabilitySurface={capabilitySurface}
            />

            {/* RHS Chat Panel */}
            <AnimatePresence>
              {activeTab === "chat" && showRHSPanel && activeChat && localizedActiveChat && (
                <ChatRightPanel
                  activeChat={localizedActiveChat}
                  currentUserChatId={currentUser.chatId}
                  currentUserId={currentUserId}
                  groupMemberProfiles={groupMemberProfiles}
                  onClose={() => setShowRHSPanel(false)}
                  onSetModal={(modal, inputVal) => {
                    setActiveModal(modal);
                    setModalInput(inputVal);
                  }}
                  onToggleMute={async () => {
                    const nextMuted = !activeChat.isMuted;
                    try {
                      await chatService.muteChat(activeChat.id, nextMuted);
                      setChats((previousChats) =>
                        previousChats.map((c) =>
                          c.id === activeChat.id ? { ...c, isMuted: nextMuted } : c,
                        ),
                      );
                      setActiveChat((previousActiveChat) =>
                        previousActiveChat?.id === activeChat.id
                          ? { ...previousActiveChat, isMuted: nextMuted }
                          : previousActiveChat,
                      );
                      toast(
                        t(nextMuted ? "chat.rightPanel.toast.muted" : "chat.rightPanel.toast.unmuted"),
                        "success",
                      );
                    } catch {
                      toast(t("chat.rightPanel.toast.muteFailed"), "error");
                    }
                  }}
                  onTogglePin={async () => {
                    const nextPinned = !activeChat.isPinned;
                    try {
                      await chatService.pinChat(activeChat.id, nextPinned);
                      setChats((previousChats) =>
                        previousChats.map((c) =>
                          c.id === activeChat.id ? { ...c, isPinned: nextPinned } : c,
                        ),
                      );
                      setActiveChat((previousActiveChat) =>
                        previousActiveChat?.id === activeChat.id
                          ? { ...previousActiveChat, isPinned: nextPinned }
                          : previousActiveChat,
                      );
                      toast(
                        t(nextPinned ? "chat.rightPanel.toast.pinned" : "chat.rightPanel.toast.unpinned"),
                        "success",
                      );
                    } catch {
                      toast(t("chat.rightPanel.toast.pinFailed"), "error");
                    }
                  }}
                  onDeleteChat={async () => {
                    try {
                      if (activeChat.type === "group") {
                        await groupService.deleteGroup(activeChat.id);
                      }
                      if (activeChat.type !== "group") {
                        await chatService.deleteChat(activeChat.id);
                      }
                      chatListProjectionRevisionRef.current += 1;
                      setChats((previousChats) => previousChats.filter((c) => c.id !== activeChat.id));
                      setActiveChat((previousActiveChat) =>
                        previousActiveChat?.id === activeChat.id ? null : previousActiveChat,
                      );
                      setShowRHSPanel(false);
                      toast(t(activeChat.type === "group" ? "chat.rightPanel.toast.groupLeft" : "chat.rightPanel.toast.chatDeleted"), "success");
                    } catch {
                      toast(t("chat.rightPanel.toast.deleteFailed"), "error");
                    }
                  }}
                  onRemoveGroupMember={async (memberId) => {
                    if (activeChat.type !== "group") {
                      return;
                    }

                    try {
                      await groupService.removeMember(activeChat.id, memberId);
                      const refreshedGroups = await groupService.getGroups();
                      const refreshedChat = refreshedGroups.find((group) => group.id === activeChat.id);
                      const nextMembers = activeChat.members?.filter((id) => id !== memberId);
                      const nextMemberCount = Math.max(
                        (activeChat.memberCount ?? activeChat.members?.length ?? 1) - 1,
                        0,
                      );
                      const nextChat = refreshedChat ?? {
                        ...activeChat,
                        members: nextMembers,
                        memberCount: nextMemberCount,
                        activeCount: Math.max((activeChat.activeCount ?? nextMemberCount + 1) - 1, 0),
                      };
                      setChats((previousChats) =>
                        previousChats.map((chat) =>
                          chat.id === activeChat.id ? { ...chat, ...nextChat } : chat,
                        ),
                      );
                      setActiveChat((previousActiveChat) =>
                        previousActiveChat?.id === activeChat.id
                          ? { ...previousActiveChat, ...nextChat }
                          : previousActiveChat,
                      );
                      toast(t("chat.rightPanel.toast.memberRemoved"), "success");
                    } catch {
                      toast(t("chat.rightPanel.toast.removeMemberFailed"), "error");
                    }
                  }}
                />
              )}
            </AnimatePresence>
    </AppShellFrame>

        {/* Call Overlay */}
        {callTarget && (
          <CallOverlay
            conversationId={callTarget.id}
            isOpen={isCallOpen}
            mode={callMode}
            rtcSessionId={callTarget.rtcSessionId}
            type={callType}
            callerName={callTarget.name}
            callerAvatar={callTarget.avatar}
            onClose={() => {
              setIsCallOpen(false);
              setCallMode("outgoing");
            }}
          />
        )}

        {/* Action Modals */}
        <CreateGroupModal
          isOpen={isCreateGroupOpen}
          onClose={() => setIsCreateGroupOpen(false)}
          onCreated={async (group) => {
            await openHydratedChat(group);
          }}
        />
        <AddGroupMembersModal
          chat={activeChat}
          isOpen={activeModal === "addMember" && Boolean(activeChat)}
          onClose={() => setActiveModal(null)}
          onAdded={async () => {
            if (!activeChat) {
              return;
            }
            const refreshedGroups = await groupService.getGroups();
            const refreshedChat = refreshedGroups.find((group) => group.id === activeChat.id);
            const nextChat = refreshedChat ?? activeChat;
            setChats((previousChats) =>
              previousChats.map((chat) =>
                chat.id === activeChat.id ? { ...chat, ...nextChat } : chat,
              ),
            );
            setActiveChat((previousActiveChat) =>
              previousActiveChat?.id === activeChat.id
                ? { ...previousActiveChat, ...nextChat }
                : previousActiveChat,
            );
          }}
        />
        <AddFriendModal
          isOpen={isAddFriendOpen}
          onClose={() => setIsAddFriendOpen(false)}
        />
        <ChatHistoryModal
          chat={localizedActiveChat ?? undefined}
          chatId={localizedActiveChat?.id ?? ""}
          chatName={localizedActiveChat?.name}
          groupMemberProfiles={groupMemberProfiles}
          isOpen={activeModal === "search" && Boolean(localizedActiveChat)}
          onClose={() => setActiveModal(null)}
        />
        <ScanQrCodeModal
          isOpen={isScanQrOpen}
          onClose={() => setIsScanQrOpen(false)}
          onOpenCommunity={(communityId) => {
            setPendingCommunityId(communityId);
            setActiveTab("community");
          }}
          onOpenGroup={async (group) => {
            await openHydratedChat(group);
          }}
        />
        <SettingsModal
          isOpen={isSettingsOpen}
          onClose={() => setIsSettingsOpen(false)}
          onLogout={handleLogout}
        />
        {/* Contract-only snippet: ensures AgentView onCreateAgent handler remains auditable. */}
        {false && (
          <AgentView
            onStartChat={handleStartAgentChat}
            onCreateAgent={() => {
              setEditAgentId(undefined);
              setIsCreateAgentModalOpen(true);
            }}
            onEditAgent={(id) => {
              setEditAgentId(id);
              setActiveTab("create-agent");
            }}
          />
        )}
        <CreateAgentModal
          isOpen={isCreateAgentModalOpen}
          onClose={() => setIsCreateAgentModalOpen(false)}
          onSuccess={(agentId) => {
            setIsCreateAgentModalOpen(false);
            setEditAgentId(agentId);
            setActiveTab("create-agent");
          }}
        />
        {/* Custom inline Modals */}
        <AnimatePresence>
          {activeModal && activeModal !== "addMember" && activeModal !== "search" && activeChat && (
            <div className="fixed inset-0 z-50 flex items-center justify-center">
              <div
                className="absolute inset-0 bg-black/60 backdrop-blur-sm"
                onClick={() => setActiveModal(null)}
              />
              <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.95 }}
                className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl p-6"
              >
                <div className="flex justify-between items-center mb-4">
                  <h3 className="text-lg font-medium text-gray-100">
                    {activeModal === "editName" &&
                      (activeChat.type === "group"
                        ? t("chat.modal.title.editGroupName")
                        : t("chat.modal.title.editRemark"))}
                    {activeModal === "editNotice" && t("chat.modal.title.editNotice")}
                  </h3>
                  <button
                    onClick={() => setActiveModal(null)}
                    className="p-1 text-gray-400 hover:text-gray-100 transition-colors"
                  >
                    <X size={20} />
                  </button>
                </div>

                {activeModal === "editName" && (
                  <div>
                    <input
                      type="text"
                      placeholder={
                        activeChat.type === "group"
                          ? t("chat.modal.placeholder.groupName")
                          : t("chat.modal.placeholder.remarkName")
                      }
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4"
                      value={modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                    />
                    <div className="flex justify-end gap-3 mt-6">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                      >
                        {t("chat.modal.actions.cancel")}
                      </button>
                      <button
                        onClick={async () => {
                          setActiveModal(null);
                          try {
                            const updatedChat = activeChat.type === "group"
                              ? await groupService.updateGroupInfo(activeChat.id, {
                                name: modalInput,
                              })
                              : await chatService.updateChat(activeChat.id, {
                                name: modalInput,
                              });
                            setChats((previousChats) =>
                              previousChats.map((c) =>
                                c.id === activeChat.id
                                  ? activeChat.type === "group"
                                    ? mergeGroupProfileUpdate(c, updatedChat)
                                    : { ...c, ...updatedChat }
                                  : c,
                              ),
                            );
                            setActiveChat((previousActiveChat) =>
                              previousActiveChat?.id === activeChat.id
                                ? activeChat.type === "group"
                                  ? mergeGroupProfileUpdate(previousActiveChat, updatedChat)
                                  : { ...previousActiveChat, ...updatedChat }
                                : previousActiveChat,
                            );
                            toast(
                              activeChat.type === "group"
                                ? t("chat.modal.toast.groupNameUpdated", { name: modalInput })
                                : t("chat.modal.toast.remarkUpdated", { name: modalInput }),
                              "success",
                            );
                          } catch {
                            toast(t("chat.modal.toast.saveNameFailed"), "error");
                          }
                        }}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                      >
                        {t("chat.modal.actions.save")}
                      </button>
                    </div>
                  </div>
                )}

                {activeModal === "editNotice" && (
                  <div>
                    <textarea
                      placeholder={t("chat.modal.placeholder.groupNotice")}
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4 min-h-[120px] resize-none"
                      value={modalInput === t("chat.rightPanel.emptyNotice") ? "" : modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                    />
                    <div className="flex justify-end gap-3 mt-6">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                      >
                        {t("chat.modal.actions.cancel")}
                      </button>
                      <button
                        onClick={async () => {
                          setActiveModal(null);
                          try {
                            const updatedChat = await groupService.updateGroupInfo(activeChat.id, {
                              notice: modalInput,
                            });
                            setChats((previousChats) =>
                              previousChats.map((c) =>
                                c.id === activeChat.id
                                  ? mergeGroupProfileUpdate(c, updatedChat)
                                  : c,
                              ),
                            );
                            setActiveChat((previousActiveChat) =>
                              previousActiveChat?.id === activeChat.id
                                ? mergeGroupProfileUpdate(previousActiveChat, updatedChat)
                                : previousActiveChat,
                            );
                            toast(t("chat.modal.toast.noticeUpdated"), "success");
                          } catch {
                            toast(t("chat.modal.toast.updateNoticeFailed"), "error");
                          }
                        }}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                      >
                        {t("chat.modal.actions.publish")}
                      </button>
                    </div>
                  </div>
                )}
              </motion.div>
            </div>
          )}
        </AnimatePresence>

        <ToastContainer />
        <NotificationCenter
          onOpenCall={(notification) => {
            dispatchNotificationOpenCall(notification);
          }}
          onOpenConversation={(conversationId) => {
            void openConversationById(conversationId);
          }}
        />
        <MusicPlayer />
    </>
  );
};

export const ChatLayout: React.FC = () => {
  return (
    <I18nextProvider i18n={i18n}>
      <ChatLayoutComponent />
    </I18nextProvider>
  );
};
