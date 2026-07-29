import React from "react";
import { Play, Pause, SkipBack, SkipForward, Heart } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface PlayerControlsProps {
  title: string;
  artist: string;
  isLiked?: boolean;
  onToggleLike?: () => void;
  progress: number;
  duration: number;
  isPlaying: boolean;
  onSeek: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onTogglePlay: () => void;
  onPrevious?: () => void;
  onNext?: () => void;
  formatTime: (secs: number) => string;
}

export const PlayerControls: React.FC<PlayerControlsProps> = ({
  title,
  artist,
  isLiked,
  onToggleLike,
  progress,
  duration,
  isPlaying,
  onSeek,
  onTogglePlay,
  onPrevious,
  onNext,
  formatTime,
}) => {
  return (
    <div className="w-full flex flex-col gap-6 mt-8">
      {/* Song Info */}
      <div className="flex items-center justify-between">
        <div className="flex flex-col min-w-0 pr-4">
          <h1 className="text-[24px] font-bold truncate">{title}</h1>
          <p className="text-[16px] text-white/70 truncate">{artist}</p>
        </div>
        {onToggleLike && typeof isLiked === "boolean" && (
          <IconButton
            icon={
              <Heart
                className={`w-7 h-7 ${
                  isLiked ? "fill-[#1ED760] text-[#1ED760]" : "text-white"
                }`}
              />
            }
            onClick={onToggleLike}
          />
        )}
      </div>

      {/* Progress */}
      <div className="flex flex-col gap-2">
        <input
          type="range"
          min="0"
          max={duration || 100}
          value={progress}
          onChange={onSeek}
          className="w-full h-1.5 bg-white/20 rounded-full appearance-none accent-white cursor-pointer"
        />
        <div className="flex justify-between text-[12px] text-white/50 font-medium">
          <span>{formatTime(progress)}</span>
          <span>{formatTime(duration)}</span>
        </div>
      </div>

      {/* Controls */}
      <div className="flex items-center justify-center gap-8 px-2">
        {onPrevious && (
          <IconButton
            icon={<SkipBack className="w-8 h-8 text-white fill-white" />}
            onClick={onPrevious}
          />
        )}
        <div
          className="w-16 h-16 rounded-full bg-white text-black flex items-center justify-center cursor-pointer active:scale-95 transition-transform"
          onClick={onTogglePlay}
        >
          {isPlaying ? (
            <Pause className="w-8 h-8 fill-black" />
          ) : (
            <Play className="w-8 h-8 fill-black ml-1" />
          )}
        </div>
        {onNext && (
          <IconButton
            icon={<SkipForward className="w-8 h-8 text-white fill-white" />}
            onClick={onNext}
          />
        )}
      </div>
    </div>
  );
};
