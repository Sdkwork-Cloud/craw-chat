import React, { useRef, useState } from "react";
import { motion } from "motion/react";
import { Plus, X, File, PlayCircle } from "lucide-react";
import { useTranslation } from "react-i18next";
import { uuid } from "@sdkwork/utils";
import type { NotaryDraftAttachment } from "../services/notaryService";

const MAX_DRAFT_ATTACHMENTS = 20;
const MAX_ATTACHMENT_BYTES = 25 * 1024 * 1024;
const MAX_TOTAL_ATTACHMENT_BYTES = 100 * 1024 * 1024;

interface Step3ApplicationInfoProps {
  applicationInfo: string;
  setApplicationInfo: (info: string) => void;
  attachments: NotaryDraftAttachment[];
  setAttachments: (attachments: NotaryDraftAttachment[]) => void;
}

export const Step3ApplicationInfo: React.FC<Step3ApplicationInfoProps> = ({
  applicationInfo,
  setApplicationInfo,
  attachments,
  setAttachments,
}) => {
  const { t } = useTranslation();
const fileInputRef = useRef<HTMLInputElement>(null);
  const [fullscreenPreview, setFullscreenPreview] = useState<{ url: string, type: 'image' | 'video' } | null>(null);
  const [attachmentError, setAttachmentError] = useState<string | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      const remainingCapacity = Math.max(0, MAX_DRAFT_ATTACHMENTS - attachments.length);
      const newAttachments: NotaryDraftAttachment[] = [];
      let totalBytes = attachments.reduce(
        (total, attachment) => total + attachment.file.size,
        0,
      );
      let rejected = files.length > remainingCapacity;
      for (const file of files) {
        if (newAttachments.length >= remainingCapacity) {
          break;
        }
        if (
          file.size < 1
          || file.size > MAX_ATTACHMENT_BYTES
          || totalBytes + file.size > MAX_TOTAL_ATTACHMENT_BYTES
        ) {
          rejected = true;
          continue;
        }
        const isVideo = file.type.startsWith('video/');
        const isImage = file.type.startsWith('image/');
        const isPdf = file.type === "application/pdf";
        if (!isVideo && !isImage && !isPdf) {
          rejected = true;
          continue;
        }
        const attachmentBase = {
          id: uuid(),
          name: file.name,
          file,
          size: `${(file.size / 1024 / 1024).toFixed(2)} MB`,
        };
        if (isVideo || isImage) {
          newAttachments.push({
            ...attachmentBase,
            previewUrl: URL.createObjectURL(file),
            type: isVideo ? "video" : "image",
          });
        } else {
          newAttachments.push({
            ...attachmentBase,
            type: "file",
          });
        }
        totalBytes += file.size;
      }
      setAttachments([...attachments, ...newAttachments]);
      setAttachmentError(
        rejected
          ? t(
            "notary.create_steps.attachment_limits",
            "Some files were skipped. Maximum 20 files, 25 MB each and 100 MB total.",
          )
          : null,
      );
      e.target.value = "";
    }
  };

  const removeAttachment = (id: string) => {
    const attachment = attachments.find((item) => item.id === id);
    if (attachment?.previewUrl) {
      URL.revokeObjectURL(attachment.previewUrl);
    }
    setAttachments(attachments.filter((item) => item.id !== id));
  };

  return (
    <>
      <motion.div
        key="step3"
        initial={{ opacity: 0, x: 20 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: -20 }}
        className="flex flex-col gap-6"
      >
        <h2 className="text-[18px] font-bold">{t("notary.create_steps.fill_info")}</h2>
        
        <div className="flex flex-col gap-2">
          <span className="text-[14px] text-text-sub font-medium">
            {t("notary.create_steps.req_desc_label")} <span className="text-red-500">*</span>
          </span>
          <textarea
            value={applicationInfo}
            onChange={(e) => setApplicationInfo(e.target.value)}
            placeholder={t("notary.create_steps.req_desc_placeholder")}
            className="w-full bg-input-bg border border-border-color rounded-xl p-4 text-[15px] h-32 outline-none focus:border-primary-blue resize-none"
          />
        </div>

        <div className="flex flex-col gap-3 mt-2">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-[14px] text-text-sub font-medium">{t("notary.create_steps.supporting_docs")}</span>
              {attachments.length > 0 && <span className="text-[12px] text-text-sub bg-input-bg px-2 py-0.5 rounded-full">{attachments.length} {t("notary.create_steps.items")}</span>}
            </div>
            
            <button
               onClick={() => fileInputRef.current?.click()}
               className="flex items-center gap-1 text-[13px] text-primary-blue font-medium active:opacity-70 px-2 py-1 bg-primary-blue/10 rounded-lg"
            >
              <Plus className="w-4 h-4" />
              {t("notary.create_steps.add_attachment")}
            </button>
            <input 
              ref={fileInputRef}
              type="file" 
              multiple
              accept="image/*,video/*,application/pdf"
              className="hidden" 
              onChange={handleFileChange}
            />
          </div>
          {attachmentError && (
            <p className="text-[12px] text-red-500">{attachmentError}</p>
          )}
          
          {attachments.length > 0 ? (
            <div className="flex flex-col gap-2">
              {attachments.map((file) => (
                <div key={file.id} className="flex items-center gap-3 p-2 rounded-xl bg-input-bg border border-border-color cursor-pointer active:scale-[0.98]" onClick={() => file.type !== "file" && setFullscreenPreview({ url: file.previewUrl, type: file.type })}>
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
                    <span className="text-[12px] text-text-sub">{file.size}</span>
                  </div>
                  <button 
                    onClick={(e) => {
                       e.stopPropagation();
                       removeAttachment(file.id);
                    }}
                    className="w-8 h-8 rounded-full flex items-center justify-center active:bg-black/10 dark:active:bg-white/10 text-text-sub shrink-0 mx-1"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              ))}
            </div>
          ) : (
            <div className="py-10 flex flex-col items-center justify-center text-text-sub border border-dashed border-border-color rounded-xl bg-input-bg/50">
               <File className="w-8 h-8 opacity-40 mb-2" />
               <span className="text-[14px]">{t("notary.create_steps.no_attachments")}</span>
            </div>
          )}
        </div>
      </motion.div>

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
    </>
  );
};
