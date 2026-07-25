import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { UploadCloud } from "lucide-react";

interface Props {
  voiceName: string;
  setVoiceName: (v: string) => void;
  voiceDesc: string;
  setVoiceDesc: (v: string) => void;
  onSave: () => void;
}

export const CreateVoiceDetailsStep: React.FC<Props> = ({ voiceName, setVoiceName, voiceDesc, setVoiceDesc, onSave }) => {
  const { t } = useTranslation();
return (
  <motion.div
    key="done"
    initial={{ opacity: 0, scale: 0.9 }}
    animate={{ opacity: 1, scale: 1 }}
    exit={{ opacity: 0 }}
    className="flex flex-col gap-6 w-full h-full"
  >
    <div className="flex flex-col items-center text-center mt-2">
      <h2 className="text-[20px] font-bold text-text-main mb-1">{t('user.auto_4873cf62', '声音克隆完成')}</h2>
      <p className="text-[14px] text-text-sub">{t('user.auto_329904a7', '请完善您的专属声音信息')}</p>
    </div>

    <div className="flex flex-col gap-4 flex-1">
      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">{t('user.auto_41e0d641', '语音头像')}</label>
        <div className="w-20 h-20 bg-chat-other-bg border border-border-color rounded-2xl flex items-center justify-center overflow-hidden active:opacity-70 transition-opacity cursor-pointer mx-auto mb-2">
          <UploadCloud className="w-8 h-8 text-text-sub opacity-50" />
        </div>
      </div>

      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">{t('user.auto_2ab2cfc6', '声音名称')}</label>
        <input
          type="text"
          value={voiceName}
          onChange={(e) => setVoiceName(e.target.value)}
          placeholder={t('user.auto_prop_n304964a', '例如：治愈系睡前故事音')}
          className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors"
        />
      </div>

      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">{t('user.auto_2ab76b8e', '声音简介')}</label>
        <textarea
          value={voiceDesc}
          onChange={(e) => setVoiceDesc(e.target.value)}
          placeholder={t('user.auto_prop_n702e9370', '描述一下这个声音的特点或用途...')}
          rows={3}
          className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors resize-none mb-4"
        />
      </div>
    </div>

    <div className="mt-auto shrink-0 mb-4">
      <button
        onClick={onSave}
        disabled={!voiceName.trim()}
        className="w-full py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity disabled:opacity-50"
      >{t('user.auto_335d8adc', '保存我的专属声音')}</button>
    </div>
  </motion.div>
  );
};
