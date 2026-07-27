import { create } from "zustand";

export interface Track {
  id: string;
  title: string;
  artist: string;
  coverUrl: string;
  audioUrl: string;
}

interface AudioState {
  currentTrack: Track | null;
  isPlaying: boolean;
  progress: number;
  duration: number;
  audioElement: HTMLAudioElement | null;
  initAudio: () => void;
  playMusic: (track: Track) => void;
  pause: () => void;
  resume: () => void;
  seek: (time: number) => void;
  stop: () => void;
}

export const useAudioStore = create<AudioState>((set, get) => ({
  currentTrack: null,
  isPlaying: false,
  progress: 0,
  duration: 0,
  audioElement: null,

  initAudio: () => {
    if (get().audioElement) return; // already initialized

    const audio = new Audio();
    // Configure background audio properties if we were native

    audio.addEventListener("timeupdate", () => {
      set({ progress: audio.currentTime });
    });

    audio.addEventListener("loadedmetadata", () => {
      set({ duration: audio.duration });
    });

    audio.addEventListener("ended", () => {
      set({ isPlaying: false, progress: 0 });
    });

    audio.addEventListener("play", () => set({ isPlaying: true }));
    audio.addEventListener("pause", () => set({ isPlaying: false }));

    set({ audioElement: audio });
  },

  playMusic: (track) => {
    const { audioElement, currentTrack } = get();
    if (!audioElement) return;

    if (currentTrack?.id === track.id) {
      if (!get().isPlaying) {
        audioElement.play().catch(console.error);
      }
      return;
    }

    set({ currentTrack: track, progress: 0 });
    audioElement.src = track.audioUrl;
    audioElement.play().catch(console.error);
  },

  pause: () => {
    const { audioElement } = get();
    if (audioElement) audioElement.pause();
  },

  resume: () => {
    const { audioElement, currentTrack } = get();
    if (audioElement && currentTrack) audioElement.play().catch(console.error);
  },

  seek: (time) => {
    const { audioElement } = get();
    if (audioElement) {
      audioElement.currentTime = time;
      set({ progress: time });
    }
  },

  stop: () => {
    const { audioElement } = get();
    if (audioElement) {
      audioElement.pause();
      audioElement.currentTime = 0;
    }
    set({ currentTrack: null, isPlaying: false, progress: 0 });
  },
}));
