import React from "react";

interface ApprovalBottomActionsProps {
  submitting: boolean;
  onAction: (action: "approve" | "reject") => void;
}

export const ApprovalBottomActions: React.FC<ApprovalBottomActionsProps> = ({
  submitting,
  onAction,
}) => {
  return (
    <div className="absolute bottom-0 left-0 right-0 bg-white dark:bg-[#1a1b1c] border-t border-border-color/30 p-4 pb-safe flex gap-3 z-20">
      <button
        className="flex-1 bg-white border border-rose-500 text-rose-500 rounded-lg py-3 font-medium active:bg-rose-50 dark:bg-transparent dark:active:bg-rose-500/10 disabled:opacity-50"
        disabled={submitting}
        onClick={() => onAction("reject")}
      >
        拒绝
      </button>
      <button
        className="flex-1 bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90 disabled:opacity-50"
        disabled={submitting}
        onClick={() => onAction("approve")}
      >
        同意
      </button>
    </div>
  );
};
