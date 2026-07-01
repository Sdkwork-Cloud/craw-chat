import React, { useEffect, useState, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, Play, Pause, SkipBack, SkipForward, Volume2, VolumeX, ListMusic, Heart, Repeat, Shuffle, Minus, Maximize2, RotateCcw, MoreHorizontal } from 'lucide-react';
import { musicService, PlayerState, MusicTrack } from '../services/MusicService';
import { DEFAULT_MUSIC_COVER_URL } from '../services/DefaultAvatarService';
import { cn, formatDuration } from '@sdkwork/im-pc-commons';
import { toast } from './Toast';

// Equalizer-style visualizer bar for the music player chrome.
const VisualizerBar: React.FC<{ isPlaying: boolean; delay: number }> = ({ isPlaying, delay }) => (
  <div 
    className="w-1 bg-[#1db954]/50 rounded-full"
    style={{ 
      height: isPlaying ? '100%' : '20%',
      transition: 'height 0.2s ease',
      animation: isPlaying ? `equalizer 1.5s ease-in-out infinite alternate ${delay}s` : 'none'
    }}
  />
);

export const MusicPlayer: React.FC = () => {
  const [playerState, setPlayerState] = useState<PlayerState>(musicService.getState());
  const [isHoveringVolume, setIsHoveringVolume] = useState(false);
  const [isPlaylistOpen, setIsPlaylistOpen] = useState(false);
  const [audioQualityMode, setAudioQualityMode] = useState<'sq' | 'hires'>('sq');
  const progressBarRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    return musicService.subscribe(setPlayerState);
  }, []);

  const handleSeek = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!progressBarRef.current || !playerState.duration) return;
    const rect = progressBarRef.current.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    musicService.seek(percent * playerState.duration);
  };

  const track = playerState.currentTrack;

  return (
    <AnimatePresence>
      {playerState.isPlayerOpen && playerState.currentTrack && track && (
        <motion.div

          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.2 }}
          className="fixed inset-0 z-[300] bg-black/50 flex items-center justify-center p-4 backdrop-blur-sm"
          onClick={() => musicService.togglePlayer()}
        >
          <motion.div
            initial={{ scale: 0.95, opacity: 0, y: 30 }}
            animate={{ scale: 1, opacity: 1, y: 0 }}
            exit={{ scale: 0.95, opacity: 0, y: 20 }}
            transition={{ type: 'spring', damping: 30, stiffness: 300 }}
            className="relative w-full max-w-[1000px] h-[680px] bg-[#0A0A0A] rounded-[24px] shadow-[0_30px_100px_rgba(0,0,0,0.8)] overflow-hidden border border-white/10 flex flex-col pointer-events-auto"
            onClick={e => e.stopPropagation()}
            style={{
               boxShadow: '0 0 0 1px rgba(255,255,255,0.05), 0 30px 100px rgba(0,0,0,0.8)'
            }}
          >
            {/* Background Blur Overlay within Modal */}
            <div className="absolute inset-0 z-0 overflow-hidden pointer-events-none">
              <div 
                className="absolute inset-[-50%] opacity-20 blur-[120px] transition-all duration-1000 saturate-150"
                style={{ 
                  backgroundImage: `url(${track.coverUrl || DEFAULT_MUSIC_COVER_URL})`,
                  backgroundSize: 'cover',
                  backgroundPosition: 'center',
                }}
              />
              <div className="absolute inset-0 bg-gradient-to-b from-[#0A0A0A]/40 via-[#0A0A0A]/80 to-[#0A0A0A] mix-blend-multiply" />
            </div>

            {/* Top Windows/Mac Control Bar */}
            <div className="relative z-20 h-14 shrink-0 flex items-center justify-between px-6 border-b border-white/5 drag-region select-none bg-gradient-to-b from-white/5 to-transparent">
               <div className="flex items-center gap-3 no-drag">
                 <div className="flex items-center gap-1.5 mr-4 group">
                    <button className="w-3.5 h-3.5 rounded-full bg-[#FF5F56] border border-[#E0443E] hover:brightness-110 flex items-center justify-center transition-all" onClick={() => musicService.togglePlayer()}>
                      <X size={8} className="opacity-0 group-hover:opacity-100 text-black/60" />
                    </button>
                    <button className="w-3.5 h-3.5 rounded-full bg-[#FFBD2E] border border-[#DEA123] hover:brightness-110 flex items-center justify-center transition-all" onClick={() => musicService.togglePlayer()}>
                      <Minus size={8} className="opacity-0 group-hover:opacity-100 text-black/60" />
                    </button>
                    <button className="w-3.5 h-3.5 rounded-full bg-[#27C93F] border border-[#1AAB29] hover:brightness-110 flex items-center justify-center transition-all" onClick={() => {
                        const el = document.documentElement;
                        if (!document.fullscreenElement) {
                          el.requestFullscreen().catch(() => toast('无法全屏显示', 'error'));
                        } else {
                          document.exitFullscreen();
                        }
                    }}>
                      <Maximize2 size={8} className="opacity-0 group-hover:opacity-100 text-black/60" />
                    </button>
                 </div>
                 <div className="w-7 h-7 rounded-lg bg-gradient-to-br from-[#1db954] to-[#15873d] flex items-center justify-center text-white shadow-lg border border-[#1db954]/50">
                   <svg viewBox="0 0 24 24" fill="currentColor" className="w-4 h-4"><path d="M12 2C6.477 2 2 6.477 2 12c0 5.523 4.477 10 10 10s10-4.477 10-10c0-5.523-4.477-10-10-10zm4.586 14.424c-.18.295-.563.387-.857.207-2.35-1.434-5.305-1.76-8.784-.964-.336.077-.67-.133-.746-.47-.077-.335.132-.67.47-.745 3.808-.87 7.076-.496 9.71 1.115.295.18.388.563.207.857zm1.144-2.553c-.225.366-.7.48-1.066.255-2.69-1.65-6.8-2.146-9.97-1.176-.41.127-.853-.105-.98-.516-.126-.41.106-.853.516-.98 3.63-1.11 8.35-.556 11.44 1.34.367.226.48.7.255 1.066zM17.9 9.68c-3.21-1.9-8.49-2.07-11.55-1.14-.49.15-.99-.13-1.14-.62-.15-.49.13-.99.62-1.14 3.53-1.07 9.38-.88 13.08 1.31.44.26.58.83.32 1.27-.26.44-.82.58-1.26.32z"/></svg>
                 </div>
                 <span className="text-white/80 text-[13px] font-bold tracking-[0.15em] ml-1">SDKWORK MUSIC</span>
               </div>
               
               <div className="flex items-center gap-2 text-gray-400 no-drag relative">
                 <button
                   type="button"
                   className="h-7 px-3 flex items-center gap-1.5 bg-white/5 hover:bg-white/10 rounded-full transition-colors text-xs font-medium border border-white/5 text-white/70"
                   onClick={() => {
                     if (audioQualityMode === 'hires') {
                       setAudioQualityMode('sq');
                       toast('已切换至 SQ 高品质', 'success');
                       return;
                     }
                     setAudioQualityMode('hires');
                     toast('已切换至 Hi-Res 无损音质', 'success');
                   }}
                 >
                   <span className={cn(
                     'w-1.5 h-1.5 rounded-full animate-pulse',
                     audioQualityMode === 'hires' ? 'bg-yellow-400' : 'bg-[#1db954]',
                   )} />
                   {audioQualityMode === 'hires' ? 'Hi-Res 无损' : 'SQ 高品质'}
                 </button>
                 <button className="w-8 h-8 flex items-center justify-center hover:bg-white/10 hover:text-white rounded-full transition-colors" onClick={(e) => {
                   e.currentTarget.blur();
                   navigator.clipboard.writeText(`https://music.sdkwork.com/play/${playerState.currentTrack?.id}`);
                   toast('已复制歌曲分享链接', 'success');
                 }}>
                   <MoreHorizontal size={18} />
                 </button>
               </div>
            </div>

            {/* Content Body */}
            <div className="relative z-10 flex-1 flex overflow-hidden">
              <style>{`
                @keyframes equalizer {
                  0% { height: 20%; }
                  100% { height: 100%; }
                }
              `}</style>
            
              {/* Left/Main Side (Cover + Specs + Lyrics placeholder) */}
              <div className={cn("flex-1 p-10 flex flex-col transition-all duration-500 ease-out", isPlaylistOpen ? "w-[65%]" : "w-full")}>
                <div className="flex gap-12 items-center w-full h-full max-w-4xl mx-auto">
                  
                  {/* Turntable / Vinyl Record Cover */}
                  <div className="relative shrink-0 flex items-center justify-center group no-drag">
                    <div 
                      className={cn(
                        "relative w-72 h-72 rounded-full flex items-center justify-center bg-black shadow-[0_20px_50px_rgba(0,0,0,0.5)] border-[5px] border-black overflow-hidden ring-1 ring-white/10",
                        playerState.isPlaying ? "animate-[spin_15s_linear_infinite]" : ""
                      )}
                    >
                      {/* Vinyl Grooves inner borders */}
                      <div className="absolute inset-2 border border-white/10 rounded-full" />
                      <div className="absolute inset-4 border border-white/5 rounded-full" />
                      <div className="absolute inset-6 border border-white/5 rounded-full" />
                      <div className="absolute inset-8 border border-white/5 rounded-full" />
                      
                      {/* Album Art inside Vinyl */}
                      <img 
                        src={track.coverUrl || DEFAULT_MUSIC_COVER_URL} 
                        className="absolute w-44 h-44 rounded-full object-cover shadow-[inset_0_0_20px_rgba(0,0,0,0.8)] border-[6px] border-black"
                      />
                      
                      <div className="absolute inset-0 bg-gradient-to-tr from-transparent via-white/10 to-transparent mix-blend-overlay pointer-events-none" />
                      
                      {/* Center Hole */}
                      <div className="absolute w-4 h-4 bg-gray-900 rounded-full border border-black/50 shadow-inner" />
                    </div>

                    {/* Play/Pause Hover Overlay for the cover */}
                    <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity no-drag cursor-pointer z-10" onClick={() => musicService.togglePlay()}>
                      <button className="w-20 h-20 bg-black/50 rounded-full flex items-center justify-center backdrop-blur-md hover:bg-[#1db954] hover:scale-105 transition-all text-white shadow-2xl border border-white/10">
                        {playerState.isPlaying ? <Pause size={32} /> : <Play size={32} className="translate-x-1" />}
                      </button>
                    </div>
                  </div>

                  {/* Track Info & Advanced Pseudo Lyrics */}
                  <div className="flex-1 flex flex-col justify-start min-w-0 h-full py-8">
                     <div className="mb-8">
                       <h1 className="text-4xl font-black text-white mb-3 truncate leading-tight tracking-tight drop-shadow-lg" title={track.title}>{track.title}</h1>
                       <div className="flex items-center gap-3 text-base text-gray-300 font-medium">
                         <span className="hover:text-white cursor-pointer truncate max-w-[200px] transition-colors">{track.artist}</span>
                         <span className="text-gray-600">•</span>
                         <span className="hover:text-white cursor-pointer truncate max-w-[200px] transition-colors">{track.album || 'Unknown Album'}</span>
                       </div>
                     </div>
                     
                     {/* Modern Lyrics Display Placeholder */}
                     <div className="relative flex-1 mask-image-fade-y flex flex-col justify-center">
                        <div className={cn("flex flex-col gap-5 transition-transform duration-700 ease-out", playerState.isPlaying ? "-translate-y-4" : "translate-y-0")}>
                           <p className="text-2xl font-bold text-white/90 transform scale-105 origin-left tracking-wide drop-shadow-md transition-all">
                             {track.title}
                           </p>
                           <p className="text-xl font-medium text-white/50 tracking-wide transition-all">
                             {track.artist}
                           </p>
                           <p className="text-lg font-medium text-white/30 tracking-wide mt-2">
                             (纯享高品质音频播放中)
                           </p>
                           
                           {/* Visualizer inside lyrics area when playing */}
                           <div className={cn("flex items-end gap-1.5 h-8 mt-5 transition-opacity duration-500", playerState.isPlaying ? "opacity-100" : "opacity-0")}>
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0} />
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0.2} />
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0.4} />
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0.1} />
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0.3} />
                             <VisualizerBar isPlaying={playerState.isPlaying} delay={0.5} />
                           </div>
                        </div>
                     </div>
                  </div>
                </div>
              </div>

            {/* Right Side (Playlist Panel) */}
            <AnimatePresence>
              {isPlaylistOpen && (
                <motion.div
                  initial={{ width: 0, opacity: 0 }}
                  animate={{ width: 340, opacity: 1 }}
                  exit={{ width: 0, opacity: 0 }}
                  transition={{ type: "spring", damping: 30, stiffness: 300 }}
                  className="h-full border-l border-white/10 bg-black/40 backdrop-blur-2xl flex flex-col shrink-0"
                >
                  <div className="p-5 border-b border-white/5 flex items-center justify-between">
                    <div className="text-sm font-bold text-white tracking-wider">播放队列 <span className="text-gray-500 font-normal ml-1 bg-white/10 px-2 py-0.5 rounded-full text-xs">{playerState.playlist.length} 首</span></div>
                    <button onClick={() => setIsPlaylistOpen(false)} className="w-8 h-8 flex items-center justify-center hover:bg-white/10 rounded-full text-gray-400 hover:text-white transition-colors">
                      <X size={18} />
                    </button>
                  </div>
                  
                  <div className="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-1">
                    {playerState.playlist.map((t, index) => {
                      const isCurrent = index === playerState.currentIndex;
                      return (
                        <div 
                          key={`${t.id}-${index}`}
                          className={cn(
                            "flex items-center gap-3 p-2.5 rounded-xl cursor-pointer group transition-all",
                            isCurrent ? "bg-white/10 shadow-sm border border-white/5" : "hover:bg-white/5 border border-transparent"
                          )}
                          onDoubleClick={() => musicService.playTrackFromList(index)}
                        >
                          <div className="w-10 h-10 rounded-md shrink-0 overflow-hidden relative shadow-md">
                            <img src={t.coverUrl} className="w-full h-full object-cover opacity-90 group-hover:opacity-100 transition-opacity" />
                            {isCurrent && (
                              <div className="absolute inset-0 bg-black/50 flex items-center justify-center backdrop-blur-[2px]">
                                {playerState.isPlaying ? (
                                  <div className="flex gap-0.5 h-3 items-end">
                                    <div className="w-[3px] bg-[#1db954] animate-[bounce_1s_infinite] rounded-full" />
                                    <div className="w-[3px] bg-[#1db954] animate-[bounce_1.2s_infinite] rounded-full" style={{ animationDelay: '0.2s' }} />
                                    <div className="w-[3px] bg-[#1db954] animate-[bounce_0.8s_infinite] rounded-full" style={{ animationDelay: '0.4s' }} />
                                  </div>
                                ) : (
                                  <Play size={14} className="text-[#1db954] translate-x-px" fill="currentColor" />
                                )}
                              </div>
                            )}
                            
                            {/* Overlay Play button on hover for non-current tracks */}
                            {!isCurrent && (
                              <div className="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity" onClick={(e) => { e.stopPropagation(); musicService.playTrackFromList(index); }}>
                                 <Play size={16} fill="currentColor" className="text-white translate-x-px" />
                              </div>
                            )}
                          </div>
                          <div className="flex-1 min-w-0 pr-2">
                            <div className={cn("text-[13px] truncate mb-0.5", isCurrent ? "text-[#1db954] font-bold" : "text-white/90 font-medium")}>{t.title}</div>
                            <div className="text-[11px] text-gray-500/80 truncate font-medium">{t.artist}</div>
                          </div>
                        </div>
                    )})}
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          {/* Bottom Premium Control Bar */}
          <div className="relative z-20 h-[104px] shrink-0 bg-[#0A0A0A]/90 backdrop-blur-3xl border-t border-white/10 px-8 flex flex-col justify-center shadow-[0_-10px_30px_rgba(0,0,0,0.5)]">
             
             {/* Embedded Smooth Progress Bar */}
             <div className="absolute top-[-1px] left-0 right-0 h-[3px] bg-white/5 group cursor-pointer" ref={progressBarRef} onClick={handleSeek}>
               <div 
                 className="h-full bg-gradient-to-r from-[#1db954] to-[#2ce86f] relative group-hover:to-[#1db954] group-hover:h-[5px] transition-all duration-150 ease-out -translate-y-px"
                 style={{ width: `${playerState.duration > 0 ? (playerState.progress / playerState.duration) * 100 : 0}%` }}
               >
                 <div className="absolute right-0 top-1/2 -translate-y-1/2 w-3.5 h-3.5 bg-white rounded-full shadow-[0_0_10px_rgba(0,0,0,0.5)] opacity-0 group-hover:opacity-100 transition-opacity translate-x-1/2" />
               </div>
             </div>

             <div className="flex items-center justify-between w-full">
                {/* Left Mini Info */}
                <div className="flex items-center gap-4 w-1/3 min-w-[220px]">
                  <div className="w-14 h-14 rounded-lg overflow-hidden shrink-0 shadow-md border border-white/10 relative group bg-black/20">
                    <img src={track.coverUrl} className="w-full h-full object-cover" />
                    <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity cursor-pointer">
                      <Maximize2 size={16} className="text-white" />
                    </div>
                  </div>
                  <div className="flex flex-col min-w-0">
                    <span className="text-[14px] font-bold text-white truncate drop-shadow-sm">{track.title}</span>
                    <span className="text-[12px] text-gray-400 font-medium truncate mt-0.5">{track.artist}</span>
                  </div>
                  <button className="text-gray-400 hover:text-[#1db954] ml-2 transition-colors focus:outline-none" onClick={(e) => {
                     e.currentTarget.classList.toggle('text-[#1db954]');
                     e.currentTarget.querySelector('svg')?.classList.toggle('fill-[#1db954]');
                     toast('已添加到我喜欢的音乐', 'success');
                  }}>
                     <Heart size={18}/>
                  </button>
                </div>

                {/* Center Core Controls */}
                <div className="flex flex-col items-center justify-center flex-1">
                   <div className="flex items-center gap-7">
                     <button 
                       className={cn("text-gray-400 hover:text-white transition-colors focus:outline-none", playerState.playMode === 'shuffle' && "text-[#1db954] hover:text-[#2ce86f]")} 
                       onClick={() => musicService.togglePlayMode()}
                       title="随机播放"
                     >
                       <Shuffle size={18} />
                     </button>
                     <button className="text-gray-300 hover:text-white transition-colors hover:scale-110 active:scale-95 focus:outline-none" onClick={() => musicService.playPrev()}>
                       <SkipBack size={26} fill="currentColor" />
                     </button>
                     <button 
                       className="w-12 h-12 bg-white text-black rounded-full flex items-center justify-center shadow-[0_0_20px_rgba(255,255,255,0.2)] hover:scale-105 active:scale-95 hover:bg-gray-100 transition-all focus:outline-none"
                       onClick={() => musicService.togglePlay()}
                     >
                       {playerState.isPlaying ? <Pause size={24} fill="currentColor" /> : <Play size={24} fill="currentColor" className="ml-1" />}
                     </button>
                     <button className="text-gray-300 hover:text-white transition-colors hover:scale-110 active:scale-95 focus:outline-none" onClick={() => musicService.playNext()}>
                       <SkipForward size={26} fill="currentColor" />
                     </button>
                     <button 
                       className={cn("text-gray-400 hover:text-white transition-colors focus:outline-none", playerState.playMode === 'loop' && "text-[#1db954] hover:text-[#2ce86f]")} 
                       onClick={() => musicService.togglePlayMode()}
                       title="循环播放"
                     >
                       {playerState.playMode === 'loop' ? <RotateCcw size={18} /> : <Repeat size={18} />}
                     </button>
                   </div>
                   
                   <div className="flex items-center justify-center gap-1.5 mt-2 transition-opacity duration-200">
                     <span className="text-[11px] text-gray-400 font-mono font-medium">{formatDuration(playerState.progress)}</span>
                     <span className="text-[11px] text-gray-600 font-mono font-medium">/</span>
                     <span className="text-[11px] text-gray-500 font-mono font-medium">{formatDuration(playerState.duration)}</span>
                   </div>
                </div>

                {/* Right Controls (Volume, Playlist Toggle) */}
                <div className="flex items-center justify-end gap-5 w-1/3 min-w-[220px]">
                   <button 
                     className={cn("transition-colors focus:outline-none hover:scale-110 active:scale-95", isPlaylistOpen ? "text-[#1db954]" : "text-gray-400 hover:text-white")}
                     onClick={() => setIsPlaylistOpen(!isPlaylistOpen)}
                     title="播放队列"
                   >
                     <ListMusic size={20} />
                   </button>
                   <div 
                     className="flex items-center gap-3 group w-28"
                     onMouseEnter={() => setIsHoveringVolume(true)}
                     onMouseLeave={() => setIsHoveringVolume(false)}
                   >
                      <button className="text-gray-400 hover:text-white transition-colors focus:outline-none" onClick={() => musicService.toggleMute()}>
                         {playerState.isMuted || playerState.volume === 0 ? <VolumeX size={18} /> : 
                          playerState.volume < 0.5 ? <Volume2 size={18} /> : <Volume2 size={18} />}
                      </button>
                      <div 
                        className="flex-1 h-1.5 bg-white/10 rounded-full overflow-hidden cursor-pointer relative" 
                        onClick={(e) => {
                          const rect = e.currentTarget.getBoundingClientRect();
                          musicService.setVolume((e.clientX - rect.left) / rect.width);
                        }}
                      >
                        <div 
                          className="absolute left-0 top-0 bottom-0 bg-gray-300 group-hover:bg-[#1db954] rounded-full transition-colors" 
                          style={{ width: `${playerState.isMuted ? 0 : playerState.volume * 100}%` }} 
                        />
                      </div>
                   </div>
                </div>
             </div>
          </div>
        </motion.div>
      </motion.div>
      )}
    </AnimatePresence>
  );
};
