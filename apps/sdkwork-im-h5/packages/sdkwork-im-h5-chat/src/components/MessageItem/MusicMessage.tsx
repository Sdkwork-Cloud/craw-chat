import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { Music2, Play } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useAudioStore } from "@sdkwork/music-mobile-react-playback";
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
  const audioUrl = msg.content.trim();
  const coverUrl = msg.metadata?.coverUrl?.trim();

  const isThisPlaying = currentTrack?.id === msg.id && isGlobalPlaying;

  const handlePlayClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!audioUrl) {
      return;
    }
    if (isThisPlaying) {
      pause();
    } else {
      playMusic({
        id: msg.id,
        title: msg.metadata?.title || t('chat.date.unknown_song'),
        artist: msg.metadata?.artist || t('chat.date.unknown_artist'),
        coverUrl: coverUrl ?? "",
        audioUrl,
      });
    }
  };

  return (
    <button
      type="button"
      aria-label={msg.metadata?.title || t('chat.date.unknown_song')}
      className="flex min-w-[200px] items-center gap-3 text-left disabled:cursor-not-allowed disabled:opacity-60"
      disabled={!audioUrl}
      onClick={handlePlayClick}
    >
      <div className="w-12 h-12 rounded-lg shrink-0 overflow-hidden relative border border-black/10 dark:border-white/10">
        {coverUrl ? (
          <img src={coverUrl} alt="" className="h-full w-full object-cover" />
        ) : (
          <div className="flex h-full w-full items-center justify-center bg-black/10 dark:bg-white/10">
            <Music2 className="h-6 w-6" aria-hidden="true" />
          </div>
        )}
        <div className="absolute inset-0 bg-black/30 flex items-center justify-center">
          {isThisPlaying ? (
            <div className="w-4 h-4 bg-white/90 rounded-sm animate-pulse" />
          ) : (
            <Play className="w-6 h-6 text-white ml-0.5" aria-hidden="true" />
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
    </button>
  );
};
