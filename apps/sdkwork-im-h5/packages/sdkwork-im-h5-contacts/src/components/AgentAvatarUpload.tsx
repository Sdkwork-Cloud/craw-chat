import React, { useRef } from "react";
import { Bot, Camera } from "lucide-react";
import { useTranslation } from "react-i18next";

interface AgentAvatarUploadProps {
  avatarPreview: string | null;
  onAvatarSelect: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const AgentAvatarUpload: React.FC<AgentAvatarUploadProps> = ({
  avatarPreview,
  onAvatarSelect,
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  return (
    <div className="flex flex-col items-center gap-3">
      <input
        type="file"
        accept="image/*"
        className="hidden"
        ref={fileInputRef}
        onChange={onAvatarSelect}
      />
      <div
        onClick={() => fileInputRef.current?.click()}
        className="w-20 h-20 rounded-2xl bg-chat-other-bg border border-border-color flex items-center justify-center relative overflow-hidden cursor-pointer active:scale-95 transition-transform group"
      >
        {avatarPreview ? (
          <img
            src={avatarPreview}
            alt="Avatar Preview"
            className="w-full h-full object-cover"
          />
        ) : (
          <Bot className="w-10 h-10 text-text-sub opacity-50" />
        )}
        <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
          <Camera className="w-6 h-6 text-white drop-shadow-md" />
        </div>
      </div>
      <span className="text-[13px] text-text-sub">
        {t("contacts.set_avatar")}
      </span>
    </div>
  );
};
