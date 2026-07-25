import React from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { Clock, Check, X } from "lucide-react";
import { ApprovalItem } from "../services/ApprovalService";

interface ApprovalProcessTimelineProps {
  approval: ApprovalItem;
}

export const ApprovalProcessTimeline: React.FC<ApprovalProcessTimelineProps> = ({ approval }) => {
  return (
    <div className="mt-4 px-4">
      <h3 className="text-[14px] font-medium text-text-sub mb-4">审批流程</h3>
      <div className="flex flex-col gap-5 pl-2 relative border-l-2 border-gray-200 dark:border-gray-800 ml-4 pb-4">
        {/* Applicant step */}
        <div className="relative">
          <div className="absolute -left-[19px] w-8 h-8 rounded-full bg-blue-500 flex items-center justify-center text-white text-[12px] shadow-sm">
            发
          </div>
          <div className="pl-6">
            <div className="flex justify-between items-start mb-1">
              <span className="text-[15px] font-medium text-text-main">
                {approval.applicant} (发起申请)
              </span>
              <span className="text-[12px] text-text-sub">
                {approval.date}
              </span>
            </div>
          </div>
        </div>

        {/* History steps */}
        {approval.history.map((record, i) => (
          <div key={i} className="relative mt-5">
            <div className={cn(
              "absolute -left-[19px] w-8 h-8 rounded-full flex items-center justify-center text-white text-[12px] shadow-sm",
              record.action === "reject" ? "bg-rose-500" : "bg-emerald-500"
            )}>
              {record.action === "reject" ? <X className="w-4 h-4" /> : <Check className="w-4 h-4" />}
            </div>
            <div className="pl-6">
              <div className="flex justify-between items-start mb-1">
                <span className="text-[15px] font-medium text-text-main">
                  {record.name} {record.action === "reject" ? "(已拒绝)" : "(已同意)"}
                </span>
                <span className="text-[12px] text-text-sub">
                  {record.actionTime}
                </span>
              </div>
              {record.comment && (
                <div className="text-[14px] text-text-sub mt-1 bg-gray-50 dark:bg-gray-800 p-2 rounded">
                  {record.comment}
                </div>
              )}
            </div>
          </div>
        ))}

        {/* Pending step */}
        {approval.status === "pending" && (
          <div className="relative mt-5">
            <div className="absolute -left-[19px] w-8 h-8 rounded-full bg-orange-400 flex items-center justify-center text-white text-[12px] shadow-sm">
              <Clock className="w-4 h-4" />
            </div>
            <div className="pl-6">
              <div className="flex justify-between items-start mb-1">
                <span className="text-[15px] font-medium text-orange-500">当前审批轮到你</span>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
