import React from "react";
import {
  PlusCircle,
  MessageSquarePlus,
  UserPlus,
  Bot,
  Scan,
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

interface AddMenuProps {
  isMenuOpen: boolean;
  setIsMenuOpen: (isOpen: boolean) => void;
}

export const AddMenu: React.FC<AddMenuProps> = ({
  isMenuOpen,
  setIsMenuOpen,
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <>
      <IconButton
        icon={
          <PlusCircle
            className={cn(
              "w-5 h-5 transition-transform duration-200",
              isMenuOpen ? "rotate-45 text-text-main" : "text-text-main",
            )}
          />
        }
        className={cn(
          "bg-black/5 dark:bg-white/5 w-8 h-8 p-0 transition-colors",
          isMenuOpen && "bg-black/10 dark:bg-white/10",
        )}
        onClick={() => setIsMenuOpen(!isMenuOpen)}
      />

      <AnimatePresence>
        {isMenuOpen && (
          <motion.div
            initial={{
              opacity: 0,
              scale: 0.9,
              y: -10,
              transformOrigin: "top right",
            }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: -10 }}
            transition={{ duration: 0.15, ease: "easeOut" }}
            className="absolute top-[44px] right-0 w-[150px] bg-bg-color/95 backdrop-blur-xl border border-border-color rounded-xl shadow-2xl z-50 overflow-visible"
          >
            {/* Triangle pointer */}
            <div className="absolute -top-1.5 right-3 w-3 h-3 bg-bg-color border-t border-l border-border-color rotate-45 z-0" />

            <div className="relative z-10 flex flex-col rounded-xl overflow-hidden">
              <div
                className="flex items-center gap-3 px-4 py-3.5 active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
                onClick={() => {
                  setIsMenuOpen(false);
                  navigate("/create-group");
                }}
              >
                <MessageSquarePlus className="w-5 h-5 text-text-main" />
                <span className="text-[15px] font-medium text-text-main">
                  {t('chat.list.start_group')}
                </span>
              </div>
              <div
                className="flex items-center gap-3 px-4 py-3.5 active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
                onClick={() => {
                  setIsMenuOpen(false);
                  navigate("/add-friend");
                }}
              >
                <UserPlus className="w-5 h-5 text-text-main" />
                <span className="text-[15px] font-medium text-text-main">
                  {t('chat.list.add_friend')}
                </span>
              </div>
              <div
                className="flex items-center gap-3 px-4 py-3.5 active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
                onClick={() => {
                  setIsMenuOpen(false);
                  navigate("/agent/create");
                }}
              >
                <Bot className="w-5 h-5 text-text-main" />
                <span className="text-[15px] font-medium text-text-main">
                  {t('chat.list.create_agent')}
                </span>
              </div>
              <div
                className="flex items-center gap-3 px-4 py-3.5 active:bg-active-bg transition-colors cursor-pointer"
                onClick={() => {
                  setIsMenuOpen(false);
                  navigate("/scan");
                }}
              >
                <Scan className="w-5 h-5 text-text-main" />
                <span className="text-[15px] font-medium text-text-main">
                  {t('chat.list.scan')}
                </span>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
};
