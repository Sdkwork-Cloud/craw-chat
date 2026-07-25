import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, FolderOpen } from "lucide-react";
import { WorkService, Work } from "../services/WorkService";
import { cn, showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { WorkCard } from "../components/WorkCard";

export const MyWorksPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [works, setWorks] = useState<Work[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<
    "all" | "video" | "article" | "audio" | "ai_image"
  >("all");
  const [actionSheetItem, setActionSheetItem] = useState<Work | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  useEffect(() => {
    loadWorks();
  }, []);

  const loadWorks = async () => {
    setLoading(true);
    try {
      const data = await WorkService.getMyWorks();
      setWorks(data);
    } catch (error) {
      showToast(t('user.auto_fn_27b527b1', '加载失败'));
    } finally {
      setLoading(false);
    }
  };

  const startLongPress = (work: Work) => {
  const handlePressStart = () => {
  setIsLongPressed(false);
      (window as any).longPressTimeout = setTimeout(() => {
        setIsLongPressed(true);
        setActionSheetItem(work);
      }, 500);
    };

    const handlePressEnd = () => {
  clearTimeout((window as any).longPressTimeout);
    };

    return {
      onPointerDown: handlePressStart,
      onPointerUp: handlePressEnd,
      onPointerLeave: () => {
        handlePressEnd();
        setIsLongPressed(false);
      },
      onContextMenu: (e: React.MouseEvent) => {
        e.preventDefault();
        handlePressStart();
        setIsLongPressed(true);
        setActionSheetItem(work);
        handlePressEnd();
      }
    };
  };

  const handleActionSheetSelect = async (action: string) => {
    if (!actionSheetItem) return;
    if (action === 'edit') {
       navigate(`/work/${actionSheetItem.id}/edit`);
    } else if (action === 'delete') {
       try {
         await WorkService.deleteWork(actionSheetItem.id);
         setWorks(works.filter((w) => w.id !== actionSheetItem.id));
         showToast(t('user.auto_fn_536f8d1b', '已删除作品'));
       } catch (error) {
         showToast(t('user.auto_fn_2794e158', '删除失败'));
       }
    } else if (action === 'share') {
       showToast(t('user.auto_fn_267caab4', '分享成功'));
    }
    setActionSheetItem(null);
  };

  const filteredWorks = works.filter(
    (w) => activeTab === "all" || w.type === activeTab,
  );

  const tabs = [
    { id: "all", label: "全部" },
    { id: "video", label: "视频" },
    { id: "article", label: "图文" },
    { id: "audio", label: "音频" },
    { id: "ai_image", label: "AI作画" },
  ];

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">{t('user.auto_2e5aeeb8', '我的作品')}</span>
        <div className="w-10 h-10" />
      </header>

      {/* Tabs */}
      <div className="flex items-center px-4 h-[44px] border-b border-border-color bg-bg-color shrink-0 sticky top-0 z-10 gap-6 overflow-x-auto no-scrollbar">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            className={cn(
              "h-full flex items-center relative whitespace-nowrap cursor-pointer transition-colors",
              activeTab === tab.id
                ? "text-text-main font-medium"
                : "text-text-sub",
            )}
            onClick={() => setActiveTab(tab.id as any)}
          >
            <span className="text-[15px]">{tab.label}</span>
            {activeTab === tab.id && (
              <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                <div className="w-4 h-0.5 bg-primary-blue rounded-full" />
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto bg-chat-other-bg pb-12 w-full">
        {loading ? (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
            <span className="text-[14px]">{t('user.auto_7f6f37e', '加载中...')}</span>
          </div>
        ) : filteredWorks.length > 0 ? (
          <div className="grid grid-cols-2 gap-[2px]">
            {filteredWorks.map((work) => (
              <WorkCard
                key={work.id}
                work={work}
                onClick={() => {
                  if (isLongPressed) {
                    setIsLongPressed(false);
                    return;
                  }
                  navigate(`/work/${work.id}`);
                }}
                onMoreClick={(e) => {
                  e.stopPropagation();
                  setActionSheetItem(work);
                }}
                onLongPressProps={startLongPress(work)}
              />
            ))}
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <FolderOpen className="w-12 h-12 mb-3 stroke-current opacity-40" />
            <p className="text-[14px]">
              暂无
              {activeTab !== "all" &&
                tabs.find((t) => t.id === activeTab)?.label}
              作品
            </p>
          </div>
        )}
      </div>

      {/* Action Sheet */}
      {actionSheetItem && (
        <ActionSheet
          isOpen={true}
          title={"管理作品"}
          options={[
            { label: '分享作品', onClick: () => handleActionSheetSelect('share') },
            { label: '编辑作品', onClick: () => handleActionSheetSelect('edit') },
            { label: '删除此作品', danger: true, onClick: () => handleActionSheetSelect('delete') }
          ]}
          onClose={() => setActionSheetItem(null)}
        />
      )}
    </div>
  );
};
