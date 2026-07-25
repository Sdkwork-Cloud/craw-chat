import { useTranslation } from "react-i18next";
import React from "react";
import { ChevronLeft, Share2 } from "lucide-react";
import { useNavigate } from "react-router";

export const ProductHeader = () => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <header className="absolute top-0 left-0 right-0 z-10 flex items-center justify-between px-2 pt-safe h-[56px] text-white">
      <div
        className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer ml-2"
        onClick={() => navigate(-1)}
      >
        <ChevronLeft className="w-5 h-5" />
      </div>
      <div className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer mr-2">
        <Share2 className="w-4 h-4" />
      </div>
    </header>
  );
};
