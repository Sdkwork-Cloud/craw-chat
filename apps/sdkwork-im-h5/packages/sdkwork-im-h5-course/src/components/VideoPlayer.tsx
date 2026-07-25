import { useTranslation } from "react-i18next";
import React, { useRef, useState, useEffect, useCallback } from "react";
import { ChevronLeft, Play, Pause, MoreVertical, Maximize, RotateCcw, Loader2 } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export interface VideoPlayerProps {
  videoSrc: string | undefined;
  isPlaying: boolean;
  setIsPlaying: (isPlaying: boolean) => void;
  onEnded?: () => void;
}

export const VideoPlayer: React.FC<VideoPlayerProps> = ({ videoSrc, isPlaying, setIsPlaying, onEnded }) => {
  const { t } = useTranslation();
const navigate = useNavigate();
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
      // Double tap detected
      const rect = e.currentTarget.getBoundingClientRect();
      const clickX = e.clientX - rect.left;
      const width = rect.width;
      
      if (videoRef.current) {
         const duration = videoRef.current.duration;
         if (!isNaN(duration)) {
             if (clickX < width / 2) {
                 // Seek backward 10s
                 videoRef.current.currentTime = Math.max(0, videoRef.current.currentTime - 10);
                 setShowSeekFeedback({ type: 'backward', show: true });
             } else {
                 // Seek forward 10s
                 videoRef.current.currentTime = Math.min(duration, videoRef.current.currentTime + 10);
                 setShowSeekFeedback({ type: 'forward', show: true });
             }
             setTimeout(() => setShowSeekFeedback(prev => ({ ...prev, show: false })), 500);
             updateProgress();
             if (!isPlaying) setIsPlaying(true);
         }
      }
      lastTapRef.current.time = 0; // Reset
    } else {
      // Single tap
      lastTapRef.current = { time: now, x: e.clientX };
      // Show/hide controls after a small delay to distinguish from double tap
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

          {/* Double Tap Seek Feedback */}
          {showSeekFeedback.show && (
            <div className={`absolute top-0 bottom-0 w-1/3 flex flex-col items-center justify-center bg-white/10 pointer-events-none transition-all duration-300 animate-pulse ${showSeekFeedback.type === 'forward' ? 'right-0 rounded-l-full' : 'left-0 rounded-r-full'}`}>
               <div className="flex gap-1 text-white">
                 <RotateCcw className={`w-8 h-8 ${showSeekFeedback.type === 'forward' ? 'scale-x-[-1]' : ''}`} />
               </div>
               <span className="text-white text-[13px] font-bold mt-2">{t('course.auto_13793', '10秒')}</span>
            </div>
          )}
          
          <div className={`absolute inset-0 flex flex-col justify-between pt-safe-top transition-opacity duration-300 ${showControls || isScrubbing || !isPlaying ? "opacity-100" : "opacity-0"}`}>
             <div className="absolute inset-0 bg-gradient-to-b from-black/60 via-transparent to-black/80 pointer-events-none" />
             
             {/* Header */}
             <div className="relative flex items-center justify-between p-2">
                <IconButton
                  icon={<ChevronLeft className="w-6 h-6 text-white" />}
                  className="bg-transparent w-9 h-9 pointer-events-auto"
                  onClick={(e) => { e.stopPropagation(); navigate(-1); }}
                />
                <IconButton
                  icon={<MoreVertical className="w-5 h-5 text-white" />}
                  className="bg-transparent w-9 h-9 pointer-events-auto"
                  onClick={(e) => { e.stopPropagation(); }}
                />
             </div>
             
             {/* Central Play/Pause */}
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
             
             {/* Bottom Controls */}
             <div className="relative flex flex-col gap-1 p-3 px-4 pointer-events-auto">
                <div className="flex items-center gap-3">
                   <div className="text-white text-[12px] font-mono shrink-0 select-none drop-shadow-md tracking-wider">
                     {currentTime} / {durationStr}
                   </div>
                   <div className="flex-1" />
                   
                   <div className="relative">
                      <div 
                        className="text-white/90 text-[13px] font-medium cursor-pointer px-2 py-1 hover:bg-white/10 rounded-lg transition-colors flex items-center justify-center min-w-[36px]"
                        onClick={(e) => { e.stopPropagation(); setShowSpeedMenu(!showSpeedMenu); }}
                      >
                         {speed}x
                      </div>
                      
                      {showSpeedMenu && (
                         <div className="absolute bottom-full right-0 mb-2 bg-black/80 backdrop-blur-md rounded-xl overflow-hidden flex flex-col min-w-[70px] shadow-lg border border-white/10">
                            {speeds.map(s => (
                               <div 
                                 key={s} 
                                 className={`px-4 py-2.5 text-center text-[13px] font-medium cursor-pointer transition-colors active:bg-white/10 ${speed === s ? 'text-blue-400 bg-white/5' : 'text-white hover:bg-white/5'}`}
                                 onClick={(e) => { e.stopPropagation(); setSpeed(s); setShowSpeedMenu(false); }}
                               >
                                 {s}x
                               </div>
                            ))}
                         </div>
                      )}
                   </div>

                   <div className="cursor-pointer p-1 hover:bg-white/10 rounded-lg transition-colors" onClick={requestFullScreen}>
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
                      {/* Buffered bar (mocked for visual effect) */}
                      <div className="absolute left-0 top-0 bottom-0 bg-white/30 rounded-full" style={{ width: `${Math.min(100, progress + 15)}%` }} />
                      
                      {/* Played bar */}
                      <div className="absolute left-0 top-0 bottom-0 bg-blue-500 rounded-full" style={{ width: `${progress}%` }} />
                      
                      {/* Thumb */}
                      <div 
                        className={`absolute top-1/2 -mt-2 -ml-2 w-4 h-4 bg-white rounded-full shadow-[0_0_8px_rgba(0,0,0,0.5)] transition-transform ${isScrubbing ? 'scale-125' : 'scale-0 group-hover/bar:scale-100'}`} 
                        style={{ left: `${progress}%` }} 
                      />
                   </div>
                </div>
             </div>
          </div>
       </div>
    </div>
  );
};

