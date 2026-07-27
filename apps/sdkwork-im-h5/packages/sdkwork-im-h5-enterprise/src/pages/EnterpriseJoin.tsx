import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { ImagePlus, X } from "lucide-react";

export const EnterpriseJoin = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [formData, setFormData] = useState({ name: "", industry: "", contact: "", phone: "", logo: "", banner: "" });

  const handleSubmit = () => {
  if (!formData.name || !formData.industry || !formData.contact || !formData.phone || !formData.logo || !formData.banner) {
      showToast(t('enterprise.auto_fn_n4661a026', '请填写完整信息并上传图片'));
      return;
    }
    showToast(t('enterprise.auto_fn_n47e91bbb', '申请提交成功，请等待审核'));
    setTimeout(() => navigate(-1), 1000);
  };

  const handleImageUpload = (field: "logo" | "banner", e: React.ChangeEvent<HTMLInputElement>) => {
  const file = e.target.files?.[0];
    if (file) {
      const url = URL.createObjectURL(file);
      setFormData(prev => ({ ...prev, [field]: url }));
    }
  };

  return (
    <PageLayout title={t('enterprise.auto_prop_375e79da', '申请入驻')} bgClass="bg-white dark:bg-[#1a1b1c]">
      <div className="p-4 flex flex-col gap-4 max-w-full pb-8">
        <h2 className="text-[20px] font-bold text-text-main mb-2">{t('enterprise.auto_n5f14612f', '欢迎入驻企业中心')}</h2>
        
        <div className="flex flex-col gap-3">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_n768b19ae', '企业 Logo')}</label>
          <div className="flex items-center gap-3">
            {formData.logo ? (
              <div className="relative w-20 h-20 rounded-xl overflow-hidden shadow-sm shrink-0">
                <img src={formData.logo} alt="Logo" className="w-full h-full object-cover" />
                <div 
                  className="absolute top-1 right-1 bg-black/50 rounded-full p-1 cursor-pointer"
                  onClick={() => setFormData(p => ({ ...p, logo: "" }))}
                >
                  <X className="w-3 h-3 text-white" />
                </div>
              </div>
            ) : (
              <label className="w-20 h-20 bg-[#f5f6f8] dark:bg-[#2c2d2e] rounded-xl flex flex-col items-center justify-center cursor-pointer text-text-sub active:scale-95 transition-transform shrink-0">
                <ImagePlus className="w-6 h-6 mb-1" />
                <span className="text-[10px]">{t('enterprise.auto_3e676d55', '上传 Logo')}</span>
                <input type="file" accept="image/*" className="hidden" onChange={(e) => handleImageUpload("logo", e)} />
              </label>
            )}
            <p className="text-[12px] text-text-sub flex-1">{t('enterprise.auto_n1679c8e0', '建议尺寸 400x400px，支持 jpg、png 格式，不超过 2MB')}</p>
          </div>
        </div>

        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_7e3194e8', '企业宣传图')}</label>
          {formData.banner ? (
            <div className="relative w-full h-40 rounded-xl overflow-hidden shadow-sm">
              <img src={formData.banner} alt="Banner" className="w-full h-full object-cover" />
              <div 
                className="absolute top-2 right-2 bg-black/50 rounded-full p-1.5 cursor-pointer"
                onClick={() => setFormData(p => ({ ...p, banner: "" }))}
              >
                <X className="w-4 h-4 text-white" />
              </div>
            </div>
          ) : (
            <label className="w-full h-40 bg-[#f5f6f8] dark:bg-[#2c2d2e] rounded-xl flex flex-col items-center justify-center cursor-pointer text-text-sub active:scale-[0.98] transition-transform">
              <ImagePlus className="w-8 h-8 mb-2 opacity-70" />
              <span className="text-[13px] font-medium opacity-90">{t('enterprise.auto_3132fdd', '点击上传企业宣传图 (横图)')}</span>
              <span className="text-[11px] opacity-60 mt-1">{t('enterprise.auto_n31bee3d', '建议尺寸 1920x1080px')}</span>
              <input type="file" accept="image/*" className="hidden" onChange={(e) => handleImageUpload("banner", e)} />
            </label>
          )}
        </div>

        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_25199c7c', '企业名称')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n53bef396', '请输入企业全称')} value={formData.name} onChange={e => setFormData({...formData, name: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3f2d03e7', '行业领域')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n4ac33c2b', '请输入所属行业')} value={formData.industry} onChange={e => setFormData({...formData, industry: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_4a63104d', '联系人姓名')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n57cc747c', '请输入联系人姓名')} value={formData.contact} onChange={e => setFormData({...formData, contact: e.target.value})} />
        </div>
        <div className="flex flex-col gap-3 mt-2">
          <label className="text-[14px] font-medium text-text-main">{t('enterprise.auto_3c3996af', '联系电话')}</label>
          <input className="w-full bg-[#f5f6f8] dark:bg-[#2c2d2e] p-3 rounded-xl border-none outline-none text-[15px] text-text-main" placeholder={t('enterprise.auto_prop_n3c9ea768', '请输入联系电话')} type="tel" value={formData.phone} onChange={e => setFormData({...formData, phone: e.target.value})} />
        </div>
        
        <button className="w-full py-3 bg-primary-blue text-white font-medium rounded-xl text-[16px] mt-6 shadow-md shadow-blue-500/20 active:scale-95 transition-transform" onClick={handleSubmit}>{t('enterprise.auto_2e953cf8', '提交申请')}</button>
      </div>
    </PageLayout>
  );
};
