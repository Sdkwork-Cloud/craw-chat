import React, { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { cn, sanitizeEnterpriseWebsiteUrl, sanitizeMessageLinkHref, sanitizeMessageUrl } from '@sdkwork/im-pc-commons';
import type { Message } from '@sdkwork/im-pc-types';
import { Play, FileText, LayoutTemplate, Volume2, Phone, X, Music } from 'lucide-react';
import { Avatar } from '@sdkwork/im-pc-commons';
import { musicService, PlayerState } from '../services/MusicService';
import { DEFAULT_MUSIC_COVER_URL } from '../services/DefaultAvatarService';
import { toast } from './Toast';

interface BaseProps {
  msg: Message;
  isMe: boolean;
  onClick?: () => void;
  onMediaClick?: (msg: Message) => void;
}

import ReactMarkdown from 'react-markdown';

export const TextMessageItem: React.FC<BaseProps> = ({ msg, isMe }) => (
  <div className={cn(
    "text-[14px] leading-relaxed whitespace-pre-wrap break-words px-4 py-2.5 shadow-sm max-w-[500px] xl:max-w-[700px]",
    isMe ? "bg-[#00b42a] text-white rounded-2xl rounded-tr-sm" : "bg-[#2b2b2d] text-gray-200 rounded-2xl rounded-tl-sm"
  )}>
    <div className={cn("prose prose-sm prose-p:leading-snug prose-p:my-1 prose-headings:my-2 prose-ul:my-1", isMe ? "prose-invert prose-p:text-white" : "prose-invert")}>
      <ReactMarkdown
        disallowedElements={['script', 'iframe', 'object', 'embed', 'style']}
        unwrapDisallowed
        components={{
          a: ({ href, children }) => {
            const safeHref = sanitizeMessageLinkHref(href ?? '');
            if (!safeHref) {
              return <span>{children}</span>;
            }
            return (
              <a href={safeHref} target="_blank" rel="noopener noreferrer">
                {children}
              </a>
            );
          },
        }}
      >
        {msg.content}
      </ReactMarkdown>
    </div>
  </div>
);

export const ImageMessageItem: React.FC<BaseProps> = ({ msg, onMediaClick }) => {
  const imageUrl = sanitizeMessageUrl(msg.content);
  if (!imageUrl) {
    return null;
  }

  return (
    <div 
      className="rounded-lg overflow-hidden max-w-[300px] mt-1 border border-white/10 shadow-sm cursor-pointer hover:opacity-90 transition-opacity" 
      onClick={() => onMediaClick?.(msg)}
    >
      <img src={imageUrl} alt="Image message" className="w-full h-auto" referrerPolicy="no-referrer" />
    </div>
  );
};

export const VideoMessageItem: React.FC<BaseProps> = ({ msg, onMediaClick }) => {
  const coverUrl = sanitizeMessageUrl(msg.coverUrl);
  if (!coverUrl) {
    return null;
  }

  return (
    <div 
      className="relative rounded-lg overflow-hidden max-w-[300px] mt-1 border border-white/10 shadow-sm cursor-pointer group bg-black" 
      onClick={() => onMediaClick?.(msg)}
    >
      <img src={coverUrl} className="w-full h-auto opacity-80 group-hover:opacity-60 transition-opacity" referrerPolicy="no-referrer" />
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="w-12 h-12 bg-black/50 rounded-full flex items-center justify-center text-white backdrop-blur-sm group-hover:scale-110 transition-transform shadow-lg">
          <Play size={24} className="ml-1" fill="currentColor" />
        </div>
      </div>
      {msg.duration && <div className="absolute bottom-2 right-2 text-white text-[10px] bg-black/60 px-1.5 py-0.5 rounded backdrop-blur-sm">{Math.floor(msg.duration / 60)}:{(msg.duration % 60).toString().padStart(2, '0')}</div>}
    </div>
  );
};

export const VoiceMessageItem: React.FC<BaseProps> = ({ msg, isMe }) => {
  const { t } = useTranslation();
  const [isPlaying, setIsPlaying] = useState(false);
  const [playPhase, setPlayPhase] = useState(2);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    let interval: number;
    if (isPlaying) {
      interval = window.setInterval(() => {
        setPlayPhase(p => (p + 1) % 3);
      }, 400);
    } else {
      setPlayPhase(2);
    }
    return () => clearInterval(interval);
  }, [isPlaying]);

  const togglePlay = () => {
    const audioUrl = sanitizeMessageUrl(msg.content);
    if (!audioUrl) {
      if (!isPlaying) {
        setIsPlaying(true);
        setTimeout(() => setIsPlaying(false), (msg.duration || 2) * 1000);
      }
      return;
    }
    
    if (audioRef.current) {
      if (isPlaying) {
        audioRef.current.pause();
        audioRef.current.currentTime = 0;
        setIsPlaying(false);
      } else {
        audioRef.current.play().catch(() => {
           setIsPlaying(false);
           toast(t('chat.messageItems.voice.playFailed'), 'error');
        });
        setIsPlaying(true);
      }
    }
  };

  return (
    <div 
      className={cn(
        "flex items-center gap-2 px-4 py-2 mt-1 rounded-2xl shadow-sm cursor-pointer active:scale-95 transition-transform min-w-[80px]",
        isMe ? "bg-[#00b42a] text-white rounded-tr-sm flex-row-reverse" : "bg-[#2b2b2d] text-gray-200 rounded-tl-sm border border-white/5",
        isPlaying && (isMe ? "bg-[#00a025]" : "bg-[#353538]")
      )} 
      style={{ width: `${Math.max(80, Math.min(240, (msg.duration || 1) * 10 + 60))}px` }} 
      onClick={togglePlay}
    >
      {sanitizeMessageUrl(msg.content) && (
        <audio 
          ref={audioRef} 
          src={sanitizeMessageUrl(msg.content) ?? undefined} 
          onEnded={() => setIsPlaying(false)} 
          className="hidden" 
        />
      )}
      <div className={cn("flex items-center justify-center relative w-5 h-5 opacity-80", isMe ? "rotate-180" : "")}>
        {/* Base dot */}
        <div className="absolute left-1 w-1 h-1 bg-current rounded-full" />
        {/* First arc */}
        <div className={cn("absolute left-2 w-1.5 h-2.5 border-r-[2px] border-y-[2px] border-y-transparent border-current rounded-r-full transition-opacity", playPhase >= 1 || !isPlaying ? "opacity-100" : "opacity-0")} />
        {/* Second arc */}
        <div className={cn("absolute left-3 w-2 h-4 border-r-[2.5px] border-y-[2.5px] border-y-transparent border-current rounded-r-full transition-opacity", playPhase >= 2 || !isPlaying ? "opacity-100" : "opacity-0")} />
      </div>
      {msg.duration && <span className="text-[14px] font-medium select-none">{msg.duration}''</span>}
    </div>
  );
};

