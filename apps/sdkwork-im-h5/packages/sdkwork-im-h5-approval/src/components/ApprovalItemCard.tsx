import React from 'react';
import { motion } from 'motion/react';
import { Clock, CheckCircle2, XCircle, UserCheck, Plane, ShoppingCart, FileText, ChevronRight } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { ApprovalItem, ApprovalStatus } from '../services/ApprovalService';
import { useTranslation } from 'react-i18next';

export const ApprovalItemCard: React.FC<{
  approval: ApprovalItem;
  onClick: () => void;
}> = ({ approval, onClick }) => {
  const { t } = useTranslation();

  const getStatusIcon = (status: ApprovalStatus) => {
  switch (status) {
      case "pending": return <Clock className="w-4 h-4 text-orange-500" />;
      case "approved": return <CheckCircle2 className="w-4 h-4 text-emerald-500" />;
      case "rejected": return <XCircle className="w-4 h-4 text-rose-500" />;
      case "withdrawn": return <XCircle className="w-4 h-4 text-gray-400" />;
    }
  };

  const getStatusText = (status: ApprovalStatus) => {
  switch (status) {
      case "pending": return t('approval.status.pending');
      case "approved": return t('approval.status.approved');
      case "rejected": return t('approval.status.rejected');
      case "withdrawn": return t('approval.status.withdrawn');
    }
  };

  const getTypeIcon = (type: string) => {
  switch (type) {
      case "请假":
      case t('approval.types.leave'):
        return <UserCheck className="w-5 h-5 text-indigo-500" />;
      case "报销":
      case t('approval.types.expense'):
        return <Plane className="w-5 h-5 text-blue-500" />;
      case "采购":
      case t('approval.types.purchase'):
        return <ShoppingCart className="w-5 h-5 text-orange-500" />;
      default:
        return <FileText className="w-5 h-5 text-primary-blue" />;
    }
  };

  const getTypeText = (type: string) => {
  switch(type) {
      case "请假": return t('approval.types.leave');
      case "报销": return t('approval.types.expense');
      case "采购": return t('approval.types.purchase');
      default: return type;
    }
  }

  return (
    <motion.div
      whileTap={{ scale: 0.98 }}
      onClick={onClick}
      className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
    >
      <div className="flex justify-between items-start mb-3">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-gray-100 dark:bg-[#3a3b3c] flex items-center justify-center">
            {getTypeIcon(approval.type)}
          </div>
          <div>
            <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
              {approval.applicant} {t('approval.of')} {getTypeText(approval.type)}
            </div>
            <div className="text-[13px] text-text-sub font-mono">
              {approval.date}
            </div>
          </div>
        </div>
        <div className="flex flex-col items-end">
          <div className="flex items-center gap-1.5 text-[14px] font-medium mb-1">
            {getStatusIcon(approval.status)}
            <span
              className={cn(
                approval.status === "pending" && "text-orange-500",
                approval.status === "approved" && "text-emerald-500",
                approval.status === "rejected" && "text-rose-500",
                approval.status === "withdrawn" && "text-gray-400",
              )}
            >
              {getStatusText(approval.status)}
            </span>
          </div>
        </div>
      </div>
      <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex items-center justify-between">
        <span className="truncate pr-4">{approval.title}</span>
        <ChevronRight className="w-4 h-4 text-text-sub shrink-0" />
      </div>
    </motion.div>
  );
};
