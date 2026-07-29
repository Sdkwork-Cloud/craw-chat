import React from 'react';
import { Play, Pause, X } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { useAudioStore } from '@sdkwork/im-h5-core';

export const GlobalMiniPlayer: React.FC = () => {
  const currentTrack = useAudioStore(s => s.currentTrack);
  const isPlaying = useAudioStore(s => s.isPlaying);
  const pause = useAudioStore(s => s.pause);
  const resume = useAudioStore(s => s.resume);
  const stop = useAudioStore(s => s.stop);

  if (!currentTrack) return null;

  return (
    <div className="absolute top-[80px] right-0 z-[100] flex items-center bg-bg-color/90 dark:bg-[#2c2c2e]/95 backdrop-blur-xl border border-r-0 border-border-color shadow-sm rounded-l-full py-1.5 pl-2 pr-1 shadow-[0_2px_8px_rgba(0,0,0,0.08)]">
      <div className={cn("w-8 h-8 rounded-full overflow-hidden shrink-0 border border-border-color/50 relative", isPlaying && "animate-spin")} style={{ animationDuration: '5s' }}>
        <img src={currentTrack.coverUrl} alt="Cover" className="w-full h-full object-cover" />
      </div>
      <button
        type="button"
        className="w-6 h-6 flex items-center justify-center shrink-0 ml-1 text-text-sub"
        onClick={(e) => {
          e.stopPropagation();
          isPlaying ? pause() : resume();
        }}
      >
        {isPlaying ? <Pause className="w-3.5 h-3.5 text-text-main fill-current" /> : <Play className="w-3.5 h-3.5 text-text-main fill-current ml-0.5" />}
      </button>
      <button
        type="button"
        className="w-6 h-6 flex items-center justify-center shrink-0 text-text-sub/60 hover:text-text-main hover:bg-chat-active-bg rounded-full transition-colors"
        onClick={(e) => {
          e.stopPropagation();
          stop();
        }}
      >
        <X className="w-3.5 h-3.5" />
      </button>
    </div>
  );
};
