import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import {
  Mic,
  MicOff,
  PhoneOff,
  Video,
  VideoOff,
  SwitchCamera,
} from "lucide-react";
import { motion } from "motion/react";
import { cn } from "@sdkwork/im-h5-commons";
import { ChatService } from "../services/ChatService";
import { useTranslation } from "react-i18next";
import { CallControlButton } from "../components/Chat/CallControlButton";

export const VideoCall: React.FC = () => {
  const { t } = useTranslation();
  const { id } = useParams();
  const navigate = useNavigate();
  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(false);
  const [isFrontCamera, setIsFrontCamera] = useState(true);
  const [callState, setCallState] = useState<"calling" | "connected">(
    "calling",
  );
  const [duration, setDuration] = useState(0);
  const [chat, setChat] = useState<any>(null);

  // Load real chat / user info
  useEffect(() => {
    if (id) {
      ChatService.getChatById(id).then((c) => {
        if (c) {
          setChat(c);
        }
      });
    }
  }, [id]);

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

  const handleHangUp = () => {
    navigate(-1);
  };

  const displayName = chat ? chat.name : t('chat.call.unknown');
  const displayAvatar = chat ? chat.avatar : "https://picsum.photos/seed/sarah-video/800/1600";

  return (
    <div className="flex flex-col h-full bg-black relative overflow-hidden">
      {/* Remote Video (Background) */}
      <div className="absolute inset-0 z-0">
        <img
          src={displayAvatar}
          alt="Remote Video"
          className="w-full h-full object-cover"
        />
        {/* Gradient overlay for text readability */}
        <div className="absolute inset-0 bg-gradient-to-b from-black/60 via-transparent to-black/80" />
      </div>

      {/* Local Video (PIP) */}
      {callState === "connected" && !isVideoOff && (
        <motion.div
          drag
          dragConstraints={{ top: 60, left: 20, right: 20, bottom: 200 }}
          dragElastic={0.1}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          className="absolute top-20 right-4 w-28 h-40 bg-gray-800 rounded-xl overflow-hidden shadow-2xl border border-white/20 z-20 cursor-grab active:cursor-grabbing"
        >
          <img
            src="https://picsum.photos/seed/me-video/300/400"
            alt="Local Video"
            className={cn(
              "w-full h-full object-cover transition-transform duration-300",
              isFrontCamera ? "scale-x-[-1]" : "scale-x-100",
            )}
          />
        </motion.div>
      )}

      {/* Header */}
      <div className="relative z-10 pt-safe px-6 flex flex-col items-center mt-4">
        <h2 className="text-xl font-bold text-white drop-shadow-md">
          {displayName}
        </h2>

        <p className="text-sm text-white/80 drop-shadow-md mt-1">
          {callState === "calling"
            ? t('chat.call.waiting')
            : formatDuration(duration)}
        </p>
      </div>

      {/* Controls */}
      <div className="absolute bottom-0 left-0 right-0 z-10 pb-[calc(40px+env(safe-area-inset-bottom))] px-8">
        <div className="flex items-center justify-between max-w-[320px] mx-auto">
          <CallControlButton
            icon={SwitchCamera}
            isActive={!isFrontCamera}
            onClick={() => setIsFrontCamera(!isFrontCamera)}
            size="md"
          />
          <CallControlButton
            icon={isVideoOff ? VideoOff : Video}
            isActive={isVideoOff}
            onClick={() => setIsVideoOff(!isVideoOff)}
            size="md"
          />
          <CallControlButton
            icon={isMuted ? MicOff : Mic}
            isActive={isMuted}
            onClick={() => setIsMuted(!isMuted)}
            size="md"
          />
          <CallControlButton
            icon={PhoneOff}
            isDanger={true}
            onClick={handleHangUp}
            size="md"
          />
        </div>
      </div>
    </div>
  );
};
