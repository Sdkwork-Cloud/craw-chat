import React, { useRef, useState, useEffect, useCallback } from "react";
import { Loader2 } from "lucide-react";
import { VideoPlayerHeader } from "./video-player/VideoPlayerHeader";
import { VideoPlayerSeekFeedback } from "./video-player/VideoPlayerSeekFeedback";
import { VideoPlayerControls } from "./video-player/VideoPlayerControls";

export interface VideoPlayerProps {
  videoSrc: string | undefined;
  isPlaying: boolean;
  setIsPlaying: (isPlaying: boolean) => void;
  onEnded?: () => void;
}

export const VideoPlayer: React.FC<VideoPlayerProps> = ({ videoSrc, isPlaying, setIsPlaying, onEnded }) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [progress, setProgress] = useState(0);
  const [currentTime, setCurrentTime] = useState("00:00");
  const [durationStr, setDurationStr] = useState("00:00");
  const [showControls, setShowControls] = useState(true);
  const [isBuffering, setIsBuffering] = useState(true);
  const [isScrubbing, setIsScrubbing] = useState(false);
  const [showSeekFeedback, setShowSeekFeedback] = useState<{ type: 'forward' | 'backward', show: boolean }>({ type: 'forward', show: false });
  const controlsTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const lastTapRef = useRef<{ time: number, x: number }>({ time: 0, x: 0 });

  const [speed, setSpeed] = useState(1);
  const [showSpeedMenu, setShowSpeedMenu] = useState(false);
  const speeds = [0.75, 1.0, 1.25, 1.5, 2.0];

  useEffect(() => {
    setProgress(0);
    setCurrentTime("00:00");
    setIsBuffering(true);
    if (videoRef.current) {
       videoRef.current.load();
       if (isPlaying) {
         videoRef.current.play().catch(() => setIsPlaying(false));
       }
    }
  }, [videoSrc]);

  useEffect(() => {
    if (videoRef.current) {
       videoRef.current.playbackRate = speed;
    }
  }, [speed, videoSrc]);

  useEffect(() => {
    if (videoRef.current && videoSrc && !isScrubbing) {
       if (isPlaying) {
         videoRef.current.play().catch(() => setIsPlaying(false));
       } else {
         videoRef.current.pause();
       }
    }
  }, [isPlaying, isScrubbing, videoSrc]);

  const formatTime = (timeInSeconds: number) => {
    if (isNaN(timeInSeconds) || !isFinite(timeInSeconds)) return "00:00";
    const minutes = Math.floor(timeInSeconds / 60);
    const seconds = Math.floor(timeInSeconds % 60);
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  const updateProgress = useCallback(() => {
    if (videoRef.current && !isScrubbing) {
      const current = videoRef.current.currentTime;
      const duration = videoRef.current.duration;
      setCurrentTime(formatTime(current));
      if (duration > 0) {
        setProgress((current / duration) * 100);
      }
    }
  }, [isScrubbing]);

  const handleLoadedMetadata = () => {
    if (videoRef.current) {
      setDurationStr(formatTime(videoRef.current.duration));
      setIsBuffering(false);
    }
  };

  const togglePlayPause = (e?: React.MouseEvent) => {
    e?.stopPropagation();
    setIsPlaying(!isPlaying);
    resetControlsTimeout();
  };

  const resetControlsTimeout = useCallback(() => {
    if (controlsTimeoutRef.current) {
      clearTimeout(controlsTimeoutRef.current);
    }
    setShowControls(true);
    controlsTimeoutRef.current = setTimeout(() => {
      if (isPlaying && !isScrubbing) {
         setShowControls(false);
      }
    }, 4000);
  }, [isPlaying, isScrubbing]);

  useEffect(() => {
     resetControlsTimeout();
     return () => {
        if (controlsTimeoutRef.current) clearTimeout(controlsTimeoutRef.current);
     };
  }, [isPlaying, resetControlsTimeout]);

  const handleVideoClick = (e: React.MouseEvent) => {
    if (showSpeedMenu) setShowSpeedMenu(false);
    const now = Date.now();
    const doubleTapDelay = 300;
    
    if (now - lastTapRef.current.time < doubleTapDelay) {
      const rect = e.currentTarget.getBoundingClientRect();
      const clickX = e.clientX - rect.left;
      const width = rect.width;
      
      if (videoRef.current) {
         const duration = videoRef.current.duration;
         if (!isNaN(duration)) {
             if (clickX < width / 2) {
                 videoRef.current.currentTime = Math.max(0, videoRef.current.currentTime - 10);
                 setShowSeekFeedback({ type: 'backward', show: true });
             } else {
                 videoRef.current.currentTime = Math.min(duration, videoRef.current.currentTime + 10);
                 setShowSeekFeedback({ type: 'forward', show: true });
             }
             setTimeout(() => setShowSeekFeedback(prev => ({ ...prev, show: false })), 500);
             updateProgress();
             if (!isPlaying) setIsPlaying(true);
         }
      }
      lastTapRef.current.time = 0;
    } else {
      lastTapRef.current = { time: now, x: e.clientX };
      setTimeout(() => {
         if (lastTapRef.current.time === now) {
            setShowControls(prev => !prev);
            resetControlsTimeout();
         }
      }, doubleTapDelay);
    }
  };

  const handlePointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    e.stopPropagation();
    e.currentTarget.setPointerCapture(e.pointerId);
    setIsScrubbing(true);
    resetControlsTimeout();
    handlePointerMove(e);
  };

  const handlePointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    if (isScrubbing && videoRef.current) {
      const rect = e.currentTarget.getBoundingClientRect();
      let percent = (e.clientX - rect.left) / rect.width;
      percent = Math.max(0, Math.min(1, percent));
      setProgress(percent * 100);
      setCurrentTime(formatTime(percent * videoRef.current.duration));
    }
  };

  const handlePointerUp = (e: React.PointerEvent<HTMLDivElement>) => {
    e.stopPropagation();
    e.currentTarget.releasePointerCapture(e.pointerId);
    setIsScrubbing(false);
    if (videoRef.current) {
      const rect = e.currentTarget.getBoundingClientRect();
      let percent = (e.clientX - rect.left) / rect.width;
      percent = Math.max(0, Math.min(1, percent));
      videoRef.current.currentTime = percent * videoRef.current.duration;
    }
    resetControlsTimeout();
  };

  const requestFullScreen = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (containerRef.current) {
      if (document.fullscreenElement) {
        document.exitFullscreen?.();
      } else if (containerRef.current.requestFullscreen) {
        containerRef.current.requestFullscreen();
      } else if ((containerRef.current as any).webkitRequestFullscreen) {
        (containerRef.current as any).webkitRequestFullscreen();
      }
    }
  };

  return (
    <div ref={containerRef} className="relative w-full bg-black aspect-video shrink-0 pt-safe-top z-20 overflow-hidden group/player">
       <div 
         className="absolute inset-0 pt-safe-top cursor-pointer"
         onClick={handleVideoClick}
         onMouseMove={resetControlsTimeout}
       >
          <video
             ref={videoRef}
             className="w-full h-full object-contain"
             src={videoSrc}
             onTimeUpdate={updateProgress}
             onLoadedMetadata={handleLoadedMetadata}
             onWaiting={() => setIsBuffering(true)}
             onPlaying={() => setIsBuffering(false)}
             onCanPlay={() => setIsBuffering(false)}
             onLoadedData={() => setIsBuffering(false)}
             onEnded={() => {
                setIsPlaying(false);
                onEnded?.();
             }}
             playsInline
             preload="metadata"
          />
          
          {/* Buffering Indicator */}
          {isBuffering && (
            <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
               <Loader2 className="w-10 h-10 text-white/80 animate-spin drop-shadow-md" />
            </div>
          )}

          {/* Seek Feedback */}
          <VideoPlayerSeekFeedback showSeekFeedback={showSeekFeedback} />
          
          <div className={`absolute inset-0 flex flex-col justify-between pt-safe-top transition-opacity duration-300 ${showControls || isScrubbing || !isPlaying ? "opacity-100" : "opacity-0"}`}>
             <div className="absolute inset-0 bg-gradient-to-b from-black/60 via-transparent to-black/80 pointer-events-none" />
             
             {/* Header */}
             <VideoPlayerHeader />

             {/* Controls */}
             <VideoPlayerControls
                isPlaying={isPlaying}
                isBuffering={isBuffering}
                currentTime={currentTime}
                durationStr={durationStr}
                speed={speed}
                speeds={speeds}
                showSpeedMenu={showSpeedMenu}
                progress={progress}
                isScrubbing={isScrubbing}
                togglePlayPause={togglePlayPause}
                setShowSpeedMenu={setShowSpeedMenu}
                setSpeed={setSpeed}
                requestFullScreen={requestFullScreen}
                handlePointerDown={handlePointerDown}
                handlePointerMove={handlePointerMove}
                handlePointerUp={handlePointerUp}
             />
          </div>
       </div>
    </div>
  );
};

