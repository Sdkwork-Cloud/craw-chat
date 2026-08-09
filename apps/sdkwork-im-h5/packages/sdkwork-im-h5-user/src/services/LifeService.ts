import { useTranslation } from "react-i18next";
import i18next from 'i18next';
const t = i18next.t.bind(i18next);
import { CircleDollarSign, Package, Ticket, Crown } from "lucide-react";

export interface LifeServiceItem {
  iconName: string;
  label: string;
  color: string;
}

const MOCK_LIFE_SERVICES: LifeServiceItem[] = [
  { iconName: "CircleDollarSign", label: t("user:life.token_recharge", "Token充值"), color: "text-slate-700 dark:text-slate-300" },
  { iconName: "Banknote", label: t("user:life.withdraw", "提现"), color: "text-slate-700 dark:text-slate-300" },
  { iconName: "Ticket", label: t("user:life.vouchers", "优惠券"), color: "text-slate-700 dark:text-slate-300" },
  { iconName: "Crown", label: t("user:life.vip_subscription", "Vip订阅"), color: "text-amber-500" },
];

export const LifeService = {
  getLifeServices: async (): Promise<LifeServiceItem[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...MOCK_LIFE_SERVICES]), 100),
    );
  },
};
