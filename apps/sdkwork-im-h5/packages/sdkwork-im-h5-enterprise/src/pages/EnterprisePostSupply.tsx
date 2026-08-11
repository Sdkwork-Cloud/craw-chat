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
  		showToast(t('enterprise.auto_fn_n69a32ba5', 'Please complete all fields'));
  		return;
  	}
  	showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_2784ba2b', 'Post supply')} bgClass="bg-chat-other-bg">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_5948e819', 'Post supply information')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_25a0018a', 'Supply title')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n702a1f51', 'e.g., Bulk supply of high-end industrial drones')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_250e698a', 'Product category')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n46d28a3f', 'e.g., Industrial equipment')} value={formData.type} onChange={e => setFormData({...formData, type: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_259cc83e', 'Supply price')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_2fc247e8', 'e.g., Negotiable or specific price')} value={formData.price} onChange={e => setFormData({...formData, price: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_4185603a', 'Details')}</label>
          <textarea className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_n7d30e0d3', 'Enter supply details...')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_2784ba2b', 'Post supply')}</button>
      </div>
    </PageLayout>
  );
};
