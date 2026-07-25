import React from "react";
import { useTranslation } from "react-i18next";
import { ChevronRight, QrCode } from "lucide-react";
import { Avatar } from "@sdkwork/im-h5-commons";
import { User } from "@sdkwork/im-h5-types";

interface ProfileHeaderCardProps {
  currentUser: User | null;
  onClick: () => void;
}

export const ProfileHeaderCard: React.FC<ProfileHeaderCardProps> = ({
  currentUser,
  onClick,
}) => {
  const { t } = useTranslation();

  return (
    <div
      onClick={onClick}
      className="bg-chat-other-bg px-4 py-8 mb-2 flex items-center justify-between active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
    >
      <div className="flex items-center gap-4 flex-1 min-w-0">
        <Avatar
          src={
            currentUser?.avatar || "https://picsum.photos/seed/me/200/200"
          }
          size="lg"
          className="w-[68px] h-[68px] rounded-[18px] shrink-0"
        />
        <div className="flex flex-col justify-center min-w-0 flex-1">
          <h2 className="text-[20px] font-bold text-text-main mb-1.5 truncate">
            {currentUser?.name || "User"}
          </h2>
          <p className="text-[14px] text-text-sub truncate">
            {t('user.auto_5f16e87c', '微信号: wxid_123456789')}
          </p>
        </div>
      </div>
      <div className="flex items-center gap-3 text-text-sub">
        <QrCode className="w-5 h-5" />
        <ChevronRight className="w-5 h-5 opacity-40" />
      </div>
    </div>
  );
};
