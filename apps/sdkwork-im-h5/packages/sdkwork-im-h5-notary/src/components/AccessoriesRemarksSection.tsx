import React from "react";
import { ChevronRight, FileText, Plus, X } from "lucide-react";
import { useTranslation } from "react-i18next";

interface AccessoriesRemarksSectionProps {
  isReadonly: boolean;
  formData: any;
  attachments: { name: string; url: string }[];
  setFullPageEditor: React.Dispatch<React.SetStateAction<any>>;
  setAttachments: React.Dispatch<
    React.SetStateAction<{ name: string; url: string }[]>
  >;
  attachmentRef: React.RefObject<HTMLInputElement>;
  handleAttachmentsChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const AccessoriesRemarksSection: React.FC<
  AccessoriesRemarksSectionProps
> = ({
  isReadonly,
  formData,
  attachments,
  setFullPageEditor,
  setAttachments,
  attachmentRef,
  handleAttachmentsChange,
}) => {
  const { t } = useTranslation();
return (
    <div className="bg-bg-color px-4 py-2 flex flex-col mb-4">
      <div
        className="flex items-start min-h-[54px] py-3 border-b border-border-color cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "remarks",
            title: t("notary.extra_info.remarks"),
            placeholder: t("notary.extra_info.remarks_placeholder"),
            value: formData.remarks,
            isTextArea: true,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0 pt-0.5">
          {t("notary.extra_info.remarks")}
        </label>
        <div className="flex-1 flex items-start justify-end text-[15px]">
          {formData.remarks ? (
            <span className="text-text-main line-clamp-2 text-right break-all max-w-[180px] pt-0.5">
              {formData.remarks}
            </span>
          ) : (
            <span className="text-text-sub pt-0.5">{t("notary.extra_info.no_remarks")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 flex-shrink-0 mt-0.5" />
        </div>
      </div>

      <div className="flex flex-col gap-3 py-4">
        <div className="flex justify-between items-center">
          <label className="text-[15px] text-text-main">{t("notary.extra_info.attachments")}</label>
          <span className="text-[13px] text-text-sub">{t("notary.extra_info.attachments_hint")}</span>
        </div>
        <input
          type="file"
          multiple
          accept="image/*,application/pdf"
          className="hidden"
          ref={attachmentRef}
          onChange={handleAttachmentsChange}
        />

        {attachments.length > 0 && (
          <div className="flex flex-col gap-2 mb-2">
            {attachments.map((att, i) => (
              <div
                key={i}
                className="px-3 py-2 bg-active-bg rounded-xl text-[14px] flex items-center gap-3 text-text-main overflow-hidden border border-transparent dark:border-border-color"
              >
                {att.url.startsWith("blob:") && att.url.length > 0 ? (
                  <img
                    src={att.url}
                    alt={t("notary.extra_info.preview")}
                    className="w-9 h-9 rounded object-contain border border-border-color shrink-0 bg-black/5 dark:bg-white/5"
                  />
                ) : (
                  <FileText className="w-5 h-5 text-primary-blue shrink-0" />
                )}
                <span className="truncate flex-1 text-[13px]">{att.name}</span>
                {!isReadonly && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setAttachments((prev) =>
                        prev.filter((_, idx) => idx !== i),
                      );
                    }}
                    className="w-7 h-7 rounded-full flex items-center justify-center bg-transparent active:bg-black/10 dark:active:bg-white/10 text-text-sub shrink-0"
                  >
                    <X className="w-4 h-4" />
                  </button>
                )}
              </div>
            ))}
          </div>
        )}
        {!isReadonly && (
          <div
            onClick={() => attachmentRef.current?.click()}
            className="w-full py-6 bg-input-bg border border-dashed border-border-color rounded-xl flex flex-col items-center justify-center cursor-pointer active:bg-active-bg transition-colors group"
          >
            <Plus className="w-6 h-6 text-text-sub mb-1 opacity-70 group-active:scale-110 transition-transform" />
            <span className="text-[13px] text-text-sub">{t("notary.extra_info.click_add")}</span>
          </div>
        )}
      </div>
    </div>
  );
};
