import { sanitizeMessageUrl } from '@sdkwork/im-pc-commons';

export interface MusicTrack {
  id: string;
  url: string;
  title: string;
  artist: string;
  coverUrl?: string;
  album?: string;
  duration?: number; // duration in seconds
}

export interface PlayerState {
  currentTrack: MusicTrack | null;
  playlist: MusicTrack[];
  currentIndex: number;
  isPlaying: boolean;
  progress: number;
  duration: number;
  volume: number;
  isMuted: boolean;
  isPlayerOpen: boolean; // Controls whether the full page player modal is visible
  playMode: 'sequence' | 'loop' | 'shuffle';
}

type Listener = (state: PlayerState) => void;

export interface MusicService {
  subscribe(listener: Listener): () => void;
  getState(): PlayerState;
  setPlaylist(tracks: MusicTrack[], startIndex?: number): void;
  addTrackToPlaylist(track: MusicTrack): void;
  play(track: MusicTrack): void;
  playTrackFromList(index: number): void;
  playNext(): void;
  playPrev(): void;
  togglePlayMode(): void;
  togglePlay(): void;
  seek(time: number): void;
  setVolume(vol: number): void;
  toggleMute(): void;
  togglePlayer(): void;
}

class SdkworkMusicService implements MusicService {
  private audio: HTMLAudioElement;
  private state: PlayerState = {
    currentTrack: null,
    playlist: [],
    currentIndex: -1,
    isPlaying: false,
    progress: 0,
    duration: 0,
    volume: 1,
    isMuted: false,
    isPlayerOpen: false,
    playMode: 'sequence',
  };
  private listeners: Set<Listener> = new Set();

  constructor() {
    this.audio = new Audio();
    
    this.audio.addEventListener('timeupdate', () => {
      this.updateState({ progress: this.audio.currentTime });
    });
    
    this.audio.addEventListener('durationchange', () => {
      this.updateState({ duration: this.audio.duration || 0 });
    });
    
    this.audio.addEventListener('ended', () => {
      if (this.state.playMode === 'loop') {
         this.audio.currentTime = 0;
         this.audio.play();
      } else {
         this.playNext();
      }
    });
    
    this.audio.addEventListener('play', () => {
      this.updateState({ isPlaying: true });
    });
    
    this.audio.addEventListener('pause', () => {
      this.updateState({ isPlaying: false });
    });
  }

  public subscribe(listener: Listener) {
    this.listeners.add(listener);
    listener(this.state);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private updateState(partial: Partial<PlayerState>) {
    this.state = { ...this.state, ...partial };
    this.listeners.forEach(l => l(this.state));
  }

  public getState() {
    return this.state;
  }

  private resolveTrackUrl(track: MusicTrack): string | null {
    return sanitizeMessageUrl(track.url);
  }

  public setPlaylist(tracks: MusicTrack[], startIndex: number = 0) {
     this.updateState({ 
       playlist: tracks, 
       currentIndex: startIndex,
       currentTrack: tracks[startIndex] || null 
     });
     
     const trackUrl = tracks[startIndex] ? this.resolveTrackUrl(tracks[startIndex]) : null;
     if (trackUrl) {
        this.audio.src = trackUrl;
        this.audio.play().catch(e => console.error("Playback failed", e));
        this.updateState({ isPlaying: true, isPlayerOpen: true });
     }
  }
  
  public addTrackToPlaylist(track: MusicTrack) {
     const newPlaylist = [...this.state.playlist, track];
     this.updateState({ playlist: newPlaylist });
     if (!this.state.currentTrack) {
        this.setPlaylist([track], 0);
     }
  }

  public play(track: MusicTrack) {
    if (this.state.currentTrack?.id === track.id) {
      if (!this.state.isPlayerOpen) {
         this.updateState({ isPlayerOpen: true });
      }
      this.togglePlay();
      return;
    }
    
    const existingIndex = this.state.playlist.findIndex(t => t.id === track.id);
    if (existingIndex !== -1) {
       this.playTrackFromList(existingIndex);
    } else {
       // Replace current playlist with a single item queue or add to it based on logic.
       // Let's just create a new playlist of 1 for now if played externally.
       this.setPlaylist([track], 0);
    }
  }

  public playTrackFromList(index: number) {
     if (index >= 0 && index < this.state.playlist.length) {
         const track = this.state.playlist[index];
         const trackUrl = this.resolveTrackUrl(track);
         if (!trackUrl) {
           return;
         }
         this.audio.src = trackUrl;
         this.audio.play().catch(e => console.error("Playback failed", e));
         this.updateState({ currentTrack: track, currentIndex: index, isPlaying: true, isPlayerOpen: true });
     }
  }

  public playNext() {
      if (this.state.playlist.length === 0) return;
      let nextIndex = this.state.currentIndex + 1;
      
      if (this.state.playMode === 'shuffle') {
          nextIndex = Math.floor(Math.random() * this.state.playlist.length);
      } else if (nextIndex >= this.state.playlist.length) {
          nextIndex = 0; // wrap around
      }
      this.playTrackFromList(nextIndex);
  }

  public playPrev() {
      if (this.state.playlist.length === 0) return;
      let prevIndex = this.state.currentIndex - 1;
      
      if (this.state.playMode === 'shuffle') {
          prevIndex = Math.floor(Math.random() * this.state.playlist.length);
      } else if (prevIndex < 0) {
          prevIndex = this.state.playlist.length - 1; // wrap around
      }
      this.playTrackFromList(prevIndex);
  }
  
  public togglePlayMode() {
     const modes: Array<'sequence' | 'loop' | 'shuffle'> = ['sequence', 'loop', 'shuffle'];
     const nextModeIndex = (modes.indexOf(this.state.playMode) + 1) % modes.length;
     this.updateState({ playMode: modes[nextModeIndex] });
  }

  public togglePlay() {
    if (!this.state.currentTrack) return;
    
    if (this.state.isPlaying) {
      this.audio.pause();
    } else {
      this.audio.play();
    }
  }

  public seek(time: number) {
    this.audio.currentTime = time;
    this.updateState({ progress: time });
  }

  public setVolume(vol: number) {
    this.audio.volume = vol;
    if (vol > 0) {
      this.audio.muted = false;
      this.updateState({ volume: vol, isMuted: false });
    } else {
      this.updateState({ volume: vol });
    }
  }

  public toggleMute() {
    this.audio.muted = !this.audio.muted;
    this.updateState({ isMuted: this.audio.muted });
  }

  public togglePlayer() {
    this.updateState({ isPlayerOpen: !this.state.isPlayerOpen });
  }
}

export const musicService: MusicService = new SdkworkMusicService();
