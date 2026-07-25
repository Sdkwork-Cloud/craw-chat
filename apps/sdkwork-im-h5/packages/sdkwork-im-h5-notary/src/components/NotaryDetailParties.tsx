import { useTranslation } from "react-i18next";
import React from "react";
import { ShieldCheck, Video } from "lucide-react";

interface NotaryDetailPartiesProps {
  parties: any[];
  isFinalState: boolean;
  onEditParty: (p: any) => void;
  onNavigateToSignature: (p: any) => void;
  onNavigateToVideo: (p: any) => void;
}

export const NotaryDetailParties: React.FC<NotaryDetailPartiesProps> = ({
  parties,
  isFinalState,
  onEditParty,
  onNavigateToSignature,
  onNavigateToVideo,
}) => {
  const { t } = useTranslation();
return (
    <div className="flex flex-col bg-[#f4f6f9] dark:bg-black">
      {parties.map((p, i) => (
        <div
          key={i}
          className="bg-bg-color p-4 flex gap-4 border-b border-border-color/50 cursor-pointer active:bg-gray-100 dark:active:bg-gray-800 transition-colors last:border-0"
          onClick={() => onEditParty(p)}
        >
          <div className="w-[84px] h-[84px] shrink-0 bg-chat-other-bg rounded-lg overflow-hidden border border-border-color/50">
            <img
              src={p.avatar}
              alt="avatar"
              className="w-full h-full object-cover"
            />
          </div>
          <div className="flex flex-col justify-between flex-1 py-0.5 relative min-w-0">
            <div className="flex flex-col items-start gap-1.5 w-full">
              <div className="flex items-start justify-between w-full">
                <span className="text-[17px] font-bold text-text-main truncate pr-2">
                  {p.name}
                </span>
                <div className="px-1.5 py-0.5 border border-green-500/30 bg-green-500/10 text-green-600 dark:text-green-400 rounded text-[11px] font-medium whitespace-nowrap flex items-center gap-1 shrink-0">
                  <ShieldCheck className="w-3 h-3" />
                  {p.status}
                </div>
              </div>
              <span className="text-[13px] text-text-sub">{t('notary.auto_7a665979', '性别：{p.gender}')}</span>
            </div>

            <div className="flex items-center justify-end mt-auto pt-2">
              {!isFinalState && (
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    onNavigateToSignature(p);
                  }}
                  className="flex items-center justify-center h-8 px-4 rounded-lg bg-orange-500/10 text-orange-600 dark:text-orange-500 text-[13px] font-bold active:opacity-80 transition-opacity shadow-sm mr-2"
                >{t('notary.auto_f484f', '签名')}</button>
              )}
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onNavigateToVideo(p);
                }}
                className="flex items-center justify-center h-8 px-4 rounded-lg bg-primary-blue text-white text-[13px] font-bold active:opacity-80 transition-opacity shadow-sm"
              >
                <Video className="w-4 h-4 mr-1.5" />{t('notary.auto_2c919b96', '开始视频')}</button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
