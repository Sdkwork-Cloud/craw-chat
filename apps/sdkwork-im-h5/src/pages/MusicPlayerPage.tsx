import React, { useState } from 'react';
import { useNavigate } from 'react-router';
import { useAudioStore } from '@sdkwork/im-h5-core';
import { PlayerHeader } from '../components/music-player/PlayerHeader';
import { PlayerCoverCard } from '../components/music-player/PlayerCoverCard';
import { PlayerControls } from '../components/music-player/PlayerControls';
import { PlayerActions } from '../components/music-player/PlayerActions';

export const MusicPlayerPage: React.FC = () => {
  const navigate = useNavigate();
  const currentTrack = useAudioStore(s => s.currentTrack);
  const isPlaying = useAudioStore(s => s.isPlaying);
  const progress = useAudioStore(s => s.progress);
  const duration = useAudioStore(s => s.duration);
  const pause = useAudioStore(s => s.pause);
  const resume = useAudioStore(s => s.resume);
  const seek = useAudioStore(s => s.seek);
  
  const [isLiked, setIsLiked] = useState(false);

  if (!currentTrack) {
    return (
      <div className="flex flex-col h-full bg-[#121212] items-center justify-center text-white">
        <p>暂无播放内容</p>
        <button onClick={() => navigate(-1)} className="mt-4 px-4 py-2 bg-white/10 rounded-full">返回</button>
      </div>
    );
  }

  const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60).toString().padStart(2, '0');
    const s = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${m}:${s}`;
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    seek(Number(e.target.value));
  };

  return (
    <div className="flex flex-col h-full bg-[#121212] text-white relative overflow-hidden">
      {/* Background Blur */}
      <div 
        className="absolute inset-0 z-0 opacity-40 scale-110 blur-3xl"
        style={{
          backgroundImage: `url(${currentTrack.coverUrl})`,
          backgroundSize: 'cover',
          backgroundPosition: 'center',
        }}
      />
      <div className="absolute inset-0 bg-gradient-to-b from-black/20 via-[#121212]/80 to-[#121212] z-0" />

      {/* Header */}
      <PlayerHeader 
        title={currentTrack.title} 
        onBack={() => navigate(-1)} 
      />

      {/* Content */}
      <div className="flex-1 flex flex-col items-center justify-between px-8 py-8 relative z-10 overflow-y-auto">
        {/* Cover */}
        <PlayerCoverCard 
          coverUrl={currentTrack.coverUrl} 
          isPlaying={isPlaying} 
        />

        {/* Info & Controls */}
        <div className="w-full flex flex-col gap-6">
          <PlayerControls
            title={currentTrack.title}
            artist={currentTrack.artist}
            isLiked={isLiked}
            onToggleLike={() => setIsLiked(!isLiked)}
            progress={progress}
            duration={duration}
            isPlaying={isPlaying}
            onSeek={handleSeek}
            onTogglePlay={isPlaying ? pause : resume}
            formatTime={formatTime}
          />

          <PlayerActions />
        </div>
      </div>
    </div>
  );
};

