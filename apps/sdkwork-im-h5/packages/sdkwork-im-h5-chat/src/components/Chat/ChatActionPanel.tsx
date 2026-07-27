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
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/newimg/800x450.png"
            )
          }
        />
        <ActionItem
          icon={Video}
          label={t("chat.detail.video")}
          onClick={() =>
            handleSendCustom(
              "video",
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/big-buck-bunny.mp4",
              {
                coverUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/vid/300x400.png",
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
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/audio/sfx-2869.mp3",
              {
                title: "Mixkit Tech House",
                artist: "Mixkit Author",
                coverUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/song/300x300.png",
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
              image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/link/100x100.png",
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
              icon: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/mini/50x50.png",
              image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/mini2/300x200.png",
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
