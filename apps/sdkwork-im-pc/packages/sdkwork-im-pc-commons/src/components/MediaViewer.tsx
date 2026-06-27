import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, ZoomIn, ZoomOut, Download, Minimize, Maximize, ChevronLeft, ChevronRight, Pin, LayoutGrid, RotateCw, MoreHorizontal, Square, Minus } from 'lucide-react';
import { cn } from '../utils';

export interface MediaItem {
  id: string;
  type: 'image' | 'video';
  src: string;
  name?: string;
}

export interface MediaViewerProps {
  isOpen: boolean;
  onClose: () => void;
  // Multiple items mode
  items?: MediaItem[];
  currentIndex?: number;
  onIndexChange?: (index: number) => void;
  // Single item mode (backward compatibility)
  type?: 'image' | 'video';
  src?: string;
  fileName?: string;
}

export const MediaViewer: React.FC<MediaViewerProps> = ({
  isOpen,
  items = [],
  currentIndex = 0,
  onIndexChange,
  onClose,
  type,
  src,
  fileName = 'Media',
}) => {
  const [scale, setScale] = useState(1);
  const [rotation, setRotation] = useState(0);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const mediaList = items.length > 0 ? items : (src ? [{ id: 'single', type: type as 'image' | 'video', src, name: fileName }] : []);
  const activeIndex = items.length > 0 ? currentIndex : 0;
  const currentMedia = mediaList[activeIndex];

  // Reset scale and rotation when opened or switched
  useEffect(() => {
    if (isOpen) {
      setScale(1);
      setRotation(0);
    }
  }, [isOpen, activeIndex]);

  const handleZoomIn = (e: React.MouseEvent) => {
    e.stopPropagation();
    setScale(prev => Math.min(prev + 0.5, 4));
  };

  const handleZoomOut = (e: React.MouseEvent) => {
    e.stopPropagation();
    setScale(prev => Math.max(prev - 0.5, 0.5));
  };

  const handleDownload = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!currentMedia) return;
    const link = document.createElement('a');
    link.href = currentMedia.src;
    link.download = currentMedia.name || 'Media';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const toggleFullscreen = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
      });
      setIsFullscreen(true);
    } else {
      if (document.exitFullscreen) {
        document.exitFullscreen();
        setIsFullscreen(false);
      }
    }
  };

  const handlePrev = (e?: React.MouseEvent) => {
    e?.stopPropagation();
    if (activeIndex > 0 && onIndexChange) {
      onIndexChange(activeIndex - 1);
    }
  };

  const handleNext = (e?: React.MouseEvent) => {
    e?.stopPropagation();
    if (activeIndex < mediaList.length - 1 && onIndexChange) {
      onIndexChange(activeIndex + 1);
    }
  };

  // Keyboard navigation & Close
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;
      if (e.key === 'Escape') {
        onClose();
        if (isFullscreen && document.exitFullscreen) {
          document.exitFullscreen();
          setIsFullscreen(false);
        }
      } else if (e.key === 'ArrowLeft') {
        handlePrev();
      } else if (e.key === 'ArrowRight') {
        handleNext();
      } else if (e.key === '=' || e.key === '+') {
         setScale(prev => Math.min(prev + 0.5, 4));
      } else if (e.key === '-') {
         setScale(prev => Math.max(prev - 0.5, 0.5));
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose, isFullscreen, activeIndex, mediaList.length, onIndexChange]);

  const handleWheel = (e: React.WheelEvent) => {
    if (e.deltaY < 0) {
      setScale(prev => Math.min(prev + 0.1, 4));
    } else {
      setScale(prev => Math.max(prev - 0.1, 0.5));
    }
  };

  if (!isOpen || !currentMedia) return null;

  return (
    <AnimatePresence>
      {/* Backdrop */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.2 }}
        className="fixed inset-0 z-[200] bg-black/60 flex items-center justify-center p-4 md:p-12"
        onClick={onClose}
      >
        {/* Modal Window */}
        <motion.div
          initial={{ scale: 0.95, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          exit={{ scale: 0.95, opacity: 0 }}
          transition={{ type: "spring", damping: 25, stiffness: 300 }}
          className={cn(
            "relative flex flex-col bg-[#1e1e1e] rounded-xl shadow-2xl overflow-hidden border border-white/10 pointer-events-auto",
            isFullscreen ? "w-full h-full rounded-none border-none" : "w-fit h-fit max-w-[90vw] max-h-[90vh] min-w-[360px]"
          )}
          onClick={(e) => e.stopPropagation()}
        >
          {/* Desktop-style Title / Tool Bar */}
          <div className="h-[54px] w-full shrink-0 border-b border-white/5 bg-[#222224] flex items-center justify-between px-4 select-none z-30">
            {/* Left Actions */}
            <div className="flex items-center gap-2 sm:gap-3 text-gray-400">
              <button className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="置顶">
                <Pin size={18} />
              </button>
              <div className="w-px h-4 bg-white/10 mx-0.5 sm:mx-1" />
              <button 
                onClick={handlePrev} 
                disabled={activeIndex === 0} 
                className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors disabled:opacity-30 disabled:hover:bg-transparent"
                title="上一张"
              >
                <ChevronLeft size={20} />
              </button>
              <button 
                onClick={handleNext} 
                disabled={activeIndex === mediaList.length - 1} 
                className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors disabled:opacity-30 disabled:hover:bg-transparent"
                title="下一张"
              >
                <ChevronRight size={20} />
              </button>
              <div className="hidden sm:block w-px h-4 bg-white/10 mx-1" />
              <button className="hidden sm:flex p-1.5 hover:text-white hover:bg-white/5 items-center justify-center rounded transition-colors" title="网格视图">
                <LayoutGrid size={18} />
              </button>
              <div className="w-px h-4 bg-white/10 mx-0.5 sm:mx-1" />
              
              {currentMedia.type === 'image' && (
                <>
                  <button onClick={handleZoomIn} className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="放大">
                    <ZoomIn size={18} />
                  </button>
                  <button onClick={handleZoomOut} className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="缩小">
                    <ZoomOut size={18} />
                  </button>
                  <button 
                    onClick={() => { setScale(1); setRotation(0); }} 
                    className="hidden sm:flex p-1.5 hover:text-white hover:bg-white/5 items-center justify-center rounded transition-colors text-[12px] font-mono mt-0.5" 
                    title="1:1"
                  >
                    1:1
                  </button>
                  <button onClick={() => setRotation(r => r + 90)} className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="旋转">
                    <RotateCw size={18} />
                  </button>
                </>
              )}
              
              <button onClick={handleDownload} className="p-1 sm:p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="下载">
                <Download size={18} />
              </button>
            </div>

            {/* Title */}
            <div className="flex-1 px-2 sm:px-4 text-center text-[13px] font-medium text-gray-300 truncate pointer-events-none min-w-[60px]">
              <span className="hidden sm:inline">{currentMedia.name || 'Media Viewer'}</span>
              {mediaList.length > 1 && <span className="opacity-60 sm:ml-2">({activeIndex + 1}/{mediaList.length})</span>}
            </div>

            {/* Right Window Controls */}
            <div className="flex items-center gap-1 sm:gap-2 text-gray-400">
              <button className="hidden sm:flex p-1.5 hover:text-white hover:bg-white/5 items-center justify-center rounded transition-colors" title="更多">
                <MoreHorizontal size={18} />
              </button>
              <div className="hidden sm:block w-px h-4 bg-white/10 mx-1" />
              <button onClick={onClose} className="p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="最小化">
                <Minus size={18} />
              </button>
              <button onClick={toggleFullscreen} className="p-1.5 hover:text-white hover:bg-white/5 flex items-center justify-center rounded transition-colors" title="最大化">
                {isFullscreen ? <Minimize size={18} /> : <Square size={14} strokeWidth={2.5} />}
              </button>
              <button onClick={onClose} className="p-1.5 hover:text-white hover:bg-red-500 hover:text-white flex items-center justify-center rounded transition-colors" title="关闭 (Esc)">
                <X size={20} />
              </button>
            </div>
          </div>

          {/* Media Content */}
          <div 
            className="flex-1 relative bg-[#0f0f0f] flex items-center justify-center overflow-hidden outline-none select-none group content-center min-h-[100px]"
            onWheel={currentMedia.type === 'image' ? handleWheel : undefined}
          >
            {/* Inline Navigation Arrows for explicit overlay interactions */}
            {mediaList.length > 1 && (
              <>
                <button 
                  onClick={handlePrev}
                  disabled={activeIndex === 0}
                  className="absolute left-4 top-1/2 -translate-y-1/2 p-3 text-white/50 hover:text-white bg-black/40 hover:bg-black/80 rounded-full transition-all disabled:opacity-0 disabled:pointer-events-none z-20 opacity-0 group-hover:opacity-100"
                >
                  <ChevronLeft size={28} />
                </button>
                <button 
                  onClick={handleNext}
                  disabled={activeIndex === mediaList.length - 1}
                  className="absolute right-4 top-1/2 -translate-y-1/2 p-3 text-white/50 hover:text-white bg-black/40 hover:bg-black/80 rounded-full transition-all disabled:opacity-0 disabled:pointer-events-none z-20 opacity-0 group-hover:opacity-100"
                >
                  <ChevronRight size={28} />
                </button>
              </>
            )}

            <AnimatePresence mode="wait">
              {currentMedia.type === 'image' ? (
                <motion.img
                  key={`img-${currentMedia.id}`}
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale, rotate: rotation }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ type: "spring", damping: 25, stiffness: 300 }}
                  src={currentMedia.src}
                  className={cn(
                    "block object-contain drop-shadow-2xl mx-auto",
                    isFullscreen ? "w-full h-[calc(100vh-54px)] max-h-none max-w-none" : "w-auto h-auto max-w-[90vw] max-h-[calc(90vh-54px)]",
                    scale > 1 && "cursor-grab active:cursor-grabbing"
                  )}
                  drag={scale > 1}
                  dragConstraints={{ top: -2000, left: -2000, right: 2000, bottom: 2000 }}
                  dragElastic={0.1}
                  onClick={(e) => e.stopPropagation()}
                  referrerPolicy="no-referrer"
                  draggable={false}
                />
              ) : (
                <div className={cn("relative flex items-center justify-center", isFullscreen ? "w-full h-[calc(100vh-54px)]" : "w-auto h-auto max-w-[90vw] max-h-[calc(90vh-54px)] min-w-[60vw] min-h-[50vh]")}>
                  <motion.video
                    key={`vid-${currentMedia.id}`}
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    transition={{ type: "spring", damping: 25, stiffness: 300 }}
                    src={currentMedia.src}
                    className="w-full h-full max-w-[90vw] max-h-[calc(90vh-54px)] object-contain drop-shadow-2xl outline-none"
                    controls
                    autoPlay
                    onClick={(e) => e.stopPropagation()}
                    controlsList="nodownload"
                  />
                </div>
              )}
            </AnimatePresence>
          </div>
        </motion.div>
      </motion.div>
    </AnimatePresence>
  );
};
