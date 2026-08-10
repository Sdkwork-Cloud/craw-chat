import React from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, MoreHorizontal, Phone, Video } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import type { Chat } from "@sdkwork/im-h5-types";

interface ChatHeaderProps {
  chat: Chat | null;
  id?: string;
}

export const ChatHeader: React.FC<ChatHeaderProps> = ({ chat, id }) => {
  const navigate = useNavigate();

  return (
    <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
      <div className="flex items-center z-10 flex-1">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
      </div>

      <div className="absolute inset-x-0 flex items-center justify-center pointer-events-none">
        <h2 className="text-[17px] font-medium text-text-main">
          {chat?.type === "group"
            ? chat.name
            : chat?.participants[0]?.name || "Chat"}
        </h2>
      </div>

      <div className="flex items-center justify-end z-10 flex-1">
        {chat && !chat.participants.some((p) => p.id.startsWith("agent_")) && (
          <>
            <IconButton
              icon={<Phone className="w-5 h-5 text-text-main" />}
              onClick={() => navigate(`/call/voice/${id}`)}
            />
            <IconButton
              icon={<Video className="w-[22px] h-[22px] text-text-main" />}
              onClick={() => navigate(`/call/video/${id}`)}
            />
          </>
        )}
        <IconButton
          icon={<MoreHorizontal className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(`/chat/${id}/profile`)}
        />
      </div>
    </header>
  );
};