export const VideoCallMessageItem: React.FC<BaseProps> = ({ msg, isMe }) => {
  const { t } = useTranslation();
  return (
  <div className={cn(
    "flex items-center gap-2 px-4 py-2 mt-1 rounded-2xl shadow-sm",
    isMe ? "bg-[#00b42a] text-white rounded-tr-sm" : "bg-[#2b2b2d] text-gray-200 rounded-tl-sm border border-white/5"
  )}>
    <Phone size={16} />
    <span className="text-[14px]">{msg.content || t('chat.messageItems.videoCall.endedFallback')} {msg.duration ? `${Math.floor(msg.duration/60)}:${(msg.duration%60).toString().padStart(2, '0')}` : ''}</span>
  </div>
  );
};

export const LinkMessageItem: React.FC<BaseProps> = ({ msg }) => {
  const linkHref = sanitizeMessageLinkHref(msg.content);
  const coverUrl = sanitizeMessageUrl(msg.coverUrl);
  if (!linkHref) {
    return null;
  }

  return (
  <a href={linkHref} target="_blank" rel="noopener noreferrer" className="block w-[280px] bg-[#2b2b2d] rounded-xl border border-white/10 p-3 mt-1 cursor-pointer hover:bg-white/5 transition-colors">
    <div className="text-[14px] text-gray-200 font-medium line-clamp-2 mb-2">{msg.fileName}</div>
    <div className="flex gap-2 h-12">
      <div className="flex-1 text-[12px] text-gray-500 line-clamp-3 leading-snug">{msg.desc}</div>
      {coverUrl ? (
        <img src={coverUrl} className="w-12 h-12 rounded object-cover shrink-0 bg-[#3b3b3d]" referrerPolicy="no-referrer" />
      ) : null}
    </div>
  </a>
  );
};

