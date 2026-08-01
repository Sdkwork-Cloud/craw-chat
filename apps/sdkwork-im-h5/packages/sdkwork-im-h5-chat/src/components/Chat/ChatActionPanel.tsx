import { FileText, Image as ImageIcon, Video, FolderOpen } from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

import type { Message } from "@sdkwork/im-h5-types";

interface ChatActionPanelProps {
  handleSendCustom: (
    type: Message["type"],
    content: string,
    metadata?: Record<string, unknown>,
  ) => void;
  onFileSelected?: (file: File, kind: "image" | "video" | "file") => void;
}

export function ChatActionPanel({ handleSendCustom: _handleSendCustom, onFileSelected }: ChatActionPanelProps) {
  const { t } = useTranslation("chat");
  const selectFile = (accept: string, kind: "image" | "video" | "file") => {
    const input = document.createElement("input");
    input.type = "file"; input.accept = accept;
    input.onchange = () => { const file = input.files?.[0]; if (file) onFileSelected?.(file, kind); };
    input.click();
  };

  return (
    <motion.div
      animate={{ height: 160, opacity: 1 }}
      className="overflow-hidden border-t border-border-color bg-input-bg"
      exit={{ height: 0, opacity: 0 }}
      initial={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.2, ease: "easeOut" }}
    >
      <div className="grid h-40 grid-cols-4 items-center justify-items-center gap-3 px-6 text-center text-text-sub">
        <button type="button" className="flex flex-col items-center gap-2 text-[12px]" onClick={() => selectFile("image/*", "image")}><ImageIcon className="h-7 w-7" /><span>{t("detail.image", "Image")}</span></button>
        <button type="button" className="flex flex-col items-center gap-2 text-[12px]" onClick={() => selectFile("video/*", "video")}><Video className="h-7 w-7" /><span>{t("detail.video", "Video")}</span></button>
        <button type="button" className="flex flex-col items-center gap-2 text-[12px]" onClick={() => selectFile("*/*", "file")}><FileText className="h-7 w-7" /><span>{t("detail.file", "File")}</span></button>
        <button type="button" className="flex flex-col items-center gap-2 text-[12px]" onClick={() => selectFile("*/*", "file")}><FolderOpen className="h-7 w-7" /><span>{t("detail.document", "Document")}</span></button>
      </div>
    </motion.div>
  );
}
