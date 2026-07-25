import { useTranslation } from "react-i18next";
import React from "react";
import { useNavigate } from "react-router";
import { ChevronLeft } from "lucide-react";
import { IconButton } from "../index";

export const PageLayout = ({
  title,
  children,
  rightElement = null,
  bgClass = "bg-bg-color",
}: {
  title?: string;
  children: React.ReactNode;
  rightElement?: React.ReactNode;
  bgClass?: string;
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <div className={`flex flex-col h-full ${bgClass} overflow-y-auto`}>
      <header className="h-[56px] flex items-center justify-between px-1 bg-chat-other-bg border-b border-border-color sticky top-0 z-20 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{title}</h2>
        </div>
        <div className="flex-1 flex justify-end pr-1">{rightElement}</div>
      </header>
      <main className="flex-1 relative flex flex-col min-h-0">{children}</main>
    </div>
  );
};
