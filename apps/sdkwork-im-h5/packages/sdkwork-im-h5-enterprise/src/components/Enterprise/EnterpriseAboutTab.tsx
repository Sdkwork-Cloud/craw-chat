import { useTranslation } from "react-i18next";
import React from "react";
import { Globe, Phone, MapPin, Flag } from "lucide-react";
import { showToast, ListItem } from "@sdkwork/im-h5-commons";
import { motion } from "motion/react";

export const EnterpriseAboutTab = () => {
  const { t } = useTranslation();
return (
    <motion.div
      key="about"
      initial={{ opacity: 0, y: 5 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="flex flex-col gap-2"
    >
      <div className="bg-chat-other-bg p-5">
        <h2 className="text-[17px] font-bold text-text-main mb-3 flex items-center gap-2">
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_263967b6', 'About us')}</h2>
        <p className="text-[15px] leading-relaxed text-text-sub space-y-4">
          <span>{t('enterprise.auto_n6c27968c', 'Geek Tech Universe was founded in 2022 as a high-tech enterprise focused on applying AI technology to mobile apps. We provide full-lifecycle AI solutions from product design and development to deployment and operations.')}</span>
          <br />
          <br />
          <span>{t('enterprise.auto_3994cc0b', 'Relying on our proprietary large-model technology architecture, we uphold the philosophy of "technology changes the world," serving over a thousand enterprise customers and contributing to a more intelligent and connected world.')}</span>
        </p>
      </div>

      <div className="bg-chat-other-bg p-5">
        <h2 className="text-[17px] font-bold text-text-main mb-4 flex items-center gap-2">
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_25209d5d', 'Business qualifications')}</h2>
        <div className="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          {[1, 2, 3].map((item) => (
            <div
              key={item}
              className="shrink-0 w-36 h-28 bg-gradient-to-br from-[#f8f9fa] to-[#f1f3f5] dark:from-[#2a2b2c] dark:to-[#1a1b1c] rounded-xl border border-border-color flex items-center justify-center flex-col gap-2 shadow-sm"
            >
              <div className="w-10 h-10 rounded-full bg-orange-500/10 flex items-center justify-center">
                <Flag className="w-5 h-5 text-orange-500" />
              </div>
              <span className="text-[12px] text-text-main font-bold text-center px-2">{t('enterprise.auto_47ed6487', 'High-tech')}<br />{t('enterprise.auto_25206996', 'Enterprise certification')}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="bg-chat-other-bg pb-6">
        <h2 className="text-[17px] font-bold text-text-main mb-3 pt-5 px-5 flex items-center gap-2">
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_3c3789dd', 'Contact')}</h2>
        <div className="flex flex-col border-y border-border-color/50">
          <ListItem
            icon={Globe}
            label={t('enterprise.auto_prop_251a8bb2', 'Official website')}
            rightText="geekcosmo.com"
            onClick={() => showToast(t('enterprise.auto_fn_39cd0c2b', 'Opening website'))}
          />
          <ListItem
            icon={Phone}
            label={t('enterprise.auto_prop_375f5782', 'Phone inquiry')}
            rightText="010-8888-9999"
            onClick={() => (window.location.href = "tel:01088889999")}
          />
          <ListItem
            icon={MapPin}
            label={t('enterprise.auto_prop_2649431c', 'Company address')}
            rightText={t('enterprise.auto_prop_4fa8250b', 'Zhongguancun Geek Tower')}
            onClick={() => showToast(t('enterprise.auto_fn_750b2c0', 'Opening map navigation...'))}
            hideBorder
          />
        </div>
      </div>
    </motion.div>
  );
};
