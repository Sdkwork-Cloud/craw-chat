import { Music2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export interface MusicTask {
  id: string;
  prompt: string;
  style: string;
  status: "processing" | "completed" | "failed";
  progress: number;
  audioUrl?: string;
  coverUrl?: string;
  title?: string;
}

export function AIMusicPage() {
  const { t } = useTranslation("ai_music");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Music2}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}
