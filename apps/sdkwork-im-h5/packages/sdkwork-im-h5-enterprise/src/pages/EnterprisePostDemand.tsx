import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export const EnterprisePostDemand = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [formData, setFormData] = useState({ title: "", type: "", budget: "", desc: "" });

  const handleSubmit = () => {
  if (!formData.title || !formData.type || !formData.desc) {
  		showToast(t('enterprise.auto_fn_n69a32ba5', 'Please complete all fields'));
  		return;
  	}
  	showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_278860fd', 'Post buying request')} bgClass="bg-chat-other-bg">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_6706caff', 'Post a buying request')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_33553bdc', 'Buying title')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n5fd8f852', 'e.g., Urgently seeking enterprise server suppliers')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_33576f7b', 'Buying category')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n7283eb69', 'e.g., IT equipment')} value={formData.type} onChange={e => setFormData({...formData, type: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_44692e59', 'Purchase budget')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_3e96089', 'e.g., 1M-2M')} value={formData.budget} onChange={e => setFormData({...formData, budget: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_464bbe61', 'Requirements')}</label>
          <textarea className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_1b0a8551', 'Enter specific requirement details...')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_278860fd', 'Post buying request')}</button>
      </div>
    </PageLayout>
  );
};
