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
  		showToast(t('enterprise.auto_fn_n69a32ba5', '请填写完整信息'));
  		return;
  	}
  	showToast(t('enterprise.auto_fn_n5709f9f4', '求购发布成功'));
  	setTimeout(() => navigate(-1), 1000);
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_278860fd', '发布求购')} bgClass="bg-white dark:bg-[#1a1b1c]">
      <div className="p-4 flex flex-col gap-4 max-w-full">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_6706caff', '发布求购需求')}</h2>
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_33553bdc', '求购标题')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n5fd8f852', '例如：急寻企业级服务器供应商')} value={formData.title} onChange={e => setFormData({...formData, title: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_33576f7b', '求购类别')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n7283eb69', '例如：IT设备')} value={formData.type} onChange={e => setFormData({...formData, type: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_44692e59', '采购预算')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_3e96089', '例如：100万-200万')} value={formData.budget} onChange={e => setFormData({...formData, budget: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_464bbe61', '需求详情')}</label>
          <textarea className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main min-h-[120px]" placeholder={t('enterprise.auto_prop_1b0a8551', '请输入具体的需求细节...')} value={formData.desc} onChange={e => setFormData({...formData, desc: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_278860fd', '发布求购')}</button>
      </div>
    </PageLayout>
  );
};
