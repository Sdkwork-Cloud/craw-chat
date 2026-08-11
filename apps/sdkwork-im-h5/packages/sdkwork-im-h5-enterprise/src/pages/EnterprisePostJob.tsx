import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export const EnterprisePostJob = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [formData, setFormData] = useState({ title: "", salary: "", location: "", desc: "" });

  const handleSubmit = () => {
  if (!formData.title || !formData.salary || !formData.desc) {
  		showToast(t('enterprise.auto_fn_n69a32ba5', 'Please complete all fields'));
  		return;
  	}
  	showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_278730af', 'Post a job')} bgClass="bg-chat-other-bg">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_n369a7e4c', 'Post a new position')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_2ee18e1e', 'Job title')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n485059bb', 'e.g., Senior frontend developer')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3ee522ab', 'Salary range')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n43d35e5b', 'e.g., 15k-30k')} value={formData.salary} onChange={e => setFormData({...formData, salary: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_2be383e0', 'Location')}</label>
          <input className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n72a04fa8', 'e.g., Beijing')} value={formData.location} onChange={e => setFormData({...formData, location: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3b886242', 'Job description')}</label>
          <textarea className="w-full bg-input-bg p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_21ff1ace', 'Enter job requirements and responsibilities')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_278730af', 'Post a job')}</button>
      </div>
    </PageLayout>
  );
};
