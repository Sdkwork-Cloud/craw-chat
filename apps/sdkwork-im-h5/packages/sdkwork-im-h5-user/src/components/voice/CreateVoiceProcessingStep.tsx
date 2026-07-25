import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { Sparkles } from "lucide-react";

export const CreateVoiceProcessingStep: React.FC = () => {
  const { t } = useTranslation();
return (
  <motion.div
    key="processing"
    initial={{ opacity: 0, scale: 0.9 }}
    animate={{ opacity: 1, scale: 1 }}
    exit={{ opacity: 0 }}
    className="flex flex-col items-center justify-center h-full gap-6 w-full"
  >
    <div className="relative w-32 h-32 flex items-center justify-center">
      <div className="absolute inset-0 border-[4px] border-primary-blue/20 rounded-full" />
      <motion.div
        className="absolute inset-0 border-[4px] border-primary-blue rounded-full border-t-transparent"
        animate={{ rotate: 360 }}
        transition={{ repeat: Infinity, duration: 1, ease: "linear" }}
      />
      <Sparkles className="w-10 h-10 text-primary-blue" />
    </div>
    <div className="text-center">
      <h2 className="text-[20px] font-bold text-text-main mb-2">{t('user.auto_n519a8a9', 'AI 正在克隆您的声音')}</h2>
      <p className="text-[14px] text-text-sub">{t('user.auto_532ea150', '分析音色特征并建立数字模型，请稍候...')}</p>
    </div>
  </motion.div>
  );
};
