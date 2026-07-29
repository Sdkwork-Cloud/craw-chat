import React from "react";
import { useNavigate } from "react-router";
import { CameraOff, ChevronLeft } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const Scan: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="h-[56px] flex items-center px-1 border-b border-border-color pt-safe">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <h2 className="flex-1 pr-10 text-center text-[17px] font-medium text-text-main">
          {t('contacts.scan_qr')}
        </h2>
      </header>
      <div className="flex-1 flex flex-col items-center justify-center gap-3 px-8 text-center">
        <CameraOff className="w-10 h-10 text-text-sub" />
        <p className="text-[15px] text-text-main">
          {t('contacts.scan_unavailable', 'QR scanning is not available in this build.')}
        </p>
      </div>
    </div>
  );
};
