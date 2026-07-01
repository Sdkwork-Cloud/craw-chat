import React, { useState, useEffect, useRef, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Phone, Video, Mic, MicOff, VideoOff, MonitorUp, PhoneOff, Maximize, Minimize, Smartphone, Monitor } from 'lucide-react';
import { Avatar, formatDuration } from '@sdkwork/im-pc-commons';
import { toast } from './Toast';
import { callService, type SdkworkCallSnapshot } from '../services/CallService';

export type CallType = 'voice' | 'video';
type CallOverlayPhase = 'incoming-ringing' | 'outgoing-ringing' | 'connected' | 'finished' | 'idle';

function stopMediaStream(stream: MediaStream | undefined): void {
  stream?.getTracks().forEach((track) => {
    track.stop();
  });
}

function readErrorMessage(error: unknown): string | undefined {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return typeof error === 'string' && error.trim().length > 0 ? error : undefined;
}

interface CallOverlayProps {
  conversationId: string;
  isOpen: boolean;
  mode?: 'incoming' | 'outgoing';
  rtcSessionId?: string;
  type: CallType;
  callerName: string;
  callerAvatar: string;
  onClose: () => void;
}

export const CallOverlay: React.FC<CallOverlayProps> = ({
  conversationId,
  isOpen,
  mode = 'outgoing',
  rtcSessionId,
  type,
  callerName,
  callerAvatar,
  onClose,
}) => {
  const { t } = useTranslation();
  const [callState, setCallState] = useState<'ringing' | 'connected'>('ringing');
  const [callSnapshot, setCallSnapshot] = useState<SdkworkCallSnapshot>(callService.getSnapshot());
  const [activeRtcSessionId, setActiveRtcSessionId] = useState<string | undefined>(rtcSessionId);
  const activeRtcSessionIdRef = useRef<string | undefined>(rtcSessionId);
  const autoClosedTerminalSessionRef = useRef<string | undefined>(undefined);
  const localPreviewContainerRef = useRef<HTMLDivElement | null>(null);
  const screenShareStreamRef = useRef<MediaStream | undefined>(undefined);
  const isMutedRef = useRef(false);
  const isVideoOffRef = useRef(type === 'voice');
  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(type === 'voice');
  const [viewMode, setViewMode] = useState<'mobile' | 'desktop' | 'fullscreen'>('mobile');
  const [callDuration, setCallDuration] = useState(0);

  const releaseCallMedia = useCallback(() => {
    void callService.bindLocalVideoElement(null).catch(() => undefined);
    stopMediaStream(screenShareStreamRef.current);
    screenShareStreamRef.current = undefined;
  }, []);

  const closeOverlayWithMediaRelease = useCallback(() => {
    releaseCallMedia();
    onClose();
  }, [onClose, releaseCallMedia]);

  useEffect(() => {
    return callService.subscribe((snapshot) => {
      const snapshotIsTerminal = snapshot.state === 'ended'
        || snapshot.state === 'rejected'
        || snapshot.state === 'errored';
      const currentRtcSessionId = activeRtcSessionIdRef.current;
      const snapshotMatchesPropSession = Boolean(rtcSessionId && snapshot.rtcSessionId === rtcSessionId);
      const snapshotMatchesActiveSession = Boolean(currentRtcSessionId && snapshot.rtcSessionId === currentRtcSessionId);
      const snapshotMatchesPendingCall = !currentRtcSessionId
        && !snapshotIsTerminal
        && snapshot.conversationId === conversationId
        && snapshot.direction === mode;
      const shouldApplyUiState = snapshotMatchesPropSession
        || snapshotMatchesActiveSession
        || snapshotMatchesPendingCall
        || snapshot.state === 'idle';

      setCallSnapshot(snapshot);
      if (!shouldApplyUiState) {
        return;
      }
      if (snapshot.rtcSessionId && snapshot.rtcSessionId !== currentRtcSessionId) {
        activeRtcSessionIdRef.current = snapshot.rtcSessionId;
        setActiveRtcSessionId(snapshot.rtcSessionId);
      }
      isMutedRef.current = snapshot.isAudioMuted;
      isVideoOffRef.current = snapshot.isVideoMuted;
      setIsMuted(snapshot.isAudioMuted);
      setIsVideoOff(snapshot.isVideoMuted);
      setCallState(snapshot.state === 'connected' ? 'connected' : 'ringing');
      if (snapshot.state === 'errored' && snapshot.errorMessage) {
        toast(snapshot.errorMessage, 'error');
      }
      if (snapshot.state === 'ended' || snapshot.state === 'rejected' || snapshot.state === 'errored') {
        setCallDuration(0);
        if (snapshot.rtcSessionId && snapshot.rtcSessionId !== autoClosedTerminalSessionRef.current) {
          autoClosedTerminalSessionRef.current = snapshot.rtcSessionId;
          if (isOpen) {
            closeOverlayWithMediaRelease();
          }
        }
      }
    });
  }, [
    closeOverlayWithMediaRelease,
    conversationId,
    isOpen,
    mode,
    rtcSessionId,
  ]);

  // Reset state when opened and start the SDK-backed outgoing call when needed.
  useEffect(() => {
    if (!isOpen) {
      releaseCallMedia();
      return;
    }

    setCallState('ringing');
    isMutedRef.current = false;
    isVideoOffRef.current = type === 'voice';
    setIsMuted(false);
    setIsVideoOff(type === 'voice');
    autoClosedTerminalSessionRef.current = undefined;
    activeRtcSessionIdRef.current = rtcSessionId;
    setActiveRtcSessionId(rtcSessionId);
    setCallDuration(0);
    setViewMode('mobile'); // Default to mobile mode

    if (mode === 'outgoing' && !rtcSessionId) {
      void callService.startOutgoingCall({
        conversationId,
        targetName: callerName,
        type,
      });
    }
  }, [callerName, conversationId, isOpen, mode, releaseCallMedia, rtcSessionId, type]);

  useEffect(() => {
    if (!isOpen || callState !== 'connected' || type !== 'video' || isVideoOff) {
      void callService.bindLocalVideoElement(null).catch(() => undefined);
      return;
    }
    void callService.bindLocalVideoElement(localPreviewContainerRef.current).catch((error) => {
      toast(error instanceof Error ? error.message : t('chat.callOverlay.toast.localVideoBindFailed'), 'error');
    });
    return () => {
      void callService.bindLocalVideoElement(null).catch(() => undefined);
    };
  }, [callState, isOpen, isVideoOff, type]);

  useEffect(() => {
    return () => {
      releaseCallMedia();
    };
  }, [releaseCallMedia]);

  // Timer for connected state
  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (callState === 'connected') {
      interval = setInterval(() => {
        setCallDuration(prev => prev + 1);
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [callState]);

  const getContainerClasses = () => {
    switch (viewMode) {
      case 'fullscreen':
        return 'inset-0 rounded-none';
      case 'desktop':
        return 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[500px] rounded-2xl border border-white/10';
      case 'mobile':
      default:
        return 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[360px] h-[680px] rounded-[3rem] border-[8px] border-[#222] shadow-2xl';
    }
  };

  const isMobile = viewMode === 'mobile';
  const controlBtnClass = isMobile ? 'w-12 h-12' : 'w-14 h-14';
  const hangupBtnClass = isMobile ? 'w-14 h-14 ml-1' : 'w-16 h-16 ml-4';
  const iconSize = isMobile ? 20 : 24;
  const hangupIconSize = isMobile ? 24 : 28;
  const isTerminalCall = callSnapshot.state === 'ended'
    || callSnapshot.state === 'rejected'
    || callSnapshot.state === 'errored';
  const snapshotMatchesActiveSession = Boolean(activeRtcSessionId && callSnapshot.rtcSessionId === activeRtcSessionId);
  const snapshotMatchesPendingCall = !activeRtcSessionId
    && !isTerminalCall
    && callSnapshot.conversationId === conversationId
    && callSnapshot.direction === mode;
  const isCurrentCallSnapshot = snapshotMatchesActiveSession || snapshotMatchesPendingCall;
  const isCurrentTerminalCall = isCurrentCallSnapshot && isTerminalCall;
  const isConnectedCall = isCurrentCallSnapshot && callSnapshot.state === 'connected';
  const isIncomingRinging = mode === 'incoming'
    && callState === 'ringing'
    && !isConnectedCall
    && !isCurrentTerminalCall;
  const isOutgoingRinging = mode === 'outgoing'
    && callState === 'ringing'
    && !isConnectedCall
    && !isCurrentTerminalCall;
  const callOverlayPhase: CallOverlayPhase = isConnectedCall
    ? 'connected'
    : isCurrentTerminalCall
      ? 'finished'
      : isIncomingRinging
        ? 'incoming-ringing'
        : isOutgoingRinging
          ? 'outgoing-ringing'
          : 'idle';
  const showAcceptAction = callOverlayPhase === 'incoming-ringing';
  const showRejectAction = callOverlayPhase === 'incoming-ringing';
  const showCancelAction = callOverlayPhase === 'outgoing-ringing';
  const showHangupAction = callOverlayPhase === 'connected';
  const showCloseAction = callOverlayPhase === 'finished';
  const canControlLocalMedia = callOverlayPhase === 'connected' || callOverlayPhase === 'outgoing-ringing';
  const canToggleAudio = canControlLocalMedia;
  const canToggleVideo = canControlLocalMedia && type === 'video';
  const canShareScreen = callOverlayPhase === 'connected' && type === 'video';
  const displayNameClass = isMobile ? 'max-w-[280px]' : 'max-w-[520px]';
  const audioStatusText = isMuted ? t('chat.callOverlay.media.micOff') : t('chat.callOverlay.media.micOn');
  const videoStatusText = isVideoOff ? t('chat.callOverlay.media.cameraOff') : t('chat.callOverlay.media.cameraOn');
  const localMediaStatusText = type === 'video'
    ? `${audioStatusText} · ${videoStatusText}`
    : audioStatusText;
  const shouldShowLocalMediaStatus = callOverlayPhase === 'outgoing-ringing' && canControlLocalMedia;
  const statusText = isCurrentCallSnapshot && callSnapshot.state === 'connecting'
    ? t('chat.callOverlay.status.connecting')
    : isCurrentCallSnapshot && callSnapshot.state === 'errored'
      ? t('chat.callOverlay.status.connectionFailed')
      : isCurrentCallSnapshot && callSnapshot.state === 'rejected'
        ? t('chat.callOverlay.status.rejected')
        : isCurrentCallSnapshot && callSnapshot.state === 'ended'
          ? t('chat.callOverlay.status.ended')
          : isOutgoingRinging
            ? t('chat.callOverlay.status.waitingAnswer')
            : t('chat.callOverlay.status.inviting');

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          transition={{ duration: 0.2, ease: "easeOut" }}
          className={`fixed z-50 flex flex-col overflow-hidden shadow-2xl bg-[#181818] ${getContainerClasses()}`}
        >
        {/* Header */}
        <div className="absolute top-0 left-0 right-0 p-4 flex justify-between items-center z-30 bg-gradient-to-b from-black/60 to-transparent">
          <div className="text-white font-medium text-lg drop-shadow-md">
            {type === 'video' ? t('chat.callOverlay.videoCall') : t('chat.callOverlay.voiceCall')}
          </div>
          <div className="flex items-center gap-2">
            {viewMode !== 'fullscreen' && (
              <button 
                onClick={() => setViewMode(isMobile ? 'desktop' : 'mobile')}
                className="w-8 h-8 flex items-center justify-center rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-sm"
                title={isMobile ? t('chat.callOverlay.view.switchToDesktop') : t('chat.callOverlay.view.switchToMobile')}
              >
                {isMobile ? <Monitor size={16} /> : <Smartphone size={16} />}
              </button>
            )}
            <button 
              onClick={() => setViewMode(viewMode === 'fullscreen' ? 'desktop' : 'fullscreen')}
              className="w-8 h-8 flex items-center justify-center rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-sm"
              title={viewMode === 'fullscreen' ? t('chat.callOverlay.view.exitFullscreen') : t('chat.callOverlay.view.enterFullscreen')}
            >
              {viewMode === 'fullscreen' ? <Minimize size={16} /> : <Maximize size={16} />}
            </button>
          </div>
        </div>

        {/* Main Content Area */}
        <div className="flex-1 relative flex items-center justify-center bg-[#111]">
          {/* Background Blur for Voice Call or Video Off */}
          {(type === 'voice' || isVideoOff) && (
            <div 
              className="absolute inset-0 bg-cover bg-center opacity-20 blur-3xl"
              style={{ backgroundImage: `url(${callerAvatar})` }}
            />
          )}

          {/* Caller Info (Ringing or Voice Call) */}
          {(callState === 'ringing' || type === 'voice' || isVideoOff) ? (
            <div className="relative z-10 flex flex-col items-center">
              <motion.div
                animate={callState === 'ringing' ? { scale: [1, 1.1, 1] } : {}}
                transition={{ repeat: Infinity, duration: 2 }}
                className="relative"
              >
                <Avatar 
                  src={callerAvatar} 
                  alt={callerName} 
                  className={`${isMobile ? 'w-24 h-24' : 'w-32 h-32'} rounded-full border-4 border-white/10 shadow-2xl transition-all`}
                />
                {callState === 'ringing' && (
                  <div className="absolute inset-0 rounded-full border-2 border-blue-500 animate-ping opacity-75" />
                )}
              </motion.div>
              <h2
                className={`mt-6 truncate text-center font-medium text-white drop-shadow-lg ${displayNameClass} ${isMobile ? 'text-2xl' : 'text-3xl'}`}
                title={callerName}
              >
                {callerName}
              </h2>
              <p className={`mt-2 text-gray-400 ${isMobile ? 'text-base' : 'text-lg'}`}>
                {callState === 'ringing' ? statusText : formatDuration(callDuration)}
              </p>
              {shouldShowLocalMediaStatus && (
                <p
                  className={`mt-1 truncate text-center text-gray-500 ${displayNameClass} ${isMobile ? 'text-xs' : 'text-sm'}`}
                  title={localMediaStatusText}
                >
                  {localMediaStatusText}
                </p>
              )}
            </div>
          ) : (
            /* Simulated Video Feed */
            <div className="absolute inset-0 bg-[#222] flex items-center justify-center">
              {/* Main Video (Remote) */}
              <div className="absolute inset-0 flex items-center justify-center">
                <Avatar src={callerAvatar} alt={callerName} className="w-full h-full object-cover opacity-50 blur-sm" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-white/30 text-2xl">{t('chat.callOverlay.media.remoteVideo')}</span>
                </div>
              </div>
              
              {/* Picture in Picture (Local) */}
              <div className={`absolute bg-black overflow-hidden shadow-2xl z-20 transition-all duration-300 ${
                isMobile 
                  ? 'top-20 right-4 w-24 h-36 rounded-lg border border-white/20' 
                  : 'bottom-24 right-6 w-48 h-72 rounded-xl border-2 border-white/20'
              }`}>
                {type === 'video' && !isVideoOff && (
                  <div
                    ref={localPreviewContainerRef}
                    className="absolute inset-0 h-full w-full object-cover"
                  />
                )}
                {(type !== 'video' || isVideoOff) && (
                  <div className="absolute inset-0 flex items-center justify-center text-white/30">
                    {t('chat.callOverlay.media.self')}
                  </div>
                )}
                {type === 'video' && (
                  <div className="absolute left-2 right-2 bottom-2 text-center">
                    <span
                      className="block truncate rounded-full bg-black/50 px-2 py-1 text-[10px] text-white/70"
                      title={localMediaStatusText}
                    >
                      {localMediaStatusText}
                    </span>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        {/* Controls Bar */}
        <div className={`absolute bottom-0 left-0 right-0 flex justify-center items-center bg-gradient-to-t from-black/80 to-transparent z-30 ${isMobile ? 'p-6 gap-3' : 'p-6 gap-6'}`}>
          {showAcceptAction && (
            <button
              onClick={() => {
                void callService.acceptIncomingCall()
                  .catch((error) => {
                    toast(error instanceof Error ? error.message : t('chat.callOverlay.toast.acceptFailed'), 'error');
                  });
              }}
              className={`${hangupBtnClass} rounded-full bg-emerald-500 hover:bg-emerald-600 text-white flex items-center justify-center shadow-lg shadow-emerald-500/20 transition-all hover:scale-105`}
              title={t('chat.callOverlay.actions.accept')}
            >
              <Phone size={hangupIconSize} />
            </button>
          )}
          {canToggleAudio && (
            <button
              onClick={() => {
                const nextMuted = !isMuted;
                isMutedRef.current = nextMuted;
                setIsMuted(nextMuted);
                void callService.setAudioMuted(nextMuted).catch((error) => {
                  toast(error instanceof Error ? error.message : t('chat.callOverlay.toast.muteFailed'), 'error');
                });
              }}
              className={`${controlBtnClass} rounded-full flex items-center justify-center transition-all ${
                isMuted ? 'bg-white text-black' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md'
              }`}
              title={isMuted ? t('chat.callOverlay.actions.unmute') : t('chat.callOverlay.actions.mute')}
            >
              {isMuted ? <MicOff size={iconSize} /> : <Mic size={iconSize} />}
            </button>
          )}

          {canToggleVideo && (
            <button
              onClick={() => {
                const nextVideoOff = !isVideoOff;
                isVideoOffRef.current = nextVideoOff;
                setIsVideoOff(nextVideoOff);
                void callService.setVideoMuted(nextVideoOff).catch((error) => {
                  toast(error instanceof Error ? error.message : t('chat.callOverlay.toast.videoFailed'), 'error');
                });
              }}
              className={`${controlBtnClass} rounded-full flex items-center justify-center transition-all ${
                isVideoOff ? 'bg-white text-black' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md'
              }`}
              title={isVideoOff ? t('chat.callOverlay.actions.enableVideo') : t('chat.callOverlay.actions.disableVideo')}
            >
              {isVideoOff ? <VideoOff size={iconSize} /> : <Video size={iconSize} />}
            </button>
          )}

          {canShareScreen && (
            <button
              className={`${controlBtnClass} rounded-full bg-white/10 hover:bg-white/20 text-white backdrop-blur-md flex items-center justify-center transition-all`}
              title={t('chat.callOverlay.actions.shareScreen')}
              onClick={async () => {
                if (navigator.mediaDevices && navigator.mediaDevices.getDisplayMedia) {
                   try {
                      stopMediaStream(screenShareStreamRef.current);
                      const stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
                      screenShareStreamRef.current = stream;
                      toast(t('chat.callOverlay.toast.screenShareStarted'), 'success');
                      stream.getVideoTracks().forEach((track) => {
                        track.onended = () => {
                         if (screenShareStreamRef.current === stream) {
                            screenShareStreamRef.current = undefined;
                         }
                         toast(t('chat.callOverlay.toast.screenShareEnded'), 'success');
                        };
                      });
                   } catch (error) {
                      if (readErrorMessage(error)?.includes('display-capture')) {
                         toast(t('chat.callOverlay.toast.screenShareDenied'), 'error');
                      } else {
                         toast(t('chat.callOverlay.toast.screenShareCancelled'), 'success');
                      }
                   }
                } else {
                   toast(t('chat.callOverlay.toast.screenShareUnsupported'), 'error');
                }
              }}
            >
              <MonitorUp size={iconSize} />
            </button>
          )}

          <button 
            onClick={() => {
              if (showCloseAction) {
                closeOverlayWithMediaRelease();
                return;
              }
              const task = showRejectAction
                ? callService.rejectIncomingCall({ reason: 'user_reject' })
                : showCancelAction || showHangupAction
                  ? callService.endCall({ reason: 'user_hangup' })
                  : Promise.resolve();
              void task.finally(closeOverlayWithMediaRelease);
            }}
            className={`${hangupBtnClass} rounded-full bg-red-500 hover:bg-red-600 text-white flex items-center justify-center shadow-lg shadow-red-500/20 transition-all hover:scale-105`}
            title={
              callOverlayPhase === 'incoming-ringing'
                ? t('chat.callOverlay.actions.reject')
                : callOverlayPhase === 'outgoing-ringing'
                  ? t('chat.callOverlay.actions.cancel')
                  : callOverlayPhase === 'connected'
                    ? t('chat.callOverlay.actions.hangup')
                    : t('chat.callOverlay.actions.close')
            }
          >
            <PhoneOff size={hangupIconSize} />
          </button>
        </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
