import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export const EnterprisePostSupply = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [formData, setFormData] = useState({ title: "", type: "", price: "", desc: "" });

  const handleSubmit = () => {
  if (!formData.title || !formData.type || !formData.desc) {
  		showToast(t('enterprise.auto_fn_n69a32ba5', '请填写完整信息'));
  		return;
  	}
  	showToast(t('enterprise.auto_fn_33a6183a', '供应发布成功'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_2784ba2b', '发布供应')} bgClass="bg-white dark:bg-[#1a1b1c]">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_5948e819', '发布供应信息')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_25a0018a', '供应标题')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n702a1f51', '例如：大批量提供高端工业无人机')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_250e698a', '产品类别')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n46d28a3f', '例如：工业设备')} value={formData.type} onChange={e => setFormData({...formData, type: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_259cc83e', '供应价格')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_2fc247e8', '例如：面议或具体价格')} value={formData.price} onChange={e => setFormData({...formData, price: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_4185603a', '详细说明')}</label>
          <textarea className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_n7d30e0d3', '请输入供应详情...')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_2784ba2b', '发布供应')}</button>
      </div>
    </PageLayout>
  );
};
