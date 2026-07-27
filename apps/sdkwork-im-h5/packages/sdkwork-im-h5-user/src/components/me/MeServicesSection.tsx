import React from "react";
import { Wallet, Package } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { MenuItem } from "../MenuItem";

export const MeServicesSection: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
      <MenuItem
        icon={Wallet}
        label={t('user.auto_prop_ccd34', '服务')}
        colorClass="text-blue-500"
        onClick={() => navigate("/me/services")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Package}
        label={t('user.auto_prop_40bbe269', '订单中心')}
        colorClass="text-orange-500"
        onClick={() => navigate("/me/orders")}
      />
    </div>
  );
};
