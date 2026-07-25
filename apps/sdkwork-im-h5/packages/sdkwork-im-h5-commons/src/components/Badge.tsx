import { useTranslation } from "react-i18next";
import React from "react";
import { cn } from "../utils/cn";

interface BadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  count?: number;
  dot?: boolean;
}

export const Badge: React.FC<BadgeProps> = ({
  count,
  dot,
  className,
  children,
  ...props
}) => {
  const { t } = useTranslation();
if (!count && !dot) return <>{children}</>;

  return (
    <div className="relative inline-flex">
      {children}
      <div
        className={cn(
          "absolute flex items-center justify-center rounded-full bg-red-500 text-white font-medium",
          dot
            ? "w-2.5 h-2.5 -top-0.5 -right-0.5"
            : "h-5 min-w-[20px] px-1.5 text-[10px] -top-2 -right-2",
          className,
        )}
        {...props}
      >
        {!dot && count && count > 99 ? "99+" : count}
      </div>
    </div>
  );
};
