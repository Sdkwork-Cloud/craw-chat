import React from "react";
import { RotateCcw } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface VideoPlayerSeekFeedbackProps {
  showSeekFeedback: { type: "forward" | "backward"; show: boolean };
}

export const VideoPlayerSeekFeedback: React.FC<VideoPlayerSeekFeedbackProps> = ({
  showSeekFeedback,
}) => {
  const { t } = useTranslation();

  if (!showSeekFeedback.show) return null;

  return (
    <div
      className={`absolute top-0 bottom-0 w-1/3 flex flex-col items-center justify-center bg-white/10 pointer-events-none transition-all duration-300 animate-pulse ${
        showSeekFeedback.type === "forward"
          ? "right-0 rounded-l-full"
          : "left-0 rounded-r-full"
      }`}
    >
      <div className="flex gap-1 text-white">
        <RotateCcw
          className={`w-8 h-8 ${
            showSeekFeedback.type === "forward" ? "scale-x-[-1]" : ""
          }`}
        />
      </div>
      <span className="text-white text-[13px] font-bold mt-2">
        {t("course.auto_13793", "10秒")}
      </span>
    </div>
  );
};
