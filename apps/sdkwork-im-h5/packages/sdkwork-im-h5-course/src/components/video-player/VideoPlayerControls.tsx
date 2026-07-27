import React from "react";
import { Play, Pause, Maximize } from "lucide-react";

export interface VideoPlayerControlsProps {
  isPlaying: boolean;
  isBuffering: boolean;
  currentTime: string;
  durationStr: string;
  speed: number;
  speeds: number[];
  showSpeedMenu: boolean;
  progress: number;
  isScrubbing: boolean;
  togglePlayPause: (e?: React.MouseEvent) => void;
  setShowSpeedMenu: (show: boolean) => void;
  setSpeed: (speed: number) => void;
  requestFullScreen: (e: React.MouseEvent) => void;
  handlePointerDown: (e: React.PointerEvent<HTMLDivElement>) => void;
  handlePointerMove: (e: React.PointerEvent<HTMLDivElement>) => void;
  handlePointerUp: (e: React.PointerEvent<HTMLDivElement>) => void;
}

export const VideoPlayerControls: React.FC<VideoPlayerControlsProps> = ({
  isPlaying,
  isBuffering,
  currentTime,
  durationStr,
  speed,
  speeds,
  showSpeedMenu,
  progress,
  isScrubbing,
  togglePlayPause,
  setShowSpeedMenu,
  setSpeed,
  requestFullScreen,
  handlePointerDown,
  handlePointerMove,
  handlePointerUp,
}) => {
  return (
    <>
      {/* Central Play/Pause Button */}
      {!isBuffering && (
        <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
          <div
            className="w-16 h-16 bg-black/40 hover:bg-black/60 backdrop-blur-md rounded-full flex items-center justify-center cursor-pointer pointer-events-auto transition-transform active:scale-95"
            onClick={togglePlayPause}
          >
            {isPlaying ? (
              <Pause className="w-8 h-8 text-white fill-white" />
            ) : (
              <Play className="w-8 h-8 text-white fill-white ml-1" />
            )}
          </div>
        </div>
      )}

      {/* Bottom Controls Bar */}
      <div className="relative flex flex-col gap-1 p-3 px-4 pointer-events-auto mt-auto">
        <div className="flex items-center gap-3">
          <div className="text-white text-[12px] font-mono shrink-0 select-none drop-shadow-md tracking-wider">
            {currentTime} / {durationStr}
          </div>
          <div className="flex-1" />

          {/* Speed selector */}
          <div className="relative">
            <div
              className="text-white/90 text-[13px] font-medium cursor-pointer px-2 py-1 hover:bg-white/10 rounded-lg transition-colors flex items-center justify-center min-w-[36px]"
              onClick={(e) => {
                e.stopPropagation();
                setShowSpeedMenu(!showSpeedMenu);
              }}
            >
              {speed}x
            </div>

            {showSpeedMenu && (
              <div className="absolute bottom-full right-0 mb-2 bg-black/80 backdrop-blur-md rounded-xl overflow-hidden flex flex-col min-w-[70px] shadow-lg border border-white/10">
                {speeds.map((s) => (
                  <div
                    key={s}
                    className={`px-4 py-2.5 text-center text-[13px] font-medium cursor-pointer transition-colors active:bg-white/10 ${
                      speed === s
                        ? "text-blue-400 bg-white/5"
                        : "text-white hover:bg-white/5"
                    }`}
                    onClick={(e) => {
                      e.stopPropagation();
                      setSpeed(s);
                      setShowSpeedMenu(false);
                    }}
                  >
                    {s}x
                  </div>
                ))}
              </div>
            )}
          </div>

          <div
            className="cursor-pointer p-1 hover:bg-white/10 rounded-lg transition-colors"
            onClick={requestFullScreen}
          >
            <Maximize className="w-5 h-5 text-white drop-shadow-md" />
          </div>
        </div>

        {/* Progress Bar Container */}
        <div
          className="h-8 flex items-center cursor-pointer group/bar relative -mx-2 px-2"
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerUp}
        >
          <div className="w-full h-1.5 bg-white/20 rounded-full relative transition-all group-hover/bar:h-2 group-active/bar:h-2">
            {/* Buffered bar */}
            <div
              className="absolute left-0 top-0 bottom-0 bg-white/30 rounded-full"
              style={{ width: `${Math.min(100, progress + 15)}%` }}
            />

            {/* Played bar */}
            <div
              className="absolute left-0 top-0 bottom-0 bg-blue-500 rounded-full"
              style={{ width: `${progress}%` }}
            />

            {/* Thumb */}
            <div
              className={`absolute top-1/2 -mt-2 -ml-2 w-4 h-4 bg-white rounded-full shadow-[0_0_8px_rgba(0,0,0,0.5)] transition-transform ${
                isScrubbing ? "scale-125" : "scale-0 group-hover/bar:scale-100"
              }`}
              style={{ left: `${progress}%` }}
            />
          </div>
        </div>
      </div>
    </>
  );
};