export const AppletMessageItem: React.FC<BaseProps> = ({ msg }) => {
  const { t } = useTranslation();
  return (
  <div className="w-[280px] bg-[#2b2b2d] rounded-xl border border-white/10 p-4 mt-1 cursor-pointer hover:bg-white/5 transition-colors flex flex-col gap-3" onClick={() => toast(t('chat.messageItems.applet.loading', { name: msg.fileName }), 'success')}>
    <div className="flex items-center gap-2 text-gray-400">
      <LayoutTemplate size={16} />
      <span className="text-[12px] font-medium uppercase tracking-widest leading-none mt-0.5">{t('chat.messageItems.applet.label')}</span>
    </div>
    <div className="flex items-center gap-2 mb-1">
      {msg.appIcon ? (
        <img
          src={sanitizeMessageUrl(msg.appIcon) ?? undefined}
          className="w-5 h-5 rounded-full"
          referrerPolicy="no-referrer"
        />
      ) : null}
      <span className="text-[14px] text-gray-200">{msg.fileName}</span>
    </div>
    <div className="text-[16px] text-gray-100 font-medium line-clamp-2 pb-1">{msg.desc}</div>
    <img src={sanitizeMessageUrl(msg.coverUrl) ?? undefined} className="w-full h-36 rounded-lg object-cover" referrerPolicy="no-referrer" />
  </div>
  );
};

const LegacyCardMessageItem: React.FC<BaseProps> = ({ msg }) => {
  const { t } = useTranslation();
  return (
  <div className="w-[260px] bg-[#2b2b2d] border border-white/10 rounded-xl overflow-hidden mt-1 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => {
     toast(t('chat.messageItems.card.addedToContacts', { name: msg.fileName }), 'success');
  }}>
    <div className="p-4 flex gap-4 border-b border-white/10">
      <Avatar src={sanitizeMessageUrl(msg.appIcon) ?? undefined} className="w-12 h-12 rounded-lg bg-[#3b3b3d]" />
      <div className="flex flex-col justify-center min-w-0">
        <div className="text-[15px] text-gray-200 truncate">{msg.fileName}</div>
        <div className="text-[12px] text-gray-500 truncate mt-0.5">{msg.desc}</div>
      </div>
    </div>
    <div className="px-4 py-1.5 text-[10px] text-gray-500 uppercase tracking-widest bg-black/20">{t('chat.messageItems.card.personalLabel')}</div>
  </div>
  );
};

function formatCardDescription(message: Message, translate: (key: string) => string): string | undefined {
  if (message.desc?.startsWith('group-invite:')) {
    return translate('chat.messageItems.card.openGroupChat');
  }
  return message.desc;
}

export const CardMessageItem: React.FC<BaseProps> = ({ msg, onClick }) => {
  const { t } = useTranslation();
  if (!onClick) {
    return <LegacyCardMessageItem msg={msg} isMe={false} />;
  }

  return (
    <button
      type="button"
      className="w-[260px] bg-[#2b2b2d] border border-white/10 rounded-xl overflow-hidden mt-1 cursor-pointer hover:bg-white/5 transition-colors text-left"
      onClick={onClick}
    >
      <div className="p-4 flex gap-4 border-b border-white/10">
        <Avatar src={sanitizeMessageUrl(msg.appIcon) ?? undefined} className="w-12 h-12 rounded-lg bg-[#3b3b3d]" />
        <div className="flex flex-col justify-center min-w-0">
          <div className="text-[15px] text-gray-200 truncate">{msg.fileName}</div>
          <div className="text-[12px] text-gray-500 truncate mt-0.5">{formatCardDescription(msg, t)}</div>
        </div>
      </div>
      <div className="px-4 py-1.5 text-[10px] text-gray-500 uppercase tracking-widest bg-black/20">{t('chat.messageItems.card.groupInviteLabel')}</div>
    </button>
  );
};

export const FileMessageItem: React.FC<BaseProps> = ({ msg, isMe }) => {
  const { t } = useTranslation();
  return (
  <div className={cn(
    "flex w-[260px] items-center gap-4 px-4 py-3 mt-1 rounded-xl shadow-sm cursor-pointer hover:brightness-110 transition-all",
    isMe ? "bg-[#00b42a] text-white" : "bg-[#2b2b2d] text-gray-200 border border-white/5 hover:bg-white/5"
  )} onClick={() => {
    const downloadUrl = sanitizeMessageUrl(msg.content);
    if (downloadUrl) {
        const a = document.createElement('a');
        a.href = downloadUrl;
        a.download = msg.fileName || 'download';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        toast(t('chat.messageItems.file.downloadStarted', { name: msg.fileName }), 'success');
    } else {
        toast(t('chat.messageItems.file.resourceNotFound', { name: msg.fileName }), 'error');
    }
  }}>
    <div className="flex-1 min-w-0">
      <div className="text-[14px] font-medium truncate mb-1">{msg.fileName}</div>
      <div className={cn("text-[12px]", isMe ? "text-green-100/80" : "text-gray-500")}>{msg.fileSize}</div>
    </div>
    <div className={cn("w-10 h-10 rounded-lg flex items-center justify-center shrink-0 shadow-sm", isMe ? "bg-white/20" : "bg-[#3b3b3d]")}>
      <FileText size={20} className={isMe ? "text-white" : "text-indigo-400"} />
    </div>
  </div>
  );
};

