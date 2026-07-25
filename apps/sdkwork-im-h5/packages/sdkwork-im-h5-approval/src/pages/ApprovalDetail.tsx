import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate, useParams } from "react-router";
import { ApprovalService, ApprovalItem } from "../services/ApprovalService";
import { ApprovalDetailInfoCard } from "../components/ApprovalDetailInfoCard";
import { ApprovalProcessTimeline } from "../components/ApprovalProcessTimeline";
import { ApprovalBottomActions } from "../components/ApprovalBottomActions";

export const ApprovalDetail = () => {
  const { t } = useTranslation();
  const { id } = useParams();
  const navigate = useNavigate();
  const [approval, setApproval] = useState<ApprovalItem | null>(null);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (id) {
      ApprovalService.getApprovalDetail(id).then(setApproval);
    }
  }, [id]);

  const handleAction = async (action: "approve" | "reject") => {
    if (!id) return;
    setSubmitting(true);
    try {
      await ApprovalService.handleApproval({ id, action, comment: "" });
      showToast(action === "approve" ? "已同意" : "已拒绝");
      navigate(-1);
    } catch (e) {
      showToast("操作失败");
    } finally {
      setSubmitting(false);
    }
  };

  if (!approval)
    return (
      <PageLayout title="审批详情">
        <div className="flex flex-col h-full items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">加载中...</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title="审批详情">
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[100px]">
        <ApprovalDetailInfoCard approval={approval} />
        <ApprovalProcessTimeline approval={approval} />
      </div>

      {approval.status === "pending" && (
        <ApprovalBottomActions
          submitting={submitting}
          onAction={handleAction}
        />
      )}
    </PageLayout>
  );
};

