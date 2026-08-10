import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { UserRound } from "lucide-react";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

/**
 * Contact / group-invite card messages. The card payload rides in the
 * message metadata (`title`/`desc`/`icon`/`appIcon`); a `group-invite:` desc
 * prefix marks group invitations.
 */
export const CardMessage = ({
  msg,
  isMe,
}: {
  msg: Message;
  isMe: boolean;
}) => {
  const { t } = useTranslation();
  const description = String(msg.metadata?.desc ?? "");
  const isGroupInvite = description.startsWith("group-invite:");
  const title = msg.metadata?.title ?? msg.content;
  const icon = msg.metadata?.icon ?? msg.metadata?.appIcon;

  return (
    <div
      className="w-[240px] rounded-xl overflow-hidden border border-border-color cursor-pointer active:opacity-80 transition-opacity mt-1"
      onClick={() => {
        showToast(isGroupInvite
          ? t("chat.detail.card_group_invite", "Group invitation sent")
          : t("chat.detail.card_contact", "Contact card"));
      }}
    >
      <div className={cn("flex items-center gap-3 p-3", isMe ? "bg-white/10" : "bg-chat-other-bg")}>
        <div className={cn(
          "w-10 h-10 rounded-lg shrink-0 flex items-center justify-center",
          isMe ? "bg-white/20" : "bg-black/5 dark:bg-white/5",
        )}>
          {icon ? (
            <img src={icon} className="w-full h-full object-cover rounded-lg" />
          ) : (
            <UserRound className="w-5 h-5" />
          )}
        </div>
        <div className="flex flex-col min-w-0 flex-1">
          <span className={cn("text-[15px] font-medium truncate", isMe ? "text-white" : "text-text-main")}>
            {title}
          </span>
          {description && (
            <span className={cn("text-[12px] truncate mt-0.5", isMe ? "text-white/70" : "text-text-sub")}>
              {isGroupInvite
                ? t("chat.detail.card_group_invite_label", "Invites you to a group")
                : description}
            </span>
          )}
        </div>
      </div>
      <div className={cn(
        "px-3 py-1 text-[10px] uppercase tracking-widest",
        isMe ? "bg-black/20 text-white/60" : "bg-black/5 dark:bg-white/5 text-text-sub",
      )}>
        {isGroupInvite
          ? t("chat.detail.card_group_label", "Group Invitation")
          : t("chat.detail.card_contact_label", "Contact Card")}
      </div>
    </div>
  );
};
