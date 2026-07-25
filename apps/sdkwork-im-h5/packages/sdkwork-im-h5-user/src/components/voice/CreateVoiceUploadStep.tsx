import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { UploadCloud } from "lucide-react";

interface Props {
  handleUpload: () => void;
}

export const CreateVoiceUploadStep: React.FC<Props> = ({ handleUpload }) => {
  const { t } = useTranslation();
return (
  <motion.div
    key="upload-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col items-center justify-center gap-8 h-full min-h-0"
  >
    <div className="w-32 h-32 bg-primary-blue/5 rounded-full flex items-center justify-center border-2 border-dashed border-primary-blue/30">
      <UploadCloud className="w-12 h-12 text-primary-blue" />
    </div>
    <div className="text-center px-4">
      <h3 className="text-[18px] font-bold text-text-main mb-2">{t('user.auto_n49aad528', '上传本地音频')}</h3>
      <p className="text-[14px] text-text-sub leading-relaxed">{t('user.auto_2ce6b97d', '请上传包含清晰人声的音频文件')}<br />{t('user.auto_9a6dcc1', '建议时长 1 分钟到 3 分钟')}<br />{t('user.auto_n7249925d', '支持 MP3, WAV, M4A 格式')}</p>
    </div>
    <button
      onClick={handleUpload}
      className="px-10 py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity whitespace-nowrap"
    >{t('user.auto_4c394363', '选择文件并开始生成')}</button>
  </motion.div>
  );
};
