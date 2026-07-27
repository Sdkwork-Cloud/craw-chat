import { useTranslation } from "react-i18next";
import React from "react";
import {
  FileText,
  Image as ImageIcon,
  Video,
  FileArchive,
  FileSpreadsheet,
  File,
  ChevronRight,
  MoreVertical,
  Folder,
} from "lucide-react";
import { cn, IconButton } from "@sdkwork/im-h5-commons";
import { NotaryFile, NotaryFileTag } from "../services/notaryService";

export interface NotaryFileItemProps {
  file: NotaryFile;
  onClick?: (file: NotaryFile) => void;
  className?: string;
  showMoreProps?: boolean;
}

export const NotaryFileItem: React.FC<NotaryFileItemProps> = ({
  file,
  onClick,
  className,
  showMoreProps = true,
}) => {
  const { t } = useTranslation();
// Determine icon and icon style based on file type matching NotaryFiles style
  const getFileIcon = (type: string) => {
  switch (type) {
      case "image":
        return <ImageIcon className="w-8 h-8 text-green-500" />;
      case "video":
        return <Video className="w-8 h-8 text-purple-500" />;
      case "pdf":
        return <FileText className="w-8 h-8 text-red-500" />;
      case "word":
        return <FileText className="w-8 h-8 text-blue-500" />;
      case "excel":
        return <FileSpreadsheet className="w-8 h-8 text-green-600" />;
      case "zip":
        return <FileArchive className="w-8 h-8 text-yellow-600" />;
      case "folder":
        return (
          <Folder
            className="w-8 h-8 text-yellow-400 fill-yellow-400"
            strokeWidth={1.5}
          />
        );
      default:
        return <File className="w-8 h-8 text-gray-400" />;
    }
  };

  const getIconBg = (type: string) => {
  switch (type) {
      case "image":
        return "bg-green-500/10";
      case "video":
        return "bg-purple-500/10";
      case "pdf":
        return "bg-red-500/10";
      case "word":
        return "bg-blue-500/10";
      case "excel":
        return "bg-green-600/10";
      case "zip":
        return "bg-yellow-600/10";
      case "folder":
        return "bg-transparent";
      default:
        return "bg-gray-500/10";
    }
  };

  const getTagStyle = (color: NotaryFileTag["color"]) => {
  switch (color) {
      case "blue":
        return "bg-blue-50 text-blue-600 border-blue-200 dark:bg-blue-900/20 dark:text-blue-400 dark:border-blue-800/30";
      case "green":
        return "bg-green-50 text-green-600 border-green-200 dark:bg-green-900/20 dark:text-green-400 dark:border-green-800/30";
      case "orange":
        return "bg-orange-50 text-orange-600 border-orange-200 dark:bg-orange-900/20 dark:text-orange-400 dark:border-orange-800/30";
      case "red":
        return "bg-red-50 text-red-600 border-red-200 dark:bg-red-900/20 dark:text-red-400 dark:border-red-800/30";
      case "gray":
        return "bg-gray-100 text-gray-500 border-gray-200 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-700";
      default:
        return "bg-gray-100 text-gray-600 border-gray-200";
    }
  };

  return (
    <div
      className={cn(
        "flex items-center pl-4 pr-2 py-3 active:bg-active-bg transition-colors cursor-pointer group",
        className,
      )}
      onClick={() => onClick && onClick(file)}
    >
      <div
        className={cn(
          "w-12 h-12 rounded-[14px] flex items-center justify-center shrink-0 mr-3",
          getIconBg(file.fileType),
        )}
      >
        {getFileIcon(file.fileType)}
      </div>

      <div
        className={cn(
          "flex-1 min-w-0 pb-3 flex items-start justify-between pt-1 border-b border-border-color/50",
        )}
      >
        <div className="flex flex-col min-w-0 pr-2">
          <span className="text-[16px] text-text-main font-medium truncate tracking-wide">
            {file.name}
          </span>

          <div className="flex items-center gap-2 mt-1 mb-1.5 flex-wrap">
            <span className="text-[12px] text-text-sub/80">
              {file.uploadTime}
            </span>
            {file.size !== "-" && (
              <span className="text-[12px] text-text-sub/80">{file.size}</span>
            )}
          </div>

          {file.tags && file.tags.length > 0 && (
            <div className="flex flex-wrap gap-1.5">
              {file.tags.map((tag, idx) => (
                <span
                  key={idx}
                  className={cn(
                    "px-1.5 py-0.5 text-[10px] font-medium rounded-sm border whitespace-nowrap",
                    getTagStyle(tag.color),
                  )}
                >
                  {tag.label}
                </span>
              ))}
            </div>
          )}
        </div>

        {showMoreProps && (
          <div className="pt-2">
            <IconButton
              icon={<MoreVertical className="w-5 h-5 text-text-sub" />}
              onClick={(e) => {
                e.stopPropagation();
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
};
