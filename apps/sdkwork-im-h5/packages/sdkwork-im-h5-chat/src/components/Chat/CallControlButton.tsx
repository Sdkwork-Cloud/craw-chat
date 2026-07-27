import React from "react";
import { cn } from "@sdkwork/im-h5-commons";

interface CallControlButtonProps {
  icon: React.ElementType;
  label?: string;
  isActive?: boolean;
  isDanger?: boolean;
  onClick?: () => void;
  size?: "md" | "lg";
}

export const CallControlButton: React.FC<CallControlButtonProps> = ({
  icon: Icon,
  label,
  isActive = false,
  isDanger = false,
  onClick,
  size = "lg",
}) => {
  const buttonSizeClass = size === "lg" ? "w-16 h-16" : "w-14 h-14";
  const iconSizeClass = size === "lg" ? "w-7 h-7" : "w-6 h-6";

  return (
    <div className="flex flex-col items-center gap-2">
      <div
        onClick={onClick}
        className={cn(
          "rounded-full flex items-center justify-center cursor-pointer transition-colors shadow-lg backdrop-blur-md",
          buttonSizeClass,
          isDanger
            ? "bg-red-500 text-white active:bg-red-600"
            : isActive
              ? "bg-white text-black active:bg-gray-200"
              : "bg-white/20 text-white border border-white/10 active:bg-white/30",
        )}
      >
        <Icon className={iconSizeClass} />
      </div>
      {label && <span className="text-[13px] text-white/80">{label}</span>}
    </div>
  );
};
