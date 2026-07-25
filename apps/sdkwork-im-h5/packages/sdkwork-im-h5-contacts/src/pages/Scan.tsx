import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Image as ImageIcon } from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ContactService } from "../services/ContactService";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

export const Scan: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [isScanning, setIsScanning] = useState(true);

  useEffect(() => {
    const timer = setTimeout(async () => {
      setIsScanning(false);
      // Scan result: add a random friend and start chatting
      try {
        const friendName = `${t('contacts.scanned_user')}_${Math.floor(Math.random() * 1000)}`;
        const user = await ContactService.addFriend(friendName);
        const chat = await ContactService.createDirectChat(user);
        navigate(`/chat/${chat.id}`, { replace: true });
      } catch (e) {
        console.error(e);
        showToast(t('contacts.scan_failed'));
      }
    }, 2500); // 2.5 seconds scan

    return () => clearTimeout(timer);
  }, [navigate]);

  return (
    <div className="flex flex-col h-full bg-black relative overflow-hidden">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 shrink-0 pt-safe relative z-10">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-white" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-white">{t('contacts.scan_qr')}</h2>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-4">
          <span className="text-white text-[15px] cursor-pointer active:opacity-70 transition-opacity">
            {t('contacts.scan_album')}
          </span>
        </div>
      </header>

      {/* Scanner Area */}
      <div className="flex-1 relative flex flex-col items-center justify-center pb-20">
        <div className="relative w-[260px] h-[260px]">
          {/* Corners */}
          <div className="absolute top-0 left-0 w-6 h-6 border-t-[3px] border-l-[3px] border-[#1664FF]" />
          <div className="absolute top-0 right-0 w-6 h-6 border-t-[3px] border-r-[3px] border-[#1664FF]" />
          <div className="absolute bottom-0 left-0 w-6 h-6 border-b-[3px] border-l-[3px] border-[#1664FF]" />
          <div className="absolute bottom-0 right-0 w-6 h-6 border-b-[3px] border-r-[3px] border-[#1664FF]" />

          {/* Scanning Line */}
          {isScanning && (
            <motion.div
              className="absolute left-0 right-0 h-[2px] bg-[#1664FF] shadow-[0_0_12px_3px_rgba(22,100,255,0.6)]"
              animate={{ top: ["0%", "100%", "0%"] }}
              transition={{ duration: 3, repeat: Infinity, ease: "linear" }}
            />
          )}
        </div>
        <p className="text-white/70 text-[14px] mt-8">
          {t('contacts.scan_desc')}
        </p>
      </div>

      {/* Bottom Actions */}
      <div className="absolute bottom-[calc(40px+env(safe-area-inset-bottom))] left-0 right-0 flex justify-center">
        <div className="flex flex-col items-center gap-2 cursor-pointer active:opacity-70 transition-opacity">
          <div className="w-12 h-12 rounded-full bg-white/10 flex items-center justify-center backdrop-blur-md">
            <ImageIcon className="w-6 h-6 text-white" />
          </div>
          <span className="text-white/80 text-[12px]">{t('contacts.my_qr_code')}</span>
        </div>
      </div>
    </div>
  );
};
