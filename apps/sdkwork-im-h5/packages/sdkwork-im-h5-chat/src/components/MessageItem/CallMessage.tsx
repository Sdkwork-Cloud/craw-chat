import { useTranslation } from "react-i18next";
import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { Phone, Video } from "lucide-react";

export const CallMessage = ({ msg }: { msg: Message }) => {
  const { t } = useTranslation();
  return (
  <div className="flex items-center gap-2">
    {msg.metadata?.isVideo ? (
      <Video className="w-5 h-5" />
    ) : (
      <Phone className="w-5 h-5" />
    )}
    <span>
      {msg.content} {msg.metadata?.duration && `· ${msg.metadata.duration}`}
    </span>
  </div>
);
};

