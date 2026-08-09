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
import { ActionItem } from "./ActionItem";

interface ChatActionPanelProps {
  handleSendCustom: (type: any, url: string, metadata?: any) => void;
}

export const ChatActionPanel: React.FC<ChatActionPanelProps> = ({
  handleSendCustom,
}) => {
  const { t } = useTranslation();
  
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
          onClick={() =>
            handleSendCustom(
              "image",
              "https://picsum.photos/seed/newimg/800/450"
            )
          }
        />
        <ActionItem
          icon={Video}
          label={t("chat.detail.video")}
          onClick={() =>
            handleSendCustom(
              "video",
              "https://storage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
              {
                coverUrl: "https://picsum.photos/seed/vid/300/400",
                duration: "0:10",
              }
            )
          }
        />
        <ActionItem
          icon={Music}
          label={t("chat.detail.music")}
          onClick={() =>
            handleSendCustom(
              "music",
              "https://assets.mixkit.co/active_storage/sfx/2869/2869-preview.mp3",
              {
                title: "Mixkit Tech House",
                artist: "Mixkit Author",
                coverUrl: "https://picsum.photos/seed/song/300/300",
              }
            )
          }
        />
        <ActionItem
          icon={Folder}
          label={t("chat.detail.file")}
          onClick={() =>
            handleSendCustom("file", t("chat.date.miniapp_title"), {
              size: "1.2 MB",
              ext: "xlsx",
            })
          }
        />
        <ActionItem
          icon={LinkIcon}
          label={t("chat.detail.link")}
          onClick={() =>
            handleSendCustom("link", "https://example.com/article", {
              title: t("chat.detail.miniapp"),
              desc: "...",
              image: "https://picsum.photos/seed/link/100/100",
            })
          }
        />
        <ActionItem
          icon={ShoppingBag}
          label={t("chat.detail.miniapp")}
          onClick={() =>
            handleSendCustom("miniapp", t("chat.detail.miniapp"), {
              title: t("chat.detail.miniapp"),
              desc: "...",
              icon: "https://picsum.photos/seed/mini/50/50",
              image: "https://picsum.photos/seed/mini2/300/200",
            })
          }
        />
        <ActionItem
          icon={Phone}
          label={t("chat.detail.call")}
          onClick={() =>
            handleSendCustom("call", t("chat.detail.call"), {
              duration: "1:23",
            })
          }
        />
      </div>
    </motion.div>
  );
};
