import React from "react";
import { ChevronLeft, MoreVertical } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export interface VideoPlayerHeaderProps {
  onBack?: () => void;
}

export const VideoPlayerHeader: React.FC<VideoPlayerHeaderProps> = ({ onBack }) => {
  const navigate = useNavigate();

  return (
    <div className="relative flex items-center justify-between p-2 z-10">
      <IconButton
        icon={<ChevronLeft className="w-6 h-6 text-white" />}
        className="bg-transparent w-9 h-9 pointer-events-auto"
        onClick={(e) => {
          e.stopPropagation();
          if (onBack) {
            onBack();
          } else {
            navigate(-1);
          }
        }}
      />
      <IconButton
        icon={<MoreVertical className="w-5 h-5 text-white" />}
        className="bg-transparent w-9 h-9 pointer-events-auto"
        onClick={(e) => {
          e.stopPropagation();
        }}
      />
    </div>
  );
};
