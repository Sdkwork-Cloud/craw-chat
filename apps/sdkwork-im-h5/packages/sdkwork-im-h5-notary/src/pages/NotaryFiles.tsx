import React, { useState, useEffect, useRef } from "react";
import {
  Folder,
  MoreVertical,
  FileText,
  Search,
  Plus,
  Image as ImageIcon,
  File,
  ChevronLeft,
  X,
  ChevronRight,
} from "lucide-react";
import { showPrompt, cn } from "@sdkwork/im-h5-commons";
import {
  IconButton,
  showToast,
  ActionSheet,
} from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { notaryService } from "../services/notaryService";
import { useTranslation } from "react-i18next";
import { NotaryFileBreadcrumbs } from "../components/NotaryFileBreadcrumbs";
import { NotaryFileListItem } from "../components/NotaryFileListItem";

export const NotaryFiles: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [items, setItems] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [activeItem, setActiveItem] = useState<any>(null);

  // Folder navigation state
  const [folderStack, setFolderStack] = useState<{id: string, name: string}[]>([]);
  const currentFolderId = folderStack.length > 0 ? folderStack[folderStack.length - 1].id : undefined;

  useEffect(() => {
    // Optional: force reload from disk in dev if schema changes 
    setIsLoading(true);
    notaryService.getCloudFiles().then((data) => {
      setItems(data as any[]);
      setIsLoading(false);
    });
  }, []);

  const handleCreateNewFolder = async () => {
    const name = await showPrompt(t("notary.files.enter_folder_name"));
    if (name && name.trim()) {
      const newFolder: any = {
        id: Math.random().toString(),
        name: name.trim(),
        type: "folder",
        size: "-",
        date: t("notary.files.just_now"),
        uploadTime: t("notary.files.just_now"),
        uploader: t("notary.files.me"),
        iconColor: "text-yellow-400",
        fill: "fill-yellow-400",
      };
      if (currentFolderId) {
        newFolder.parentId = currentFolderId;
      }
      await notaryService.addCloudFile(newFolder);
      fetchFiles();
      showToast(t("notary.files.folder_created"));
    }
  };

  const handleUploadFile = async () => {
    const newFile: any = {
      id: Math.random().toString(),
      name: `${t("notary.files.uploaded_prefix")}${Date.now()}.png`,
      type: "image",
      size: "1.2 MB",
      date: t("notary.files.just_now"),
      uploadTime: t("notary.files.just_now"),
      uploader: t("notary.files.me"),
      iconColor: "text-green-500",
      bg: "bg-green-500/10",
    };
    if (currentFolderId) {
      newFile.parentId = currentFolderId;
    }
    await notaryService.addCloudFile(newFile);
    fetchFiles();
    showToast(t("notary.files.upload_success"));
  };

  const fetchFiles = () => {
  notaryService.getCloudFiles().then((data) => setItems(data as any[]));
  };

  const currentFolderItems = items.filter((item) => {
    if (searchQuery) return item.name.toLowerCase().includes(searchQuery.toLowerCase());
    return item.parentId === currentFolderId || (!item.parentId && !currentFolderId);
  });

  const getIcon = (type: string) => {
  switch (type) {
      case "folder":
        return Folder;
      case "image":
        return ImageIcon;
      case "pdf":
        return FileText;
      case "doc":
        return File;
      default:
        return File;
    }
  };

  // Long press handler
  const timeoutRef = useRef<any>(null);
  const handleTouchStart = (item: any) => {
  timeoutRef.current = setTimeout(() => {
      if (window.navigator?.vibrate) {
        window.navigator.vibrate(50);
      }
      setActiveItem(item);
    }, 600); // 600ms for long press
  };

  const clearTouchTimeout = () => {
  if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black fixed inset-0 z-50">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative border-b border-border-color">
        {isSearchOpen ? (
          <div className="flex items-center w-full px-2 gap-2">
            <div className="flex-1 bg-chat-other-bg h-[36px] rounded-full flex items-center px-3 border border-border-color">
              <Search className="w-4 h-4 text-text-sub shrink-0" />
              <input
                autoFocus
                type="text"
                placeholder={t("notary.files.search_files")}
                className="flex-1 bg-transparent px-2 text-[14px] outline-none text-text-main"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
              {searchQuery && (
                <X
                  className="w-4 h-4 text-text-sub cursor-pointer"
                  onClick={() => setSearchQuery("")}
                />
              )}
            </div>
            <div
              className="text-[15px] text-text-sub cursor-pointer px-2"
              onClick={async () => {
                setIsSearchOpen(false);
                setSearchQuery("");
              }}
            >
              {t("notary.files.cancel")}
            </div>
          </div>
        ) : (
          <>
            <div className="flex items-center z-10 w-16">
              {folderStack.length > 0 ? (
                <IconButton
                  icon={<ChevronLeft />}
                  onClick={() => {
                    setFolderStack(prev => prev.slice(0, prev.length - 1));
                  }}
                />
              ) : (
                <IconButton
                  icon={<ChevronLeft />}
                  onClick={() => navigate(-1)}
                />
              )}
            </div>
            <div className="absolute left-1/2 -translate-x-1/2 flex items-center pointer-events-none">
              <span className="text-[17px] font-bold text-text-main">
                {folderStack.length > 0 ? folderStack[folderStack.length - 1].name : t("notary.files.title")}
              </span>
            </div>
            <div className="flex items-center gap-1 z-10 pr-1 w-16 justify-end">
              <IconButton
                icon={<Search className="w-5 h-5 text-text-main" />}
                onClick={() => setIsSearchOpen(true)}
              />
              <IconButton
                icon={<Plus className="w-6 h-6 text-text-main" />}
                onClick={() => setIsActionSheetOpen(true)}
              />
            </div>
          </>
        )}
      </header>

      {/* Breadcrumb if nested */}
      {!isSearchOpen && (
        <NotaryFileBreadcrumbs
          folderStack={folderStack}
          onRootClick={() => setFolderStack([])}
          onFolderClick={(idx) => {
            if (idx < folderStack.length - 1) {
              setFolderStack((prev) => prev.slice(0, idx + 1));
            }
          }}
        />
      )}

      <div className="flex-1 overflow-y-auto pb-[90px] pt-2">
        {/* File List */}
        <div className="flex flex-col px-3">
          {isLoading ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
              <p className="text-[14px]">{t("notary.files.loading")}</p>
            </div>
          ) : (
            currentFolderItems.map((item) => (
              <NotaryFileListItem
                key={item.id}
                item={item}
                icon={getIcon(item.type)}
                onTouchStart={handleTouchStart}
                clearTouchTimeout={clearTouchTimeout}
                onItemClick={(fileItem) => {
                  if (fileItem.type === "folder") {
                    setFolderStack([
                      ...folderStack,
                      { id: fileItem.id, name: fileItem.name },
                    ]);
                  }
                }}
                onMoreClick={(fileItem, e) => {
                  e.stopPropagation();
                  setActiveItem(fileItem);
                }}
              />
            ))
          )}

          {!isLoading && currentFolderItems.length === 0 && (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <div className="w-20 h-20 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center mb-4">
                <Folder
                  className="w-10 h-10 opacity-40 stroke-current text-text-sub"
                  strokeWidth={1.5}
                />
              </div>
              <p className="text-[15px] font-medium text-text-main mb-1">{t("notary.files.empty_folder")}</p>
              <p className="text-[13px] text-text-sub">{t("notary.files.add_hint")}</p>
            </div>
          )}
        </div>
      </div>

      <ActionSheet
        isOpen={activeItem !== null}
        onClose={() => setActiveItem(null)}
        title={`${t("notary.files.actions_for")} "${activeItem?.name}"`}
        options={[
          { label: t("notary.files.share"), onClick: () => showToast(t("notary.files.link_copied")) },
          {
            label: t("notary.files.rename"),
            onClick: async () => {
              const newName = await showPrompt(
                t("notary.files.enter_new_name"),
                activeItem?.name,
              );
              if (newName && newName.trim()) {
                await notaryService.renameCloudFile(activeItem.id, newName);
                fetchFiles();
                showToast(t("notary.files.renamed"));
              }
            },
          },
          {
            label: t("notary.files.delete"),
            danger: true,
            onClick: async () => {
              await notaryService.deleteCloudFile(activeItem?.id);
              fetchFiles();
              showToast(t("notary.files.file_deleted"));
            },
          },
        ]}
      />

      <ActionSheet
        isOpen={isActionSheetOpen}
        onClose={() => setIsActionSheetOpen(false)}
        title={t("notary.files.add")}
        options={[
          {
            label: t("notary.files.new_folder"),
            onClick: handleCreateNewFolder,
          },
          {
            label: t("notary.files.upload_file_photo"),
            onClick: handleUploadFile,
          },
        ]}
      />
    </div>
  );
};

