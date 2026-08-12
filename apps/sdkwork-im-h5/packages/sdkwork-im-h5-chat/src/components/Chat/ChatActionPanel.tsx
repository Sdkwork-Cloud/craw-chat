import React from "react";
import {
  ImageIcon,
  Video,
  Music,
  Folder,
  Phone,
  Link as LinkIcon,
  ShoppingBag,
} from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { showToast } from "@sdkwork/im-h5-commons";
import { ActionItem } from "./ActionItem";

interface ChatActionPanelProps {
  /** Opens the host file picker for the given media kind (image/video/file). */
  onFileSelected: (kind: "image" | "video" | "file") => void;
}

/**
 * Chat attachment action panel.
 *
 * Image/video/file entries open the host file picker through `onFileSelected`;
 * every other entry (music, link, miniapp, call) fails closed with a typed
 * toast because no composed owner SDK surface exists for those attachments.
 * The panel never fabricates media URLs or metadata.
 */
export const ChatActionPanel: React.FC<ChatActionPanelProps> = ({
  onFileSelected,
}) => {
  const { t } = useTranslation();

  const selectFile = (kind: "image" | "video" | "file") => {
    onFileSelected(kind);
  };

  const showUnavailable = (capability: string) => {
    showToast(t("chat.detail.attachments_unavailable", "{capability} is unavailable", { capability }));
  };

  return (
    <motion.div
      initial={{ height: 0, opacity: 0 }}
      animate={{ height: 256, opacity: 1 }}
      exit={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.2, ease: "easeOut" }}
      className="bg-input-bg border-t border-border-color overflow-hidden"
    >
      <div className="grid grid-cols-4 gap-y-6 p-6 h-64">
        <ActionItem
          icon={ImageIcon}
          label={t("chat.detail.photo")}
          onClick={() => selectFile("image")}
        />
        <ActionItem
          icon={Video}
          label={t("chat.detail.video")}
          onClick={() => selectFile("video")}
        />
        <ActionItem
          icon={Folder}
          label={t("chat.detail.file")}
          onClick={() => selectFile("file")}
        />
        <ActionItem
          icon={Music}
          label={t("chat.detail.music")}
          onClick={() => showUnavailable("Music")}
        />
        <ActionItem
          icon={LinkIcon}
          label={t("chat.detail.link")}
          onClick={() => showUnavailable("Link")}
        />
        <ActionItem
          icon={ShoppingBag}
          label={t("chat.detail.miniapp")}
          onClick={() => showUnavailable("Mini program")}
        />
        <ActionItem
          icon={Phone}
          label={t("chat.detail.call")}
          onClick={() => showUnavailable("Calls")}
        />
      </div>
    </motion.div>
  );
};
