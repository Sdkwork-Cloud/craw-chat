import React from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { ApprovalItem } from "../services/ApprovalService";

interface ApprovalDetailInfoCardProps {
  approval: ApprovalItem;
}

export const ApprovalDetailInfoCard: React.FC<ApprovalDetailInfoCardProps> = ({ approval }) => {
  return (
    <div className="bg-chat-other-bg p-5 pb-6 border-b border-border-color/30">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 rounded-xl bg-primary-blue/10 flex items-center justify-center text-primary-blue text-[18px] font-medium">
            {approval.applicant.charAt(0)}
          </div>
          <div>
            <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
              {approval.applicant}
            </div>
            <div className="text-[13px] text-text-sub">
              {approval.department}
            </div>
          </div>
        </div>

        <span
          className={cn(
            "text-[14px] font-medium px-3 py-1 rounded-full",
            approval.status === "pending" && "bg-orange-50 text-orange-500",
            approval.status === "approved" && "bg-emerald-50 text-emerald-500",
            approval.status === "rejected" && "bg-rose-50 text-rose-500",
          )}
        >
          {approval.status === "pending"
            ? "待审批"
            : approval.status === "approved"
              ? "已同意"
              : "已拒绝"}
        </span>
      </div>

      <h2 className="text-[16px] font-medium text-text-main leading-relaxed border-t border-border-color/30 pt-4 mb-2">
        {approval.title}
      </h2>
      <div className="text-[15px] text-text-main/80 leading-relaxed whitespace-pre-wrap">
        {approval.content}
      </div>
    </div>
  );
};
