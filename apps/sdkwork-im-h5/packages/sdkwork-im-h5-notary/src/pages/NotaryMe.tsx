import React, { useState, useEffect } from "react";
import {
  ShieldCheck,
  ChevronRight,
  FileCheck,
  Award,
  PenLine,
  MessageSquareText,
  CalendarDays,
  PieChart,
  CreditCard,
  LayoutTemplate,
  ChevronLeft,
} from "lucide-react";
import { useNavigate } from "react-router";
import { IconButton } from "@sdkwork/im-h5-commons";
import { notaryService } from "../services/notaryService";
import { useTranslation } from "react-i18next";

export const NotaryMe: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [roles, setRoles] = useState<any[]>([]);

  useEffect(() => {
    notaryService.getNotaryRoles().then((data) => setRoles(data as any[]));
  }, []);

  const Cell = ({
    icon: Icon,
    label,
    colorClass = "text-text-main",
    onClick,
  }: any) => {
  const { t } = useTranslation();
  return (
    <div
      className="flex items-center gap-3 px-4 py-4 bg-bg-color active:bg-active-bg transition-colors cursor-pointer"
      onClick={onClick}
    >
      <Icon className={`w-6 h-6 ${colorClass}`} />
      <span className="text-[16px] text-text-main font-medium flex-1">
        {label}
      </span>
      <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
    </div>
  );
};


  return (
    <div className="flex flex-col h-full bg-[#f2f2f2] dark:bg-[#121212]">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe relative"></header>

      <div className="flex-1 overflow-y-auto pb-[90px]">
        {/* User Info Cell */}
        <div className="bg-bg-color p-6 flex items-center gap-5 mb-2 pt-8 pb-8 border-b border-border-color/50 relative">
          <div className="w-[72px] h-[72px] rounded-2xl bg-chat-other-bg overflow-hidden border border-border-color/50 shrink-0 shadow-sm relative">
            <img
              src="https://picsum.photos/seed/notaryUser/200/200"
              alt="avatar"
              className="w-full h-full object-cover"
            />
          </div>
          <div className="flex flex-col flex-1 min-w-0">
            <div className="flex items-center gap-2 min-w-0 mb-1">
              <h2 className="text-[22px] font-bold text-text-main truncate shrink">{t('notary.auto_177dc26', '张雨绮')}</h2>
              <div className="bg-primary-blue/10 px-1.5 py-0.5 rounded text-[10px] font-bold text-primary-blue flex items-center gap-0.5 shrink-0 whitespace-nowrap ml-1">
                <ShieldCheck className="w-[10px] h-[10px] shrink-0" /> {t('notary.me.real_name_verified')}
              </div>
            </div>

            {/* Roles Display */}
            <div className="flex flex-wrap items-center gap-1.5 mt-1.5 mb-1.5">
              {roles.map((role, idx) => (
                <div
                  key={idx}
                  className={`px-2 py-[2px] rounded-md text-[11px] font-bold flex items-center ${role.color}`}
                >
                  {role.name}
                </div>
              ))}
            </div>

            <p className="text-[13px] text-text-sub mt-0.5 truncate">{t('notary.auto_200594f7', '执业编号: NOTARY-2026-A01L')}</p>
          </div>
          <ChevronRight className="w-5 h-5 text-text-sub absolute right-4 top-1/2 -translate-y-1/2" />
        </div>

        {/* Modules */}
        <div className="bg-bg-color mb-2 border-y border-border-color/50">
          <Cell
            icon={FileCheck}
            label={t('notary.me.digital_cert')}
            colorClass="text-blue-500"
          />
          <div className="h-[1px] bg-border-color/50 ml-14" />
          <Cell icon={Award} label={t('notary.me.e_seal_management')} colorClass="text-red-500" />
          <div className="h-[1px] bg-border-color/50 ml-14" />
          <Cell
            icon={PenLine}
            label={t('notary.me.signature_fingerprint')}
            colorClass="text-indigo-500"
          />
        </div>

        <div className="bg-bg-color mb-2 border-y border-border-color/50">
          <Cell
            icon={LayoutTemplate}
            label={t('notary.me.notary_template_lib')}
            colorClass="text-orange-500"
          />
          <div className="h-[1px] bg-border-color/50 ml-14" />
          <Cell
            icon={MessageSquareText}
            label={t('notary.me.inquiry_script_lib')}
            colorClass="text-green-500"
          />
        </div>

        <div className="bg-bg-color mb-2 border-y border-border-color/50">
          <Cell
            icon={CalendarDays}
            label={t('notary.me.appointment_mgmt')}
            colorClass="text-purple-500"
          />
          <div className="h-[1px] bg-border-color/50 ml-14" />
          <Cell
            icon={PieChart}
            label={t('notary.me.performance_stats')}
            colorClass="text-cyan-500"
          />
          <div className="h-[1px] bg-border-color/50 ml-14" />
          <Cell
            icon={CreditCard}
            label={t('notary.me.fee_records')}
            colorClass="text-yellow-500"
          />
        </div>
      </div>
    </div>
  );
};
