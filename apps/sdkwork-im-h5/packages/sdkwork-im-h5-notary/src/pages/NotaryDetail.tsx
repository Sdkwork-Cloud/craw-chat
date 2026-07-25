import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import React, { useState, useEffect } from "react";
import {} from "react-router";
import {
  ChevronLeft,
  MoreHorizontal,
  Video,
  ShieldCheck,
  Loader2,
  File,
} from "lucide-react";
import {
  IconButton,
  cn,
  MediaPreview,
  showToast,
  ActionSheet,
  showPrompt,
  showConfirm,
  Tabs,
} from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { NotaryPartyParams } from "./NotaryAddParty";
import { GLOBAL_STORE } from "./CreateNotaryProcess";
import {
  notaryService,
  NotaryDetailData,
  NotaryFile,
} from "../services/notaryService";
import { NotaryFileItem } from "../components/NotaryFileItem";

import { NotaryDetailParties } from "../components/NotaryDetailParties";
import { NotaryDetailMaterials } from "../components/NotaryDetailMaterials";
import { NotaryDetailInfoCard } from "../components/NotaryDetailInfoCard";

export const NotaryDetail: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams();

  const [detail, setDetail] = useState<NotaryDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<"parties" | "materials">(
    "parties",
  );
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);

  // Preview state
  const [previewMedia, setPreviewMedia] = useState<{
    type: string;
    url: string;
    name?: string;
  } | null>(null);

  const touchStartX = React.useRef<number | null>(null);
  const touchStartY = React.useRef<number | null>(null);

  const handleTouchStart = (e: React.TouchEvent) => {
  touchStartX.current = e.touches[0].clientX;
    touchStartY.current = e.touches[0].clientY;
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
  if (touchStartX.current === null || touchStartY.current === null) return;
    const touchEndX = e.changedTouches[0].clientX;
    const touchEndY = e.changedTouches[0].clientY;

    const diffX = touchStartX.current - touchEndX;
    const diffY = touchStartY.current - touchEndY;

    if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 50) {
      if (diffX > 0 && activeTab === "parties") {
        setActiveTab("materials");
      } else if (diffX < 0 && activeTab === "materials") {
        setActiveTab("parties");
      }
    }
    touchStartX.current = null;
    touchStartY.current = null;
  };

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const data = await notaryService.getNotaryDetail(id || "1");
        setDetail(data);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    loadData();
  }, [id]);

  const handleEditParty = (p: any) => {
  // Pass context to the AddParty page for editing
    NotaryPartyParams.editData = p;
    NotaryPartyParams.onEdit = (updated: any) => {
      showToast(t('notary.auto_fn_705305e', "当事人信息已更新"));
    };
    navigate("/notary/add-party");
  };

  const handleFileClick = (file: NotaryFile) => {
  if (file.fileType === "image" || file.fileType === "video") {
      // Provide mockup URLs based on file type for preview
      const previewUrl =
        file.fileType === "image"
          ? "https://picsum.photos/seed/notaryfile/800/1200"
          : "https://www.w3schools.com/html/mov_bbb.mp4";

      setPreviewMedia({
        type: file.fileType,
        url: previewUrl,
        name: file.name,
      });
    } else {
      showToast(`正在外部应用中打开:\n${file.name}`);
    }
  };

  if (loading || !detail) {
    return (
      <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black font-sans relative animate-in slide-in-from-right z-10 w-full absolute inset-0 items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-primary-blue" />
        <span className="mt-4 text-[14px] text-text-sub">{t('notary.auto_n18559448', "正在加载公证详情...")}</span>
      </div>
    );
  }

  const isFinalState = detail.status === "已完成" || detail.status === "completed" || detail.status === "已处理" || detail.status === "已撤销" || detail.status === "cancelled";

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black font-sans text-text-main relative animate-in slide-in-from-right z-10 w-full absolute inset-0">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-2 z-20 glass-header border-b border-border-color">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={
              <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-medium text-[17px] pointer-events-none flex-1">{t('notary.auto_27211834', "公证详情")}</div>
        <div className="flex justify-end z-10 w-[80px] pl-2">
          {/* Mini-program style capsule button */}
          <div className="flex items-center bg-black/5 dark:bg-white/10 rounded-full h-[32px] border border-black/5 dark:border-white/10 overflow-hidden shrink-0 mt-1">
            <div
              className="flex items-center justify-center w-[40px] h-full cursor-pointer active:bg-black/10 transition-colors"
              onClick={() => setIsActionSheetOpen(true)}
            >
              <MoreHorizontal className="w-5 h-5 text-text-main" />
            </div>
            <div className="w-[1px] h-4 bg-black/10 dark:bg-white/10" />
            <div
              className="flex items-center justify-center w-[40px] h-full cursor-pointer active:bg-black/10 transition-colors"
              onClick={() => navigate("/workspace")}
            >
              <div className="w-5 h-5 rounded-full border border-text-main/80 flex items-center justify-center">
                <div className="w-2 h-2 rounded-full bg-text-main/80" />
              </div>
            </div>
          </div>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto">
        {/* Main Info Block */}
        <NotaryDetailInfoCard detail={detail} />

        {/* Tabs block */}
        <div className="bg-bg-color min-h-[500px]">
          <div className="border-b border-border-color sticky top-0 bg-bg-color z-10 px-4">
             <Tabs
               tabs={[
                 { id: 'parties', name: t('notary.detail_parties', "当事人") },
                 { id: 'materials', name: t('notary.detail_materials', "公证材料") }
               ]}
               activeTab={activeTab}
               onChange={(id) => setActiveTab(id as any)}
               className="justify-around"
               itemClassName="flex-1 py-3 text-center text-[16px] text-text-sub flex justify-center items-center"
               activeItemClassName="text-primary-blue text-[16px]"
             />
          </div>

          {/* Tab Content */}
          <div
            className="bg-bg-color min-h-[300px]"
            onTouchStart={handleTouchStart}
            onTouchEnd={handleTouchEnd}
          >
            <AnimatePresence mode="wait">
              {activeTab === "parties" && (
                <motion.div
                  key="parties"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  transition={{ duration: 0.2 }}
                  className="flex flex-col bg-[#f4f6f9] dark:bg-black"
                >
                  <NotaryDetailParties
                    parties={detail.parties}
                    isFinalState={isFinalState}
                    onEditParty={handleEditParty}
                    onNavigateToSignature={(p) => {
                      if (!GLOBAL_STORE.parties.find(x => x.id === p.id)) {
                        GLOBAL_STORE.parties.push(p);
                      }
                      navigate(`/notary/party-signature/${p.id}`);
                    }}
                    onNavigateToVideo={(p) => {
                      navigate(`/call/video-notary/${p.id}`);
                    }}
                  />
                </motion.div>
              )}
              {activeTab === "materials" && (
                <motion.div
                  key="materials"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  transition={{ duration: 0.2 }}
                  className="flex flex-col"
                >
                  <NotaryDetailMaterials materials={detail.materials || []} onFileClick={handleFileClick} />
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>

      {/* Media Preview Overlay */}
      <MediaPreview
        media={previewMedia as any}
        onClose={() => setPreviewMedia(null)}
      />

      <ActionSheet
        isOpen={isActionSheetOpen}
        onClose={() => setIsActionSheetOpen(false)}
        title={t('notary.auto_prop_41808bcf', "详细操作")}
        options={[
          ...(!isFinalState ? [{
            label: t('notary.auto_fn_n38d7c4de', "修改公证"),
            onClick: () => {
              showToast(t('notary.auto_fn_n395aa11f', "暂未开放编辑"));
              setIsActionSheetOpen(false);
            }
          }] : []),
          {
            label: t('notary.auto_fn_5b9fbca5', "与公证员沟通"),
            onClick: () => navigate(`/notary/chat/${id}`),
          },
          {
            label: t('notary.auto_fn_1efbcbc', "分享公证"),
            onClick: async () => {
              await showPrompt(
                t('notary.auto_n5b004245', "分享链接："),
                "https://sdkwork_im_h5.sdkwork.com/notary/" + id,
              );
            },
          },
          {
            label: t('notary.auto_fn_6d709ec5', "复制公证号"),
            onClick: () => {
              if (detail) {
                navigator.clipboard.writeText(detail.id);
                showToast(t('notary.auto_45dfd785', "已复制：") + detail.id);
              }
            },
          },
          {
            label: t('notary.auto_fn_6938a9bd', "发起视频通话"),
            onClick: () => navigate(`/call/video-notary/${id}`),
          },
          {
            label: t('notary.auto_fn_1821af2f', "撤销"),
            danger: true,
            onClick: async () => {
              const confirm = await showConfirm(t('notary.auto_fn_46d3fdae', "确定要撤销公证吗？"));
              if (confirm) {
                await notaryService.updateRecordStatus(id!, "cancelled");
                showToast(t('notary.auto_fn_37dd4bb8', "该公证已申请撤销"));
                navigate(-1);
              }
            },
          },
        ]}
      />
    </div>
  );
};
