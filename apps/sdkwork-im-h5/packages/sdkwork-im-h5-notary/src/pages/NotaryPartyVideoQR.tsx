import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, QrCode, Video } from "lucide-react";
import { motion } from "motion/react";
import QRCode from "react-qr-code";

export const NotaryPartyVideoQR: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams();

  const handleBack = () => {
  navigate(-1);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="h-[44px] pt-safe flex items-center justify-between px-2 shrink-0 border-b border-border-color">
        <div className="w-[44px] flex items-center">
          <ChevronLeft className="w-7 h-7 text-text-main cursor-pointer" onClick={handleBack} />
        </div>
        <span className="font-medium text-[17px] text-text-main">{t('notary.auto_n58b99775', '视频通话二维码')}</span>
        <div className="w-[44px]" />
      </header>

      <div className="flex-1 overflow-y-auto no-scrollbar flex flex-col items-center justify-center p-6 bg-input-bg pb-safe">
        <motion.div
           initial={{ opacity: 0, scale: 0.95 }}
           animate={{ opacity: 1, scale: 1 }}
           className="w-full max-w-[320px] bg-white dark:bg-[#1a1a1a] rounded-2xl p-8 shadow-sm border border-border-color flex flex-col items-center"
        >
          <div className="w-16 h-16 rounded-full bg-primary-blue/10 flex items-center justify-center text-primary-blue mb-4">
            <Video className="w-8 h-8" />
          </div>
          <h2 className="text-[20px] font-bold text-text-main mb-2">{t('notary.auto_342a56f0', '当事人视频通话')}</h2>
          <p className="text-[14px] text-text-sub text-center mb-8">{t('notary.auto_n1d996706', '请当事人使用微信扫一扫上方二维码，即可进入视频通话房间进行面签核身')}</p>
          
          <div className="p-4 bg-white rounded-xl shadow-sm border border-border-color/50 mb-6">
            <QRCode 
              value={`https://im.sdkwork.com/call/video-notary/${id}`}
              size={200}
              level="H"
            />
          </div>

          <div className="flex items-center gap-2 text-primary-blue text-[13px] font-medium bg-primary-blue/5 px-4 py-2 rounded-full cursor-pointer active:scale-95 transition-transform">
             <QrCode className="w-4 h-4" />{t('notary.auto_n6fcbe137', '保存到相册')}</div>
        </motion.div>
      </div>
    </div>
  );
};
