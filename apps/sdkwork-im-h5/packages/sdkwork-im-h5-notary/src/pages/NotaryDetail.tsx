import React, { useEffect, useState } from "react";
import { ChevronLeft, File, Loader2, MoreHorizontal, X } from "lucide-react";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

import {
  ActionSheet,
  IconButton,
  Tabs,
  showToast,
} from "@sdkwork/im-h5-commons";
import { NotaryDetailInfoCard } from "../components/NotaryDetailInfoCard";
import { NotaryDetailMaterials } from "../components/NotaryDetailMaterials";
import { NotaryDetailParties } from "../components/NotaryDetailParties";
import {
  notaryService,
  type NotaryDetailData,
  type NotaryFile,
} from "../services/notaryService";

type DetailTab = "parties" | "materials";

export const NotaryDetail: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { id } = useParams();
  const [detail, setDetail] = useState<NotaryDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const [reloadVersion, setReloadVersion] = useState(0);
  const [activeTab, setActiveTab] = useState<DetailTab>("parties");
  const [actionsOpen, setActionsOpen] = useState(false);
  const [preview, setPreview] = useState<{
    type: "image" | "video";
    url: string;
    name: string;
  } | null>(null);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setLoadError(false);
    if (!id) {
      setLoading(false);
      setLoadError(true);
      return () => {
        active = false;
      };
    }
    void notaryService.getNotaryDetail(id).then(
      (value) => {
        if (active) {
          setDetail(value);
        }
      },
      () => {
        if (active) {
          setLoadError(true);
        }
      },
    ).finally(() => {
      if (active) {
        setLoading(false);
      }
    });
    return () => {
      active = false;
    };
  }, [id, reloadVersion]);

  const openFile = (file: NotaryFile) => {
    if (
      (file.fileType === "image" || file.fileType === "video")
      && file.previewUrl
    ) {
      setPreview({
        type: file.fileType,
        url: file.previewUrl,
        name: file.name,
      });
      return;
    }
    showToast(t("notary.detail.preview_unavailable", "Preview unavailable"));
  };

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center bg-bg-color">
        <Loader2 className="h-7 w-7 animate-spin text-primary-blue" />
      </div>
    );
  }

  if (loadError || !detail) {
    return (
      <div className="flex h-full flex-col items-center justify-center gap-4 bg-bg-color">
        <File className="h-10 w-10 text-text-sub" />
        <span className="text-[14px] text-text-sub">
          {t("notary.detail.load_failed", "Unable to load notary case")}
        </span>
        <button
          type="button"
          className="text-[14px] font-medium text-primary-blue"
          onClick={() => setReloadVersion((version) => version + 1)}
        >
          {t("notary.records.retry", "Retry")}
        </button>
      </div>
    );
  }

  const isFinalState = [
    "COMPLETED",
    "REJECTED",
    "CANCELLED",
    "CREATE_FAILED",
  ].includes(detail.status);

  return (
    <div className="relative flex h-full flex-col bg-bg-color text-text-main">
      <header className="glass-header flex h-[56px] shrink-0 items-center border-b border-border-color px-1 pt-safe">
        <IconButton
          icon={<ChevronLeft className="h-6 w-6" />}
          onClick={() => navigate(-1)}
        />
        <h1 className="flex-1 text-center text-[17px] font-semibold">
          {t("notary.detail.title", "Notary case")}
        </h1>
        <IconButton
          icon={<MoreHorizontal className="h-5 w-5" />}
          onClick={() => setActionsOpen(true)}
        />
      </header>

      <div className="min-h-0 flex-1 overflow-y-auto">
        <NotaryDetailInfoCard detail={detail} />
        <div className="sticky top-0 z-10 border-b border-border-color bg-bg-color px-4">
          <Tabs
            tabs={[
              { id: "parties", label: t("notary.detail_parties", "Parties") },
              { id: "materials", label: t("notary.detail_materials", "Materials") },
            ]}
            activeTab={activeTab}
            onChange={(tab) => setActiveTab(tab as DetailTab)}
            className="justify-around"
            itemClassName="flex-1 justify-center text-center"
            activeItemClassName="text-primary-blue"
          />
        </div>
        {activeTab === "parties" ? (
          <NotaryDetailParties
            parties={detail.parties}
            isFinalState={isFinalState}
            onNavigateToSignature={(party) => navigate(
              `/notary/cases/${detail.id}/parties/${party.id}/signature`,
            )}
            onNavigateToVideo={(party) => navigate(
              `/notary/cases/${detail.id}/parties/${party.id}/video`,
            )}
          />
        ) : (
          <NotaryDetailMaterials
            materials={detail.materials}
            onFileClick={openFile}
          />
        )}
      </div>

      {preview && (
        <div className="fixed inset-0 z-[500] flex items-center justify-center bg-black/95 p-4">
          <button
            type="button"
            className="absolute right-3 top-[calc(env(safe-area-inset-top)+12px)] flex h-10 w-10 items-center justify-center text-white"
            onClick={() => setPreview(null)}
            aria-label={t("common.close", "Close")}
          >
            <X className="h-6 w-6" />
          </button>
          {preview.type === "image" ? (
            <img
              src={preview.url}
              alt={preview.name}
              className="max-h-full max-w-full object-contain"
            />
          ) : (
            <video
              src={preview.url}
              controls
              playsInline
              className="max-h-full max-w-full"
            />
          )}
        </div>
      )}

      <ActionSheet
        isOpen={actionsOpen}
        onClose={() => setActionsOpen(false)}
        title={t("notary.detail.actions", "Case actions")}
        options={[
          {
            label: t("notary.detail.copy_case_id", "Copy case ID"),
            onClick: async () => {
              try {
                await navigator.clipboard.writeText(detail.id);
                showToast(t("notary.detail.case_id_copied", "Case ID copied"));
              } catch {
                showToast(t("notary.detail.copy_failed", "Unable to copy case ID"));
              }
              setActionsOpen(false);
            },
          },
        ]}
      />
    </div>
  );
};
