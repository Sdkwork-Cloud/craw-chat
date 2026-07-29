import React, { useState } from "react";
import { motion } from "motion/react";
import { File, ChevronRight, X, PlayCircle } from "lucide-react";
import { useTranslation } from "react-i18next";
import type {
  NotaryDraftAttachment,
  NotaryStaffMember,
} from "../services/notaryService";
import {
  notaryDraftSession,
  type NotaryDraftPartyWithId,
} from "../state/notaryDraftSession";

interface Step4ConfirmationProps {
  notaryTypes: Array<{ id: string; name: string }>;
  selectedType: string;
  selectedNotaryObj: NotaryStaffMember | null;
  parties: NotaryDraftPartyWithId[];
  applicationInfo: string;
  attachments: NotaryDraftAttachment[];
  navigate: ReturnType<typeof import("react-router").useNavigate>;
}

export const Step4Confirmation: React.FC<Step4ConfirmationProps> = ({
  notaryTypes,
  selectedType,
  selectedNotaryObj,
  parties,
  applicationInfo,
  attachments,
  navigate,
}) => {
  const { t } = useTranslation();
const [fullscreenPreview, setFullscreenPreview] = useState<{ url: string, type: 'image' | 'video' } | null>(null);

  const handlePreviewParty = (party: NotaryDraftPartyWithId) => {
    notaryDraftSession.openPartyEditor({
      mode: "readonly",
      partyId: party.id,
    });
    navigate("/notary/add-party");
  };
  return (
    <motion.div
      key="step4"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-4"
    >
      <h2 className="text-[18px] font-bold">{t("notary.create_steps.confirm_info")}</h2>
      <div className="bg-chat-other-bg border border-border-color rounded-xl p-4 flex flex-col gap-4 text-[14px]">
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">{t("notary.create_steps.service_type")}</span>
          <span className="font-medium">
            {notaryTypes.find((t) => t.id === selectedType)?.name}
          </span>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">{t("notary.create_steps.handling_notary")}</span>
          <span className="font-medium">{selectedNotaryObj?.name}</span>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-2">
          <span className="text-text-sub">{t("notary.create_steps.parties_count", { count: parties.length })}</span>
          <div className="flex flex-col gap-2">
            {parties.map((p) => (
              <div 
                key={p.id}
                className="flex items-center justify-between bg-input-bg p-2 rounded-lg cursor-pointer active:scale-[0.98] transition-transform"
                onClick={() => handlePreviewParty(p)}
              >
                <div className="flex items-center gap-2">
                  <div className="w-8 h-8 rounded-full bg-primary-blue/10 flex items-center justify-center text-primary-blue font-bold text-[13px]">
                    {p.name.substring(0, 1)}
                  </div>
                  <span className="font-medium text-[15px]">{p.name}</span>
                </div>
                <ChevronRight className="w-4 h-4 text-text-sub" />
              </div>
            ))}
          </div>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">{t("notary.create_steps.app_description")}</span>
          <p className="font-medium whitespace-pre-wrap">{applicationInfo}</p>
        </div>
        {attachments.length > 0 && (
          <>
            <div className="h-[1px] w-full bg-border-color" />
            <div className="flex flex-col gap-2">
              <span className="text-text-sub">{t("notary.create_steps.supporting_docs_count", { count: attachments.length })}</span>
              <div className="flex flex-col gap-2">
                {attachments.map((file) => (
                  <div 
                    key={file.id}
                    className="flex items-center gap-3 p-2 rounded-xl bg-input-bg border border-border-color cursor-pointer active:scale-[0.98]"
                    onClick={() => file.type !== "file" && setFullscreenPreview({ url: file.previewUrl, type: file.type })}
                  >
                    <div className="w-12 h-12 rounded-lg overflow-hidden shrink-0 bg-black/5 dark:bg-white/5 relative">
                      {file.type === 'image' ? (
                        <img src={file.previewUrl} alt={file.name} className="w-full h-full object-cover" />
                      ) : file.type === "video" ? (
                        <div className="w-full h-full flex items-center justify-center">
                          <PlayCircle className="w-6 h-6 text-text-sub" />
                        </div>
                      ) : (
                        <div className="flex h-full w-full items-center justify-center">
                          <File className="h-6 w-6 text-text-sub" />
                        </div>
                      )}
                    </div>
                    <div className="flex-1 min-w-0 flex flex-col justify-center">
                      <span className="text-[14px] font-medium text-text-main truncate">{file.name}</span>
                      <span className="text-[12px] text-text-sub">{file.size || t("notary.create_steps.unknown_size")}</span>
                    </div>
                    <ChevronRight className="w-4 h-4 text-text-sub mx-2" />
                  </div>
                ))}
              </div>
            </div>
          </>
        )}
      </div>

      {/* Media Preview Overlay */}
      {fullscreenPreview && (
        <div
          className="fixed inset-0 z-[200] bg-black flex flex-col animate-in fade-in cursor-pointer"
          onClick={() => setFullscreenPreview(null)}
        >
          <div className="h-14 flex items-center justify-end px-4 pt-safe safe-area-top">
            <button
              className="text-white p-2"
              onClick={(e) => {
                e.stopPropagation();
                setFullscreenPreview(null);
              }}
            >
              <X className="w-8 h-8" />
            </button>
          </div>
          <div className="flex-1 flex items-center justify-center p-4">
            {fullscreenPreview.type === 'image' ? (
              <img
                src={fullscreenPreview.url}
                alt={t("notary.create_steps.preview")}
                className="max-w-full max-h-full object-contain"
              />
            ) : (
              <video 
                src={fullscreenPreview.url} 
                controls 
                autoPlay
                className="max-w-full max-h-full object-contain"
                onClick={(e) => e.stopPropagation()}
              />
            )}
          </div>
        </div>
      )}
    </motion.div>
  );
};
