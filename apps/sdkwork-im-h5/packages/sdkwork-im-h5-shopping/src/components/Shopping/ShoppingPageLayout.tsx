import React from "react";
import { useNavigate } from "react-router";
import { ChevronLeft } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface ShoppingPageLayoutProps {
  title?: string;
  children: React.ReactNode;
  rightElement?: React.ReactNode;
}

export const ShoppingPageLayout: React.FC<ShoppingPageLayoutProps> = ({
  title,
  children,
  rightElement = null,
}) => {
  const navigate = useNavigate();
  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      <header className="flex items-center px-2 pt-safe h-[56px] shrink-0 sticky top-0 bg-bg-color/80 backdrop-blur-md z-10">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{title}</h2>
        </div>
        <div className="flex-1 flex justify-end pr-1">{rightElement}</div>
      </header>
      <div className="flex flex-col px-0 sm:px-4 pb-12 mt-2">{children}</div>
    </div>
  );
};
