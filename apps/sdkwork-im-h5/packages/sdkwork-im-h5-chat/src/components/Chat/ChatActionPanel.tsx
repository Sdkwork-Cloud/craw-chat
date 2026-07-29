import { Paperclip } from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

import type { Message } from "@sdkwork/im-h5-types";

interface ChatActionPanelProps {
  handleSendCustom: (
    type: Message["type"],
    content: string,
    metadata?: Record<string, unknown>,
  ) => void;
}

export function ChatActionPanel({ handleSendCustom: _handleSendCustom }: ChatActionPanelProps) {
  const { t } = useTranslation("chat");

  return (
    <motion.div
      animate={{ height: 160, opacity: 1 }}
      className="overflow-hidden border-t border-border-color bg-input-bg"
      exit={{ height: 0, opacity: 0 }}
      initial={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.2, ease: "easeOut" }}
    >
      <div
        className="flex h-40 flex-col items-center justify-center gap-3 px-6 text-center text-text-sub"
        role="status"
      >
        <Paperclip aria-hidden="true" className="h-7 w-7" />
        <p className="m-0 text-sm">{t("detail.attachments_unavailable")}</p>
      </div>
    </motion.div>
  );
}
