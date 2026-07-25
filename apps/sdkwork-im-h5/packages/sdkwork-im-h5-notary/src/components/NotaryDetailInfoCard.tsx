import React from "react";
import { useTranslation } from "react-i18next";

interface NotaryDetailInfoCardProps {
  detail: {
    title: string;
    time: string;
    item: string;
    notaryName: string;
    notaryNo: string;
    status: string;
    remarks: string;
  };
}

export const NotaryDetailInfoCard: React.FC<NotaryDetailInfoCardProps> = ({ detail }) => {
  const { t } = useTranslation();

  return (
    <div className="bg-bg-color px-5 pt-5 pb-6 mb-2 border-b border-border-color">
      <h1 className="text-[20px] font-bold mb-6">{detail.title}</h1>

      <div className="flex flex-col gap-4 text-[15px]">
        <div className="flex items-start gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_ceebe', "时间")}</span>
          <span className="flex-1 text-text-main">{detail.time}</span>
        </div>
        <div className="flex items-start gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_2719e1e3', "公证事项")}</span>
          <span className="flex-1 text-text-main font-medium">
            {detail.item}
          </span>
        </div>
        <div className="flex items-start gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_142e723', "公证员")}</span>
          <span className="flex-1 text-text-main">{detail.notaryName}</span>
        </div>
        <div className="flex items-start gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_n43ca9dfc', "公证员编号")}</span>
          <span className="flex-1 text-text-main break-all">
            {detail.notaryNo}
          </span>
        </div>
        <div className="flex items-center gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_e440b', "状态")}</span>
          <div className="px-2 py-0.5 border border-border-color bg-input-bg text-text-sub rounded-sm text-[13px]">
            {detail.status}
          </div>
        </div>
        <div className="flex items-start gap-4">
          <span className="w-[85px] text-text-sub shrink-0">{t('notary.auto_b34c1', "备注")}</span>
          <span className="flex-1 text-text-main leading-relaxed">
            {detail.remarks}
          </span>
        </div>
      </div>
    </div>
  );
};
