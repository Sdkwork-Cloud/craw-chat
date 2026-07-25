import { useTranslation } from "react-i18next";
import React from "react";
import { File } from "lucide-react";
import { NotaryFileItem } from "./NotaryFileItem";
import { NotaryFile } from "../services/notaryService";

interface NotaryDetailMaterialsProps {
  materials: NotaryFile[];
  onFileClick: (file: NotaryFile) => void;
}

export const NotaryDetailMaterials: React.FC<NotaryDetailMaterialsProps> = ({
  materials,
  onFileClick,
}) => {
  const { t } = useTranslation();
return (
    <div className="flex flex-col">
      {materials && materials.length > 0 ? (
        materials.map((file) => (
          <NotaryFileItem
            key={file.id}
            file={file}
            onClick={onFileClick}
          />
        ))
      ) : (
        <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
          <File className="w-12 h-12 mb-3 stroke-current opacity-40" />
          <span className="text-[14px]">{t('notary.auto_n2ad5cdd3', '暂无相对应的公证材料')}</span>
        </div>
      )}
    </div>
  );
};
