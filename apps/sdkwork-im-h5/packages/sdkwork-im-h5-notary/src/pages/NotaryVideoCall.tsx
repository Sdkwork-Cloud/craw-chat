import React, { useState, useEffect } from "react";
import { useNavigate, useParams, useLocation } from "react-router";
import {
  Mic,
  MicOff,
  PhoneOff,
  Video,
  VideoOff,
  SwitchCamera,
  ShieldCheck,
  AlertCircle,
} from "lucide-react";
import { motion } from "motion/react";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { ChatService } from "@sdkwork/im-h5-chat";
import { GLOBAL_STORE } from "./CreateNotaryProcess";
import { useTranslation } from "react-i18next";

export const NotaryVideoCall: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const location = useLocation();
  const navigate = useNavigate();
  // We can extract party name or other details from query param or state if passed.
  // For demo, we just use a generic name based on the id.
  const isParty1 = id === "p1";
  const partyName = isParty1 ? `刘* (${t("notary.video_call.party")})` : t("notary.video_call.party");

  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(false);
  const [isFrontCamera, setIsFrontCamera] = useState(true);
  const [callState, setCallState] = useState<"calling" | "connected">(
    "calling",
  );
  const [duration, setDuration] = useState(0);

  // Simulate connection after 2 seconds
  useEffect(() => {
    if (callState === "calling") {
      const timer = setTimeout(() => {
        setCallState("connected");
      }, 2000);
      return () => clearTimeout(timer);
    }
  }, [callState]);

  // Timer for connected state
  useEffect(() => {
    if (callState === "connected") {
      const interval = setInterval(() => {
        setDuration((prev) => prev + 1);
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [callState]);

  const formatDuration = (seconds: number) => {
  const m = Math.floor(seconds / 60)
      .toString()
      .padStart(2, "0");
    const s = (seconds % 60).toString().padStart(2, "0");
    return `${m}:${s}`;
  };

  const handleHangUp = async () => {
    try {
      const notaryId = GLOBAL_STORE.selectedNotary;
      if (notaryId) {
        // Find existing direct chat with the notary
        const chats = await ChatService.getChats();
        let targetChat = chats.find(c => c.type === 'direct' && c.participants.some(p => p.id === notaryId));
        
        if (!targetChat) {
          // Add a generic mock logic if no existing chat
          targetChat = await ChatService.createDirectChat({ id: notaryId, name: GLOBAL_STORE.selectedNotaryObj?.name || t("notary.video_call.notary"), avatar: GLOBAL_STORE.selectedNotaryObj?.avatar || '' });
        }
        
        if (targetChat) {
          await ChatService.sendMessage(
            targetChat.id,
            "u1", // assume 'u1' is current user
            t("notary.video_call.call_message"),
            "call",
            { duration: formatDuration(duration), isVideo: true }
          );
        }
      }
    } catch (error) {
      console.error(error);
    }
    navigate(-1);
  };

  const ControlButton = ({
    icon: Icon,
    isActive,
    onClick,
    isDanger,
  }: {
    icon: React.ElementType;
    isActive?: boolean;
    onClick?: () => void;
    isDanger?: boolean;
  }) => (
    <div
      onClick={onClick}
      className={cn(
        "w-14 h-14 rounded-full flex items-center justify-center cursor-pointer transition-colors shadow-lg backdrop-blur-md",
        isDanger
          ? "bg-[#FA5151] text-white active:bg-red-600"
          : isActive
            ? "bg-white text-black active:bg-gray-200"
            : "bg-black/40 text-white border border-white/10 active:bg-black/60",
      )}
    >
      <Icon className="w-6 h-6" />
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-[#1A1A1A] relative overflow-hidden animate-in fade-in duration-300 z-[200] fixed inset-0">
      {/* Remote Video (Background) */}
      <div className="absolute inset-0 z-0">
        <img
          src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/notary-remote/800x1600.png"
          alt="Remote Video"
          className="w-full h-full object-cover"
        />
        {/* Notary Professional gradient overlay */}
        <div className="absolute inset-0 bg-gradient-to-b from-black/80 via-transparent to-black/90" />
      </div>

      {/* Professional Watermark / Logo */}
      <div className="absolute top-[calc(env(safe-area-inset-top)+20px)] right-4 opacity-50 z-10 pointer-events-none flex flex-col items-end">
        <ShieldCheck className="w-8 h-8 text-white mb-1" />
        <span className="text-white text-[10px] font-bold tracking-widest text-[#E5E5E5]">
          {t("notary.video_call.e_notary")}
        </span>
      </div>

      {/* Recording Indicator */}
      {callState === "connected" && (
        <div className="absolute top-[calc(env(safe-area-inset-top)+20px)] left-4 z-10 flex items-center bg-black/40 backdrop-blur-md border border-white/10 rounded-full px-3 py-1.5">
          <div className="w-2 h-2 rounded-full bg-[#FA5151] animate-pulse mr-2" />
          <span className="text-white text-[12px] font-medium tracking-wider">
            {t("notary.video_call.recording")}
          </span>
          <span className="ml-2 text-white/80 text-[12px] font-mono">
            {formatDuration(duration)}
          </span>
        </div>
      )}

      {/* Local Video (PIP) */}
      {callState === "connected" && !isVideoOff && (
        <motion.div
          drag
          dragConstraints={{ top: 80, left: 16, right: 16, bottom: 200 }}
          dragElastic={0.1}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          className="absolute top-24 right-4 w-[110px] h-[160px] bg-black rounded-xl overflow-hidden shadow-2xl border border-white/20 z-20 cursor-grab active:cursor-grabbing"
        >
          <img
            src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/notary-local/300x400.png"
            alt="Local Video"
            className={cn(
              "w-full h-full object-cover transition-transform duration-300",
              isFrontCamera ? "scale-x-[-1]" : "scale-x-100",
            )}
          />
          <div className="absolute bottom-1 left-2 text-[10px] text-white/90 drop-shadow-md">
            {t("notary.video_call.notary")}
          </div>
        </motion.div>
      )}

      {/* Header Info */}
      <div className="relative z-10 pt-[calc(env(safe-area-inset-top)+70px)] px-6 flex flex-col items-center">
        <h2 className="text-xl font-bold text-white drop-shadow-md tracking-wide">
          {partyName}
        </h2>
        <p className="text-[14px] text-white/80 drop-shadow-md mt-2 flex items-center">
          {callState === "calling" ? (
            <span className="flex items-center">
              <div className="w-1.5 h-1.5 rounded-full bg-primary-blue animate-pulse mr-2" />{" "}
              {t("notary.video_call.calling")}
            </span>
          ) : (
            <span className="flex items-center text-green-400">
              <ShieldCheck className="w-3.5 h-3.5 mr-1" /> {t("notary.video_call.verified")}
            </span>
          )}
        </p>
      </div>

      {/* Security Tip */}
      <div className="absolute bottom-[200px] left-0 right-0 z-10 px-8 flex justify-center pointer-events-none">
        <div className="flex items-start bg-black/60 backdrop-blur-md rounded-lg p-3 border border-white/10 max-w-[300px]">
          <AlertCircle className="w-4 h-4 text-primary-blue shrink-0 mt-0.5 mr-2" />
          <p className="text-[12px] text-white/80 leading-relaxed">
            {t("notary.video_call.security_tip")}
          </p>
        </div>
      </div>

      {/* Controls */}
      <div className="absolute bottom-0 left-0 right-0 z-10 pb-[calc(40px+env(safe-area-inset-bottom))] px-8">
        <div className="flex items-center justify-between max-w-[320px] mx-auto">
          <ControlButton
            icon={SwitchCamera}
            isActive={!isFrontCamera}
            onClick={() => setIsFrontCamera(!isFrontCamera)}
          />
          <ControlButton
            icon={isVideoOff ? VideoOff : Video}
            isActive={isVideoOff}
            onClick={() => setIsVideoOff(!isVideoOff)}
          />
          <ControlButton
            icon={isMuted ? MicOff : Mic}
            isActive={isMuted}
            onClick={() => setIsMuted(!isMuted)}
          />
          <ControlButton
            icon={PhoneOff}
            isDanger={true}
            onClick={handleHangUp}
          />
        </div>
      </div>
    </div>
  );
};
