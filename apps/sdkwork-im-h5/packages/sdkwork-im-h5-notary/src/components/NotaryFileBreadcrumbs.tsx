import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface FolderStackItem {
  id: string;
  name: string;
}

interface NotaryFileBreadcrumbsProps {
  folderStack: FolderStackItem[];
  onRootClick: () => void;
  onFolderClick: (index: number) => void;
}

export const NotaryFileBreadcrumbs: React.FC<NotaryFileBreadcrumbsProps> = ({
  folderStack,
  onRootClick,
  onFolderClick,
}) => {
  const { t } = useTranslation();

  if (folderStack.length === 0) return null;

  return (
    <div className="px-4 py-3 bg-white dark:bg-[#1c1c1e] text-[13px] text-primary-blue flex items-center border-b border-border-color/50 overflow-x-auto no-scrollbar whitespace-nowrap">
      <span
        className="cursor-pointer active:opacity-70"
        onClick={onRootClick}
      >
        {t("notary.files.title")}
      </span>
      {folderStack.map((folder, idx) => (
        <React.Fragment key={folder.id}>
          <ChevronRight className="w-4 h-4 mx-1 text-text-sub/50" />
          <span
            className={cn(
              "cursor-pointer",
              idx === folderStack.length - 1
                ? "text-text-main font-medium"
                : "active:opacity-70"
            )}
            onClick={() => onFolderClick(idx)}
          >
            {folder.name}
          </span>
        </React.Fragment>
      ))}
    </div>
  );
};
