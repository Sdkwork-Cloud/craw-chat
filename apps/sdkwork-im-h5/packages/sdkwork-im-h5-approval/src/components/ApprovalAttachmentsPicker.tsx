import React from "react";
import { FileImage } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface ApprovalAttachmentsPickerProps {
  attachments?: string[];
  onAddAttachments: (urls: string[]) => void;
  onRemoveAttachment: (index: number) => void;
}

export const ApprovalAttachmentsPicker: React.FC<ApprovalAttachmentsPickerProps> = ({
  attachments = [],
  onAddAttachments,
  onRemoveAttachment,
}) => {
  const { t } = useTranslation();

  return (
    <div className="bg-chat-other-bg mt-2 border-y border-border-color/30 p-4">
      <div className="text-[15px] text-text-main font-medium mb-3">
        {t('approval.createForm.attachments')}
      </div>
      <div className="flex gap-2 flex-wrap">
        {attachments.map((url, i) => (
          <div key={i} className="w-16 h-16 rounded-xl relative group">
            <img
              src={url}
              className="w-full h-full object-cover rounded-xl border border-border-color/20"
              alt={`attachment-${i}`}
            />
            <div
              className="absolute -top-2 -right-2 bg-red-500 rounded-full w-5 h-5 flex items-center justify-center text-white cursor-pointer"
              onClick={() => onRemoveAttachment(i)}
            >
              <span className="text-xs font-bold leading-none">&times;</span>
            </div>
          </div>
        ))}
        <label className="w-16 h-16 rounded-xl bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color relative">
          <FileImage className="w-6 h-6 text-text-sub" />
          <input
            type="file"
            className="hidden"
            accept="image/*"
            multiple
            onChange={(e) => {
              const files = Array.from(e.target.files || []) as File[];
              const urls = files.map((f) => window.URL.createObjectURL(f));
              onAddAttachments(urls);
            }}
          />
        </label>
      </div>
    </div>
  );
};