export const MusicMessageItem: React.FC<BaseProps & { allMessages?: Message[] }> = ({ msg, isMe, allMessages = [] }) => {
  const [playerState, setPlayerState] = useState<PlayerState>(musicService.getState());

  useEffect(() => {
    const unsubscribe = musicService.subscribe(setPlayerState);
    return unsubscribe;
  }, []);

  const isCurrentTrack = playerState.currentTrack?.id === msg.id;
  const isPlaying = isCurrentTrack && playerState.isPlaying;

  const playMusic = () => {
    const trackUrl = sanitizeMessageUrl(msg.content);
    if (!trackUrl) {
      toast('暂无播放源', 'error');
      return;
    }
    
    // Auto build playlist if not currently this track
    if (!isCurrentTrack && allMessages.length > 0) {
       const musicMessages = allMessages.filter(m => m.type === 'music');
       if (musicMessages.length > 0) {
           const playlist = musicMessages.flatMap(m => {
             const url = sanitizeMessageUrl(m.content);
             if (!url) {
               return [];
             }
             return [{
               id: m.id,
               url,
               title: m.fileName || '未知歌曲',
               artist: m.desc || '未知歌手',
               coverUrl: sanitizeMessageUrl(m.coverUrl) || DEFAULT_MUSIC_COVER_URL,
               album: '对话分享记录'
             }];
           });
           if (playlist.length === 0) {
             return;
           }
           const startIndex = playlist.findIndex(t => t.id === msg.id);
           musicService.setPlaylist(playlist, startIndex >= 0 ? startIndex : 0);
           return;
       }
    }

    musicService.play({
      id: msg.id,
      url: trackUrl,
      title: msg.fileName || '未知歌曲',
      artist: msg.desc || '未知歌手',
      coverUrl: sanitizeMessageUrl(msg.coverUrl) ?? undefined,
      album: '对话分享记录'
    });
  };

  const togglePlay = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (isCurrentTrack) {
        musicService.togglePlay();
    } else {
        playMusic();
    }
  };

  const openPlayer = () => {
    if (!isCurrentTrack) {
      playMusic();
    } else if (!playerState.isPlayerOpen) {
      musicService.togglePlayer();
    }
  };

  return (
    <div 
      className="w-[280px] bg-[#2b2b2d] border border-white/10 rounded-xl overflow-hidden mt-1 cursor-pointer hover:bg-white/5 transition-colors group"
      onClick={openPlayer}
    >
      <div className="p-3 flex gap-3 border-b border-white/10 relative">
        <div className="relative w-14 h-14 shrink-0 rounded-lg overflow-hidden bg-[#3b3b3d]">
          <img src={msg.coverUrl || DEFAULT_MUSIC_COVER_URL} className={cn("w-full h-full object-cover transition-transform duration-1000", isPlaying ? "scale-110" : "")} referrerPolicy="no-referrer" />
          <div 
            className="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 sm:opacity-100 transition-opacity"
            onClick={togglePlay}
          >
            <div className="w-8 h-8 rounded-full bg-black/60 flex items-center justify-center text-white backdrop-blur-sm shadow-lg hover:scale-110 active:scale-95 transition-all">
              {isPlaying ? <span className="w-2.5 h-2.5 bg-white rounded-[1px]"></span> : <Play size={16} fill="currentColor" className="ml-0.5" />}
            </div>
          </div>
        </div>
        <div className="flex flex-col justify-center min-w-0 flex-1">
          <div className="text-[14px] text-gray-200 truncate font-medium">{msg.fileName || '未知歌曲'}</div>
          <div className="text-[12px] text-gray-500 truncate mt-1 flex items-center gap-1">
            <Music size={12} /> {msg.desc || '未知歌手'}
          </div>
        </div>
        {isPlaying && (
          <div className="absolute right-3 top-1/2 -translate-y-1/2 flex gap-0.5 items-end h-4">
            <div className="w-1 bg-[#00b42a] animate-[bounce_1s_infinite] h-full" style={{ animationDelay: '0ms' }} />
            <div className="w-1 bg-[#00b42a] animate-[bounce_1s_infinite] h-2/3" style={{ animationDelay: '100ms' }} />
            <div className="w-1 bg-[#00b42a] animate-[bounce_1s_infinite] h-4/5" style={{ animationDelay: '200ms' }} />
          </div>
        )}
      </div>
      <div className="px-4 py-1.5 text-[10px] text-gray-500 uppercase tracking-widest bg-black/20 flex justify-between items-center">
        <span>音乐卡片</span>
        <span>Claw Music</span>
      </div>
    </div>
  );
};
