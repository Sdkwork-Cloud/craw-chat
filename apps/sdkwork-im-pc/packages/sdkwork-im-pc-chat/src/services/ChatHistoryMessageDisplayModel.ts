import type { Chat, Message, User } from '@sdkwork/im-pc-types';

export interface ChatHistoryResolvedSender {
  avatar: string | undefined;
  isCurrentUser: boolean;
  name: string;
}

export interface ResolveChatHistoryMessageSenderOptions {
  chat?: Pick<Chat, 'avatar' | 'id' | 'name' | 'type'>;
  currentUser?: Pick<User, 'avatar' | 'chatId' | 'id' | 'name'> | null;
  fallbackMemberName: string;
  message: Pick<Message, 'senderId'>;
  senderProfiles: Readonly<Record<string, Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>>>;
}

export function createChatHistorySenderProfileIndex(
  profiles: readonly Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>[],
  extraProfiles: Readonly<Record<string, Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>>> = {},
): Record<string, Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>> {
  const index: Record<string, Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>> = {};

  for (const profile of Object.values(extraProfiles)) {
    indexProfile(index, profile);
  }
  for (const profile of profiles) {
    indexProfile(index, profile);
  }

  return index;
}

export function resolveChatHistoryMessageSender(
  options: ResolveChatHistoryMessageSenderOptions,
): ChatHistoryResolvedSender {
  const { chat, currentUser, fallbackMemberName, message, senderProfiles } = options;
  const isCurrentUser = Boolean(
    currentUser
    && (
      message.senderId === 'me'
      || message.senderId === currentUser.id
      || (Boolean(currentUser.chatId) && message.senderId === currentUser.chatId)
    ),
  );

  if (isCurrentUser && currentUser) {
    return {
      avatar: currentUser.avatar,
      isCurrentUser: true,
      name: currentUser.name,
    };
  }

  const senderProfile = senderProfiles[message.senderId];
  if (senderProfile) {
    return {
      avatar: senderProfile.avatar,
      isCurrentUser: false,
      name: senderProfile.name,
    };
  }

  if (chat?.type === 'single') {
    return {
      avatar: chat.avatar,
      isCurrentUser: false,
      name: chat.name,
    };
  }

  return {
    avatar: undefined,
    isCurrentUser: false,
    name: fallbackMemberName,
  };
}

function indexProfile(
  index: Record<string, Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>>,
  profile: Pick<User, 'avatar' | 'chatId' | 'id' | 'name'>,
): void {
  index[profile.id] = profile;
  if (profile.chatId) {
    index[profile.chatId] = profile;
  }
}
