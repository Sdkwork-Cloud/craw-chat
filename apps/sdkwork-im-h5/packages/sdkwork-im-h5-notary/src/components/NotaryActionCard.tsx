import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { useNavigate } from "react-router";

interface NotaryActionCardProps {
  icon: React.ElementType;
  title: string;
  desc: string;
  color?: string;
  onClick?: () => void;
}

export const NotaryActionCard: React.FC<NotaryActionCardProps> = ({
  icon: Icon,
  title,
  desc,
  color,
  onClick
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <motion.div
      onClick={onClick || (() => navigate("/notary/create"))}
      whileTap={{ scale: 0.98 }}
      className="bg-chat-other-bg rounded-2xl p-4 shadow-sm border border-border-color flex items-center gap-4 cursor-pointer"
    >
      <div
        className={`w-12 h-12 rounded-full flex items-center justify-center bg-black/5 dark:bg-white/5`}
      >
        <Icon className={`w-6 h-6 ${color}`} />
      </div>
      <div className="flex-1">
        <h3 className="text-[16px] font-bold text-text-main">{title}</h3>
        <p className="text-[13px] text-text-sub mt-0.5">{desc}</p>
      </div>
    </motion.div>
  );
};
