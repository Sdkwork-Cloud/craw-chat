import { useTranslation } from "react-i18next";
import React from "react";
import { cn } from "../utils/cn";

export interface SwitchProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  checkedColor?: string;
}

export const Switch: React.FC<SwitchProps> = ({
  checked,
  onChange,
  checkedColor = "bg-primary-blue",
}) => {
  const { t } = useTranslation();
  return (
  <div
    onClick={async (e) => {
      e.stopPropagation();
      onChange(!checked);
    }}
    className={cn(
      "w-12 h-6 rounded-full transition-colors relative cursor-pointer",
      checked ? checkedColor : "bg-gray-300 dark:bg-gray-600",
    )}
  >
    <div
      className={cn(
        "absolute top-1 w-4 h-4 rounded-full bg-white transition-transform shadow-sm",
        checked ? "left-7" : "left-1",
      )}
    />
  </div>
);
};

