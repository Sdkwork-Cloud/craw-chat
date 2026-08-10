import React from "react";
import { Users } from "lucide-react";
import { useTranslation } from "react-i18next";
import { showPrompt, showToast } from "@sdkwork/im-h5-commons";

export interface ApprovalApproversPickerProps {
  approverIds?: string[];
  onAddApprover: (name: string) => void;
  onRemoveApprover: (index: number) => void;
}

export const ApprovalApproversPicker: React.FC<ApprovalApproversPickerProps> = ({
  approverIds = [],
  onAddApprover,
  onRemoveApprover,
}) => {
  const { t } = useTranslation();

  const handleAddClick = async () => {
    const name = await showPrompt(t('approval.createForm.enterApprover'));
    if (name && name.trim()) {
      onAddApprover(name.trim());
      showToast(t('approval.createForm.addedApprover', { name: name.trim() }));
    }
  };

  return (
    <div className="bg-chat-other-bg mt-2 border-y border-border-color/30 p-4">
      <div className="text-[15px] text-text-main font-medium mb-3">
        {t('approval.createForm.approvers')}
      </div>
      <div className="flex gap-2 flex-wrap items-center">
        {approverIds.map((approver, i) => (
          <div key={i} className="relative group">
            <div className="w-12 h-12 rounded-full bg-primary-blue/10 text-primary-blue flex flex-col items-center justify-center text-[10px] whitespace-nowrap overflow-hidden text-ellipsis shadow-sm ring-1 ring-primary-blue/20">
              {approver.slice(0, 2)}
            </div>
            <div
              className="absolute -top-1 -right-1 bg-red-500 rounded-full w-4 h-4 flex items-center justify-center text-white cursor-pointer"
              onClick={() => onRemoveApprover(i)}
            >
              <span className="text-[10px] font-bold leading-none">&times;</span>
            </div>
          </div>
        ))}
        <div
          className="w-12 h-12 rounded-full bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color"
          onClick={handleAddClick}
        >
          <Users className="w-5 h-5 text-text-sub" />
        </div>
      </div>
    </div>
  );
};
