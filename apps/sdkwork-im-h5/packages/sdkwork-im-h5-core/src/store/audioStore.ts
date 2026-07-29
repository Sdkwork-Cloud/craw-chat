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
      if (Number.isFinite(audio.currentTime)) {
        set({ progress: audio.currentTime });
      }
    });

    audio.addEventListener("loadedmetadata", () => {
      set({ duration: Number.isFinite(audio.duration) ? audio.duration : 0 });
    });

    audio.addEventListener("ended", () => {
      releaseAudioSource(audio);
      set({
        currentTrack: null,
        isPlaying: false,
        progress: 0,
        duration: 0,
      });
    });

    audio.addEventListener("play", () => set({ isPlaying: true }));
    audio.addEventListener("pause", () => set({ isPlaying: false }));

    set({ audioElement: audio });
  },

  playMusic: (track) => {
    let { audioElement } = get();
    if (!audioElement) {
      get().initAudio();
      audioElement = get().audioElement;
    }
    if (!audioElement) return;
    const { currentTrack } = get();

    if (currentTrack?.id === track.id) {
      if (!get().isPlaying) {
        void audioElement.play().catch(() => {
          set({ isPlaying: false });
        });
      }
      return;
    }

    set({ currentTrack: track, progress: 0 });
    audioElement.src = track.audioUrl;
    void audioElement.play().catch(() => {
      if (get().currentTrack?.id === track.id) {
        set({ isPlaying: false });
      }
    });
  },

  pause: () => {
    const { audioElement } = get();
    if (audioElement) audioElement.pause();
  },

  resume: () => {
    const { audioElement, currentTrack } = get();
    if (audioElement && currentTrack) {
      void audioElement.play().catch(() => {
        set({ isPlaying: false });
      });
    }
  },

  seek: (time) => {
    const { audioElement } = get();
    if (audioElement && Number.isFinite(time)) {
      const duration = Number.isFinite(audioElement.duration)
        ? audioElement.duration
        : 0;
      const boundedTime = Math.min(Math.max(time, 0), duration);
      audioElement.currentTime = boundedTime;
      set({ progress: boundedTime });
    }
  },

  stop: () => {
    const { audioElement } = get();
    if (audioElement) {
      audioElement.pause();
      releaseAudioSource(audioElement);
    }
    set({
      currentTrack: null,
      isPlaying: false,
      progress: 0,
      duration: 0,
    });
  },
}));

function releaseAudioSource(audio: HTMLAudioElement): void {
  audio.removeAttribute("src");
  audio.load();
}
