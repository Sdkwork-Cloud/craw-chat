import { useTranslation } from "react-i18next";
import React from "react";
import { cn } from "../utils/cn";

interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  icon: React.ReactNode;
}

export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  className,
  ...props
}) => {
  const { t } = useTranslation();
return (
    <button
      className={cn(
        "inline-flex items-center justify-center rounded-full p-2 active:bg-active-bg transition-colors focus:outline-none",
        className,
      )}
      {...props}
    >
      {icon}
    </button>
  );
};
