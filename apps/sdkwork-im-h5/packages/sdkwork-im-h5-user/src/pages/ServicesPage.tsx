import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { Wallet, Smartphone, Zap, Umbrella, Coffee, CircleDollarSign, Package, Ticket, Crown } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { LifeService, type LifeServiceItem } from "../services/LifeService";
import { PageLayout } from "../components/PageLayout";

const ICON_MAP: Record<string, any> = {
  Smartphone: Smartphone,
  Zap: Zap,
  Umbrella: Umbrella,
  Coffee: Coffee,
  CircleDollarSign: CircleDollarSign,
  Package: Package,
  Ticket: Ticket,
  Crown: Crown,
};

export const ServicesPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [services, setServices] = useState<LifeServiceItem[]>([]);

  useEffect(() => {
    LifeService.getLifeServices().then(setServices);
  }, []);

  return (
    <PageLayout 
      title={t('user.auto_prop_ccd34', '服务')} 
      bgClass="bg-[#F3F3F3] dark:bg-black"
      rightElement={
        <span 
          className="text-[14px] font-medium text-text-main hover:opacity-70 cursor-pointer"
          onClick={() => navigate("/billing-records")}
        >{t('user.auto_4173b4d4', '账单记录')}</span>
      }
    >
      <div className="p-3 w-full">
        <div className="bg-gradient-to-br from-blue-600 to-indigo-700 dark:from-blue-900/80 dark:to-indigo-950 rounded-2xl p-6 text-white mb-3 shadow-[0_8px_30px_rgba(37,99,235,0.2)] dark:shadow-none border border-transparent dark:border-blue-800/30 relative overflow-hidden flex flex-col pointer-events-none">
          <div className="absolute top-0 right-0 p-12 bg-white/10 blur-[50px] rounded-full pointer-events-none" />
          <div className="flex items-center justify-between w-full mb-8 relative z-10">
             <div className="flex items-center gap-2 text-white/95">
                <Wallet className="w-5 h-5 text-white" strokeWidth={1.5} />
                <span className="text-[15px] font-bold tracking-wide">{t('user.auto_2e635247', '我的钱包')}</span>
             </div>
          </div>
          <div className="flex items-end justify-between w-full relative z-10">
            <div>
               <div className="text-[11px] text-white/70 mb-2 uppercase tracking-widest font-bold">{t('user.auto_n14f50176', '总资产 (¥)')}</div>
               <div className="text-[38px] font-bold leading-none tracking-tight font-mono text-white">12,500.00</div>
            </div>
            <div className="text-right">
               <div className="text-[11px] text-white/70 mb-2 uppercase tracking-widest font-bold">{t('user.auto_2c9904d1', '当前积分')}</div>
               <div className="text-[22px] font-medium leading-none tracking-tight font-mono text-white">3,200</div>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-[#1A1A1A] rounded-2xl p-5 shadow-sm border border-border-color flex flex-col">
          <h3 className="text-[15px] text-text-main mb-6 font-bold">{t('user.auto_30865237', '智能服务')}</h3>
          <div className="grid grid-cols-4 gap-y-8 pointer-events-auto">
            {services.map((item, i) => {
              const Icon = ICON_MAP[item.iconName] || Smartphone;
              return (
                <div
                  key={i}
                  className="flex flex-col items-center gap-2.5 cursor-pointer active:scale-95 transition-transform"
                  onClick={async () => {
                    const path = item.label === "Vip订阅" ? "/vip-subscription" : "/token-recharge";
                    const tabParam = item.label === "Token Plan" ? "plan" : item.label === "优惠券" ? "coupon" : "recharge";
                    navigate(path, { state: { tab: tabParam } });
                  }}
                >
                  <Icon className={cn("w-8 h-8", item.color)} strokeWidth={1.5} />
                  <span className="text-[12px] text-text-main font-medium whitespace-nowrap">
                    {item.label}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </PageLayout>
  );
};
