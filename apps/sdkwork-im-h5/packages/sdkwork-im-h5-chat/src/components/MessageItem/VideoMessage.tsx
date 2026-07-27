import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { Play } from "lucide-react";

export const VideoMessage = ({
  msg,
  onClick,
}: {
  msg: Message;
  onClick: () => void;
}) => (
  <div
    className="overflow-hidden rounded-lg relative cursor-pointer group w-[160px] h-[220px] bg-black/10 flex items-center justify-center shrink-0"
    onClick={onClick}
  >
    {msg.metadata?.coverUrl ? (
      <img
        src={msg.metadata.coverUrl}
        className="absolute inset-0 w-full h-full object-cover opacity-80 group-active:opacity-75 transition-opacity"
        alt="Video thumbnail"
      />
    ) : (
      <video
        src={msg.content}
        className="absolute inset-0 w-full h-full object-cover opacity-80 group-active:opacity-75 transition-opacity"
        preload="metadata"
      />
    )}
    <div className="absolute inset-0 flex items-center justify-center pointer-events-none z-10">
      <div className="w-12 h-12 bg-black/40 rounded-full flex items-center justify-center backdrop-blur-sm shadow-sm">
        <Play className="w-6 h-6 text-white ml-1" />
      </div>
    </div>
    {msg.metadata?.duration && (
      <span className="absolute bottom-2 right-2 text-white text-[12px] font-medium z-10 bg-black/40 px-1.5 rounded pointer-events-none">
        {msg.metadata?.duration}
      </span>
    )}
  </div>
);
