import React from "react";
import { Lock } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Community } from "../../types";

export const CommunityLockedView = ({
  community,
  onJoin,
}: {
  community: Community;
  onJoin: () => void;
}) => {
  const { t } = useTranslation();

  return (
    <div className="flex-1 flex flex-col items-center justify-center bg-white dark:bg-[#1C1C1E] p-8 -mt-2 z-10">
      <div className="w-16 h-16 bg-blue-500/10 rounded-full flex items-center justify-center mb-6">
        <Lock className="w-8 h-8 text-blue-500" />
      </div>
      <h3 className="text-[18px] font-semibold text-text-main mb-2">
        {t("community.auto_25f42a69", "付费圈子")}
      </h3>
      <p className="text-[14px] text-text-sub text-center mb-8">
        {t("community.auto_n3a0b56d", "解锁专享内容、群组资源以及与优质圈友互动。")}
      </p>

      <button
        className="w-full max-w-[240px] py-3.5 bg-blue-500 text-white rounded-full font-bold text-[16px] shadow-lg shadow-blue-500/30 active:scale-95 transition-transform flex items-center justify-center gap-2"
        onClick={onJoin}
      >
        {t("community.auto_20ed0b2", "¥{{price}} 购买解锁", { price: community.price })}
      </button>
    </div>
  );
};
