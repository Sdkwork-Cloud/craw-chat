import React from "react";
import { useNavigate } from "react-router";

import { useAudioStore } from "@sdkwork/im-h5-core";

import { PlayerControls } from "../components/music-player/PlayerControls";
import { PlayerCoverCard } from "../components/music-player/PlayerCoverCard";
import { PlayerHeader } from "../components/music-player/PlayerHeader";

export const MusicPlayerPage: React.FC = () => {
  const navigate = useNavigate();
  const currentTrack = useAudioStore((state) => state.currentTrack);
  const isPlaying = useAudioStore((state) => state.isPlaying);
  const progress = useAudioStore((state) => state.progress);
  const duration = useAudioStore((state) => state.duration);
  const pause = useAudioStore((state) => state.pause);
  const resume = useAudioStore((state) => state.resume);
  const seek = useAudioStore((state) => state.seek);

  if (!currentTrack) {
    return (
      <div className="flex h-full flex-col items-center justify-center bg-[#121212] text-white">
        <p>No track is playing</p>
        <button
          type="button"
          onClick={() => navigate(-1)}
          className="mt-4 rounded-full bg-white/10 px-4 py-2"
        >
          Back
        </button>
      </div>
    );
  }

  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-[#121212] text-white">
      <div
        className="absolute inset-0 z-0 scale-110 bg-cover bg-center opacity-40 blur-3xl"
        style={{ backgroundImage: `url(${currentTrack.coverUrl})` }}
      />
      <div className="absolute inset-0 z-0 bg-gradient-to-b from-black/20 via-[#121212]/80 to-[#121212]" />

      <PlayerHeader
        title={currentTrack.title}
        onBack={() => navigate(-1)}
      />

      <div className="relative z-10 flex flex-1 flex-col items-center justify-between overflow-y-auto px-8 py-8">
        <PlayerCoverCard
          coverUrl={currentTrack.coverUrl}
          isPlaying={isPlaying}
        />

        <div className="flex w-full flex-col gap-6">
          <PlayerControls
            title={currentTrack.title}
            artist={currentTrack.artist}
            progress={progress}
            duration={duration}
            isPlaying={isPlaying}
            onSeek={(event) => seek(Number(event.target.value))}
            onTogglePlay={isPlaying ? pause : resume}
            formatTime={formatTime}
          />
        </div>
      </div>
    </div>
  );
};

function formatTime(seconds: number): string {
  const minutes = Math.floor(seconds / 60).toString().padStart(2, "0");
  const remainingSeconds = Math.floor(seconds % 60)
    .toString()
    .padStart(2, "0");
  return `${minutes}:${remainingSeconds}`;
}
