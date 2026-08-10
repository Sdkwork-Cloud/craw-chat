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
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_263967b6', '关于我们')}</h2>
        <p className="text-[15px] leading-relaxed text-text-sub space-y-4">
          <span>{t('enterprise.auto_n6c27968c', '极客科技宇宙创立于2022年，是一家专注于人工智能技术在移动端应用的高新技术企业。我们致力于为客户提供从产品设计、研发到部署运维的全生命周期AI解决方案。')}</span>
          <br />
          <br />
          <span>{t('enterprise.auto_3994cc0b', '依托自研的大模型技术架构，我们秉持着"技术改变世界"的理念，服务了超过上千家的企业级客户，为构建更加智能互联的环境贡献力量。')}</span>
        </p>
      </div>

      <div className="bg-chat-other-bg p-5">
        <h2 className="text-[17px] font-bold text-text-main mb-4 flex items-center gap-2">
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_25209d5d', '企业资质')}</h2>
        <div className="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          {[1, 2, 3].map((item) => (
            <div
              key={item}
              className="shrink-0 w-36 h-28 bg-gradient-to-br from-[#f8f9fa] to-[#f1f3f5] dark:from-[#2a2b2c] dark:to-[#1a1b1c] rounded-xl border border-border-color flex items-center justify-center flex-col gap-2 shadow-sm"
            >
              <div className="w-10 h-10 rounded-full bg-orange-500/10 flex items-center justify-center">
                <Flag className="w-5 h-5 text-orange-500" />
              </div>
              <span className="text-[12px] text-text-main font-bold text-center px-2">{t('enterprise.auto_47ed6487', '高新技术')}<br />{t('enterprise.auto_25206996', '企业认证')}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="bg-chat-other-bg pb-6">
        <h2 className="text-[17px] font-bold text-text-main mb-3 pt-5 px-5 flex items-center gap-2">
          <span className="w-1 h-4 bg-primary-blue rounded-full"></span>{t('enterprise.auto_3c3789dd', '联系方式')}</h2>
        <div className="flex flex-col border-y border-border-color/50">
          <ListItem
            icon={Globe}
            label={t('enterprise.auto_prop_251a8bb2', '企业官网')}
            rightText="geekcosmo.com"
            onClick={() => showToast(t('enterprise.auto_fn_39cd0c2b', '正在打开官网'))}
          />
          <ListItem
            icon={Phone}
            label={t('enterprise.auto_prop_375f5782', '电话咨询')}
            rightText="010-8888-9999"
            onClick={() => (window.location.href = "tel:01088889999")}
          />
          <ListItem
            icon={MapPin}
            label={t('enterprise.auto_prop_2649431c', '公司地址')}
            rightText={t('enterprise.auto_prop_4fa8250b', '中关村极客大厦')}
            onClick={() => showToast(t('enterprise.auto_fn_750b2c0', '正在打开地图导航...'))}
            hideBorder
          />
        </div>
      </div>
    </motion.div>
  );
};
