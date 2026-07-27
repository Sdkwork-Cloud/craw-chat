import React from "react";
import { Search, MessageSquare, User } from "lucide-react";
import { Avatar } from "@sdkwork/im-h5-commons";
import type { Chat, User as UserType } from "@sdkwork/im-h5-types";

interface GlobalSearchResultsProps {
  t: (key: string) => string;
  query: string;
  isSearching: boolean;
  contacts: UserType[];
  chats: Chat[];
  onSelectContact: (contactId: string) => void;
  onSelectChat: (chatId: string) => void;
}

export const GlobalSearchResults: React.FC<GlobalSearchResultsProps> = ({
  t,
  query,
  isSearching,
  contacts,
  chats,
  onSelectContact,
  onSelectChat,
}) => {
  return (
    <div className="p-4">
      {isSearching ? (
        <div className="flex justify-center p-4">
          <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
        </div>
      ) : (
        <div className="space-y-6">
          {contacts.length > 0 && (
            <div>
              <h3 className="text-[14px] text-text-sub mb-3 px-2 flex items-center gap-2">
                <User className="w-4 h-4" /> {t('chat.search.contacts')}
              </h3>
              <div className="bg-bg-color rounded-xl overflow-hidden">
                {contacts.map((contact) => (
                  <div
                    key={contact.id}
                    className="flex items-center gap-3 p-3 active:bg-active-bg cursor-pointer"
                    onClick={() => onSelectContact(contact.id)}
                  >
                    <Avatar
                      src={contact.avatar}
                      fallback={contact.name[0]}
                    />
                    <span className="text-[16px] text-text-main font-medium">
                      {contact.name}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {chats.length > 0 && (
            <div>
              <h3 className="text-[14px] text-text-sub mb-3 px-2 flex items-center gap-2">
                <MessageSquare className="w-4 h-4" /> {t('chat.search.chat_history')}
              </h3>
              <div className="bg-bg-color rounded-xl overflow-hidden">
                {chats.map((chat) => {
                  const chatName =
                    chat.type === "group"
                      ? chat.name
                      : chat.participants[0]?.name;
                  const avatar =
                    chat.type === "group"
                      ? chat.avatar
                      : chat.participants[0]?.avatar;
                  return (
                    <div
                      key={chat.id}
                      className="flex items-center gap-3 p-3 active:bg-active-bg cursor-pointer"
                      onClick={() => onSelectChat(chat.id)}
                    >
                      <Avatar src={avatar} fallback={chatName?.[0]} />
                      <span className="text-[16px] text-text-main font-medium">
                        {chatName}
                      </span>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {chats.length === 0 && contacts.length === 0 && (
            <div className="flex flex-col items-center justify-center py-10 text-text-sub">
              <Search className="w-10 h-10 mb-3 opacity-20" />
              <p className="text-[15px]">{t('chat.search.no_result_prefix')}{query}{t('chat.search.no_result_suffix')}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
