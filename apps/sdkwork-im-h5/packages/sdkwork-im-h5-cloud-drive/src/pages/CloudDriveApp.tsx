import React, { useState, useEffect } from "react";
import {
  showPrompt,
  PageLayout,
  IconButton,
  showToast,
  ActionSheet,
} from "@sdkwork/im-h5-commons";
import {
  Search,
  Filter,
  Plus,
  HardDrive,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { CloudDriveService, CloudFile } from "../services/CloudDriveService";
import { motion } from "motion/react";
import { CloudDriveHeaderStats } from "../components/CloudDriveHeaderStats";
import { CloudDriveActionGrid } from "../components/CloudDriveActionGrid";
import { CloudDriveFileItem } from "../components/CloudDriveFileItem";

export const CloudDriveApp = () => {
  const { t } = useTranslation();
const [activeTab, setActiveTab] = useState<"recent" | "files" | "shared">("files");
  const [files, setFiles] = useState<CloudFile[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isFabSheetOpen, setIsFabSheetOpen] = useState(false);
  const [activeFile, setActiveFile] = useState<string | null>(null);

  useEffect(() => {
    setIsLoading(true);
    CloudDriveService.getFiles().then(data => {
      setFiles(data);
      setIsLoading(false);
    });
  }, []);

  return (
    <PageLayout title={t('drive.title')}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        <CloudDriveHeaderStats />
        <CloudDriveActionGrid activeTab={activeTab} setActiveTab={setActiveTab} />

        {/* File List */}
        <div className="flex-1 overflow-y-auto px-4 pb-20">
          <div className="flex items-center justify-between py-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {activeTab === "recent" ? t('drive.sections.recent_used') : t('drive.sections.all_files')}
            </h2>
            <div className="flex gap-2">
              <IconButton
                icon={<Filter className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
            </div>
          </div>

          <div className="flex flex-col gap-2">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">{t('drive.loading')}</p>
              </div>
            ) : files.length > 0 ? (
              files.map((file) => (
                <CloudDriveFileItem key={file.id} file={file} setActiveFile={setActiveFile} />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <HardDrive className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('drive.no_files')}</span>
              </div>
            )}
          </div>
        </div>

        {/* FAB */}
        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => setIsFabSheetOpen(true)}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>

      <ActionSheet
        isOpen={isFabSheetOpen}
        onClose={() => setIsFabSheetOpen(false)}
        title={t('drive.upload_file_title')}
        options={[
          {
            label: t('drive.new_folder'),
            onClick: async () => {
              await CloudDriveService.createFolder(t('drive.new_folder'));
              setFiles(await CloudDriveService.getFiles());
              showToast(t('drive.folder_created'));
            },
          },
          {
            label: t('drive.upload_file'),
            onClick: async () => {
              await CloudDriveService.uploadFile(
                new File([""], `新文件_${Date.now()}.txt`, {
                  type: "text/plain",
                }),
              );
              setFiles(await CloudDriveService.getFiles());
              showToast(t('drive.file_uploaded'));
            },
          },
        ]}
      />

      <ActionSheet
        isOpen={activeFile !== null}
        onClose={() => setActiveFile(null)}
        title={t('drive.file_actions_title')}
        options={[
          { label: t('drive.share'), onClick: () => showToast(t('drive.link_copied')) },
          {
            label: t('drive.rename'),
            onClick: async () => {
              const fileData = files.find((f) => f.id === activeFile);
              if (activeFile && fileData) {
                const newName = await showPrompt(t('drive.enter_new_name'), fileData.name);
                if (newName && newName.trim()) {
                  await CloudDriveService.renameFile(
                    activeFile,
                    newName.trim(),
                  );
                  setFiles(await CloudDriveService.getFiles());
                }
              }
            },
          },
          {
            label: t('drive.delete'),
            danger: true,
            onClick: async () => {
              if (activeFile) {
                await CloudDriveService.deleteFile(activeFile);
                setFiles(await CloudDriveService.getFiles());
                showToast(t('drive.file_deleted'));
              }
            },
          },
        ]}
      />
    </PageLayout>
  );
};
