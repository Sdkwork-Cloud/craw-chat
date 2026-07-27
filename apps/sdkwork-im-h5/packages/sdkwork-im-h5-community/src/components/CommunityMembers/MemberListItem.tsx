import React from "react";
import { useTranslation } from "react-i18next";
import { Shield, ShieldAlert, MoreVertical } from "lucide-react";
import { cn, IconButton } from "@sdkwork/im-h5-commons";
import { CommunityMember } from "../../types";

interface MemberListItemProps {
  member: CommunityMember;
  isLast: boolean;
  onSelect: (member: CommunityMember) => void;
}

export const MemberListItem: React.FC<MemberListItemProps> = ({
  member,
  isLast,
  onSelect,
}) => {
  const { t } = useTranslation();

  const getRoleIcon = (role: string) => {
    if (role === "owner")
      return <ShieldAlert className="w-3.5 h-3.5 text-orange-500" />;
    if (role === "admin")
      return <Shield className="w-3.5 h-3.5 text-blue-500" />;
    return null;
  };

  const getRoleLabel = (role: string) => {
    if (role === "owner") return "圈主";
    if (role === "admin") return "管理员";
    return "";
  };

  return (
    <div
      className={cn(
        "flex items-center pl-4 py-3 active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer",
        !isLast && "border-b border-black/5 dark:border-white/5"
      )}
      onClick={() => onSelect(member)}
    >
      <div className="w-12 h-12 rounded-full overflow-hidden bg-gray-100 dark:bg-[#2C2C2E] shrink-0 mr-3">
        <img
          src={member.avatar}
          alt={member.name}
          className="w-full h-full object-cover"
        />
      </div>
      <div className="flex flex-col flex-1 overflow-hidden pr-2">
        <div className="flex items-center gap-2">
          <span className="text-[16px] font-medium truncate">
            {member.name}
          </span>
          {getRoleIcon(member.role) && (
            <span
              className={cn(
                "flex items-center gap-1 text-[11px] font-medium px-1.5 py-0.5 rounded",
                member.role === "owner"
                  ? "bg-orange-500/10 text-orange-600 dark:text-orange-400"
                  : "bg-blue-500/10 text-blue-600 dark:text-blue-400"
              )}
            >
              {getRoleIcon(member.role)}
              {getRoleLabel(member.role)}
            </span>
          )}
          {(member.status === "banned" || member.status === "muted") && (
            <span className="text-[11px] font-medium px-1.5 py-0.5 rounded bg-red-500/10 text-red-600 dark:text-red-400">
              {t("community.auto_16fea11", "已禁言")}
            </span>
          )}
        </div>
        <div className="text-[13px] text-text-sub mt-0.5 truncate">
          {member.bio || "暂无简介"}
        </div>
      </div>
      <div className="pr-4 hidden md:block text-[13px] text-text-sub">
        加入于 {new Date(member.joinDate).toLocaleDateString()}
      </div>
      <div className="pr-4 flex items-center justify-center -mr-2">
        <IconButton
          icon={<MoreVertical className="w-5 h-5 text-text-sub" />}
          className="bg-transparent"
          onClick={(e) => {
            e.stopPropagation();
            onSelect(member);
          }}
        />
      </div>
    </div>
  );
};
