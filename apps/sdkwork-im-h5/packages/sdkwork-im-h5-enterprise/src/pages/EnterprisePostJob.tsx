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
  		showToast(t('enterprise.auto_fn_n69a32ba5', '请填写完整信息'));
  		return;
  	}
  	showToast(t('enterprise.auto_fn_n173e5342', '招聘发布成功'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_278730af', '发布招聘')} bgClass="bg-white dark:bg-[#1a1b1c]">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_n369a7e4c', '发布新岗位')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_2ee18e1e', '招聘职位')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n485059bb', '例如：高级前端开发')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3ee522ab', '薪资范围')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n43d35e5b', '例如：15k-30k')} value={formData.salary} onChange={e => setFormData({...formData, salary: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_2be383e0', '工作地点')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n72a04fa8', '例如：北京')} value={formData.location} onChange={e => setFormData({...formData, location: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3b886242', '职位描述')}</label>
          <textarea className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_21ff1ace', '请输入职位要求和岗位职责')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_278730af', '发布招聘')}</button>
      </div>
    </PageLayout>
  );
};
