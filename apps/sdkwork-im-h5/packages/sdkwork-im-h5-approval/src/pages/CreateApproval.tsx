import React, { useState } from "react";
import {
  PageLayout,
  showToast,
  ActionSheet,
} from "@sdkwork/im-h5-commons";
import { useNavigate, useSearchParams } from "react-router";
import {
  ApprovalService,
  SubmitApprovalRequest,
} from "../services/ApprovalService";
import { useTranslation } from "react-i18next";
import { ApprovalFormItem } from "../components/ApprovalFormItem";
import { ApprovalAttachmentsPicker } from "../components/ApprovalAttachmentsPicker";
import { ApprovalApproversPicker } from "../components/ApprovalApproversPicker";

export const CreateApproval = () => {
  const { t } = useTranslation();
  
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  
  const [formData, setFormData] = useState<SubmitApprovalRequest>({
    title: "",
    type: searchParams.get('type') || t('approval.createForm.typeLeave'),
    content: "",
    approverIds: [],
    attachments: [],
  });
  const [isTypeSheetOpen, setIsTypeSheetOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    if (!formData.title) return showToast(t('approval.createForm.titleEmpty'));
    if (!formData.content) return showToast(t('approval.createForm.contentEmpty'));
    setLoading(true);
    try {
      await ApprovalService.submitApproval(formData);
      showToast(t('approval.createForm.submitSuccess'));
      navigate(-1);
    } catch (e) {
      const error = e as Error;
      showToast(error.message || t('approval.createForm.submitFailed'));
    } finally {
      setLoading(false);
    }
  };

  return (
    <PageLayout title={t('approval.createFrom', { type: formData.type })}>
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-8">
        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <ApprovalFormItem label={t('approval.createForm.type')} onClick={() => setIsTypeSheetOpen(true)}>
            <div className="flex justify-between items-center w-full">
              <span className="text-text-main">{formData.type}</span>
            </div>
          </ApprovalFormItem>
          <ApprovalFormItem label={t('approval.createForm.title')} required>
            <input
              type="text"
              placeholder={t('approval.createForm.titlePlaceholder')}
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.title}
              onChange={(e) =>
                setFormData((s) => ({ ...s, title: e.target.value }))
              }
            />
          </ApprovalFormItem>

          <ApprovalFormItem label={t('approval.createForm.content')} required>
            <textarea
              placeholder={t('approval.createForm.contentPlaceholder')}
              className="w-full text-[16px] bg-transparent outline-none py-1 min-h-[100px] text-text-main"
              value={formData.content}
              onChange={(e) =>
                setFormData((s) => ({ ...s, content: e.target.value }))
              }
            />
          </ApprovalFormItem>
        </div>

        <ApprovalAttachmentsPicker
          attachments={formData.attachments}
          onAddAttachments={(urls) =>
            setFormData((s) => ({
              ...s,
              attachments: [...(s.attachments || []), ...urls],
            }))
          }
          onRemoveAttachment={(index) =>
            setFormData((s) => ({
              ...s,
              attachments: s.attachments?.filter((_, i) => i !== index),
            }))
          }
        />

        <ApprovalApproversPicker
          approverIds={formData.approverIds}
          onAddApprover={(name) =>
            setFormData((s) => ({
              ...s,
              approverIds: [...(s.approverIds || []), name],
            }))
          }
          onRemoveApprover={(index) =>
            setFormData((s) => ({
              ...s,
              approverIds: s.approverIds?.filter((_, i) => i !== index),
            }))
          }
        />

        <div className="p-6 mt-4">
          <button
            className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90 disabled:opacity-50"
            onClick={handleSubmit}
            disabled={loading}
          >
            {loading ? t('approval.createForm.submitting') : t('approval.createForm.submit')}
          </button>
        </div>
      </div>

      <ActionSheet
        isOpen={isTypeSheetOpen}
        onClose={() => setIsTypeSheetOpen(false)}
        title={t('approval.createForm.selectType')}
        options={[
          {
            label: t('approval.createForm.typeLeave'),
            onClick: () => setFormData((s) => ({ ...s, type: t('approval.createForm.typeLeave') })),
          },
          {
            label: t('approval.createForm.typeExpense'),
            onClick: () => setFormData((s) => ({ ...s, type: t('approval.createForm.typeExpense') })),
          },
          {
            label: t('approval.createForm.typePurchase'),
            onClick: () => setFormData((s) => ({ ...s, type: t('approval.createForm.typePurchase') })),
          },
          {
            label: t('approval.createForm.typeCar'),
            onClick: () => setFormData((s) => ({ ...s, type: t('approval.createForm.typeCar') })),
          },
          {
            label: t('approval.createForm.typeGeneral'),
            onClick: () => setFormData((s) => ({ ...s, type: t('approval.createForm.typeGeneral') })),
          },
        ]}
      />
    </PageLayout>
  );
};

