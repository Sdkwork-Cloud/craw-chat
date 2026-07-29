import { ImageOff, X } from "lucide-react";
import { useTranslation } from "react-i18next";

import { IconButton } from "@sdkwork/im-h5-commons";

interface CharacterAssetSelectorModalProps {
  isOpen: boolean;
  assets: {
    referenceImage: string | null;
    introVideo: string | null;
  };
  onClose: () => void;
  onUpdateAssets: (assets: {
    referenceImage: string | null;
    introVideo: string | null;
  }) => void;
}

export function CharacterAssetSelectorModal({
  isOpen,
  assets: _assets,
  onClose,
  onUpdateAssets: _onUpdateAssets,
}: CharacterAssetSelectorModalProps) {
  const { t } = useTranslation("user");

  if (!isOpen) {
    return null;
  }

  return (
    <div
      aria-modal="true"
      className="absolute inset-0 z-50 flex items-end bg-black/40"
      role="dialog"
    >
      <div className="flex w-full flex-col items-center gap-4 rounded-t-lg bg-bg-color p-6 pb-safe text-center">
        <div className="flex w-full justify-end">
          <IconButton icon={<X className="h-6 w-6" />} onClick={onClose} />
        </div>
        <ImageOff aria-hidden="true" className="h-8 w-8 text-text-sub" />
        <h2 className="m-0 text-base font-semibold text-text-main">
          {t("capability_title")}
        </h2>
        <p className="m-0 text-sm text-text-sub">{t("capability_unavailable")}</p>
      </div>
    </div>
  );
}
