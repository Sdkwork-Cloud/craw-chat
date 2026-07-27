import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { Play } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useAudioStore } from "@sdkwork/im-h5-core";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

export const MusicMessage = ({
  msg,
  isMe,
}: {
  msg: Message;
  isMe: boolean;
}) => {
  const { t } = useTranslation();
const currentTrack = useAudioStore((s) => s.currentTrack);
  const isGlobalPlaying = useAudioStore((s) => s.isPlaying);
  const playMusic = useAudioStore((s) => s.playMusic);
  const pause = useAudioStore((s) => s.pause);
  const navigate = useNavigate();

  const isThisPlaying = currentTrack?.id === msg.id && isGlobalPlaying;

  const handlePlayClick = (e: React.MouseEvent) => {
  e.stopPropagation();
    if (isThisPlaying) {
      pause();
    } else {
      playMusic({
        id: msg.id,
        title: msg.metadata?.title || t('chat.date.unknown_song'),
        artist: msg.metadata?.artist || t('chat.date.unknown_artist'),
        coverUrl:
          msg.metadata?.coverUrl || "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/music/300x300.png",
        audioUrl:
          msg.content ||
          "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/audio/sample.mp3",
      });
      navigate("/music-player");
    }
  };

  return (
    <div
      className="flex items-center gap-3 min-w-[200px] cursor-pointer"
      onClick={handlePlayClick}
    >
      <div className="w-12 h-12 rounded-lg shrink-0 overflow-hidden relative border border-black/10 dark:border-white/10">
        <img
          src={
            msg.metadata?.coverUrl || "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/music/200x200.png"
          }
          className="w-full h-full object-cover"
        />
        <div className="absolute inset-0 bg-black/30 flex items-center justify-center">
          {isThisPlaying ? (
            <div className="w-4 h-4 bg-white/90 rounded-sm animate-pulse" />
          ) : (
            <Play className="w-6 h-6 text-white ml-0.5" />
          )}
        </div>
      </div>
      <div className="flex-1 flex flex-col min-w-0">
        <span
          className="text-[15px] font-bold truncate leading-tight mb-0.5"
          style={{ color: isMe ? "white" : "inherit" }}
        >
          {msg.metadata?.title || t('chat.date.unknown_song')}
        </span>
        <span
          className={cn(
            "text-[12px]",
            isMe ? "text-white/70" : "text-text-sub",
          )}
        >
          {msg.metadata?.artist || t('chat.date.unknown_artist')}
        </span>
      </div>
    </div>
  );
};
