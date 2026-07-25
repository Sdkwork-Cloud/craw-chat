import { useTranslation } from "react-i18next";
import React from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { Check } from "lucide-react";

interface SuccessModalProps {
  isPaid: boolean;
  communityName: string;
  hasGroups: boolean;
  onClose: () => void;
  onEnterGroups: () => void;
  onEnterResources: () => void;
}

export const SuccessModal: React.FC<SuccessModalProps> = ({
  isPaid,
  communityName,
  hasGroups,
  onClose,
  onEnterGroups,
  onEnterResources
}) => {
  const { t } = useTranslation();
return (
    <div className="absolute inset-0 z-[100] flex items-center justify-center pointer-events-auto p-4 overflow-hidden">
      <div 
        className="absolute inset-0 bg-black/50 backdrop-blur-sm transition-opacity"
        onClick={onClose}
      />
      <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl w-full max-w-sm mx-auto relative z-10 p-6 animate-in zoom-in-95 duration-200 shadow-2xl flex flex-col items-center text-center">
        <div className="w-16 h-16 bg-emerald-500/10 rounded-full flex items-center justify-center mb-4">
          <Check className="w-8 h-8 text-emerald-500" />
        </div>
        <h3 className="text-[20px] font-bold text-text-main mb-2">{isPaid ? "支付成功" : "加入成功"}</h3>
        <p className="text-[15px] text-text-sub mb-6 leading-relaxed">{t('community.auto_n3df725c7', '欢迎加入「{communityName}」。我们为您准备了圈子专属文档指南，建议您优先阅读。同时请别忘了加入圈子群组，开启热聊！')}</p>

        <div className="w-full flex flex-col gap-3">
          {hasGroups && (
            <button 
              onClick={onEnterGroups}
              className="w-full bg-blue-500 text-white py-3.5 rounded-xl font-medium text-[16px] shadow-lg shadow-blue-500/30 active:scale-95 transition-transform"
            >{t('community.auto_316501e9', '进入聊天群组')}</button>
          )}
          <button 
            onClick={onEnterResources}
            className={cn("w-full py-3.5 rounded-xl font-medium text-[16px] transition-colors", hasGroups ? "bg-black/5 dark:bg-white/10 text-text-main" : "bg-blue-500 text-white shadow-lg shadow-blue-500/30")}
          >{t('community.auto_691fca', '查看圈子文档')}</button>
        </div>
      </div>
    </div>
  );
};
