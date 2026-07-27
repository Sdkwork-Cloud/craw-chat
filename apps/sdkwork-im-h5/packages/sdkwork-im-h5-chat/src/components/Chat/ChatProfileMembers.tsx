import React from "react";
import { Plus } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Avatar } from "@sdkwork/im-h5-commons";
import type { Chat } from "@sdkwork/im-h5-types";

interface ChatProfileMembersProps {
  chat: Chat | null;
  onAddMember: () => void;
}

export const ChatProfileMembers: React.FC<ChatProfileMembersProps> = ({
  chat,
  onAddMember,
}) => {
  const { t } = useTranslation();

  return (
    <div className="bg-chat-other-bg px-4 py-6 mb-2 sm:mb-4 sm:rounded-xl sm:mt-2 border-y sm:border border-border-color">
      <div className="flex flex-wrap gap-5">
        {chat?.participants.map((p) => (
          <div key={p.id} className="flex flex-col items-center gap-2 w-[52px]">
            <Avatar
              src={p.avatar}
              size="lg"
              className="w-[52px] h-[52px] rounded-xl"
            />
            <span className="text-[12px] text-text-sub truncate w-full text-center">
              {p.name}
            </span>
          </div>
        ))}
        <div className="flex flex-col items-center gap-2 w-[52px]">
          <div
            onClick={onAddMember}
            className="w-[52px] h-[52px] rounded-xl border border-dashed border-border-color flex items-center justify-center text-text-sub active:bg-active-bg cursor-pointer transition-colors"
          >
            <Plus className="w-6 h-6" />
          </div>
          <span className="text-[12px] text-text-sub truncate w-full text-center">
            {t("chat.profile.add")}
          </span>
        </div>
      </div>
    </div>
  );
};
