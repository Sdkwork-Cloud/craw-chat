import React from "react";
import { X, Image as ImageIcon, Video } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";

interface CharacterAssetSelectorModalProps {
  isOpen: boolean;
  assets: {
    referenceImage: string | null;
    introVideo: string | null;
  };
  onClose: () => void;
  onUpdateAssets: (assets: { referenceImage: string | null; introVideo: string | null }) => void;
}

export const CharacterAssetSelectorModal: React.FC<CharacterAssetSelectorModalProps> = ({
  isOpen,
  assets,
  onClose,
  onUpdateAssets,
}) => {
  const { t } = useTranslation();

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="absolute inset-0 z-50 flex flex-col justify-end">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="absolute inset-0 bg-black/40 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div
            initial={{ y: "100%" }}
            animate={{ y: 0 }}
            exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 300 }}
            className="relative bg-bg-color rounded-t-3xl overflow-hidden pb-safe shadow-2xl flex flex-col h-[80vh]"
          >
            <div className="p-4 border-b border-border-color/50 flex items-center justify-between shrink-0">
              <div className="w-8" />
              <h3 className="font-bold text-[17px] text-text-main">
                {t("user.auto_40a164c3", "角色资产")}
              </h3>
              <IconButton
                icon={<X className="w-6 h-6 text-text-sub" />}
                onClick={onClose}
              />
            </div>

            <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-8">
              <div className="flex flex-col gap-3">
                <label className="text-[16px] font-medium text-text-main">
                  {t("user.auto_n284be0ba", "三视图 (可选)")}
                </label>
                <p className="text-[13px] text-text-sub leading-relaxed">
                  {t(
                    "user.auto_n38ee5d04",
                    "用于生成角色的 3D 模型或插画。建议上传包含正视图、侧视图、背视图的全身图片，能够显著提升生成模型的准确度。"
                  )}
                </p>
                <div
                  className="bg-chat-other-bg border border-border-color/50 rounded-2xl p-4 flex flex-col items-center justify-center aspect-video cursor-pointer active:opacity-70 transition-opacity relative overflow-hidden group"
                  onClick={() =>
                    onUpdateAssets({
                      ...assets,
                      referenceImage:
                        "https://picsum.photos/seed/ref/800/600",
                    })
                  }
                >
                  {assets.referenceImage ? (
                    <img
                      src={assets.referenceImage}
                      alt={t("user.auto_prop_135f5c1", "三视图")}
                      className="absolute inset-0 w-full h-full object-cover"
                    />
                  ) : (
                    <>
                      <div className="w-14 h-14 bg-primary-blue/10 rounded-full flex items-center justify-center mb-3 group-hover:scale-105 transition-transform">
                        <ImageIcon className="w-7 h-7 text-primary-blue" />
                      </div>
                      <span className="text-[15px] text-text-main font-medium">
                        {t("user.auto_4a0c0089", "点击上传三视图")}
                      </span>
                      <span className="text-[12px] text-text-sub mt-1">
                        {t("user.auto_4651028b", "支持 JPG / PNG")}
                      </span>
                    </>
                  )}
                </div>
              </div>

              <div className="flex flex-col gap-3">
                <label className="text-[16px] font-medium text-text-main">
                  {t("user.auto_n683f996d", "自我介绍视频 (可选)")}
                </label>
                <p className="text-[13px] text-text-sub leading-relaxed">
                  {t(
                    "user.auto_n4c7ece9a",
                    "用于克隆角色的动作及神态特征。建议视频时长在 10-30 秒之间，画面清晰，包含角色丰富的面部表情和肢体动作。"
                  )}
                </p>
                <div
                  className="bg-chat-other-bg border border-border-color/50 rounded-2xl p-4 flex flex-col items-center justify-center aspect-video cursor-pointer active:opacity-70 transition-opacity relative overflow-hidden group"
                  onClick={() =>
                    onUpdateAssets({
                      ...assets,
                      introVideo:
                        "https://storage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4",
                    })
                  }
                >
                  {assets.introVideo ? (
                    <video
                      src={assets.introVideo}
                      className="absolute inset-0 w-full h-full object-cover"
                      muted
                      autoPlay
                      loop
                      playsInline
                    />
                  ) : (
                    <>
                      <div className="w-14 h-14 bg-primary-blue/10 rounded-full flex items-center justify-center mb-3 group-hover:scale-105 transition-transform">
                        <Video className="w-7 h-7 text-primary-blue" />
                      </div>
                      <span className="text-[15px] text-text-main font-medium">
                        {t("user.auto_4074328c", "点击上传自我介绍视频")}
                      </span>
                      <span className="text-[12px] text-text-sub mt-1">
                        {t("user.auto_1607dea6", "支持 MP4 / MOV")}
                      </span>
                    </>
                  )}
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
