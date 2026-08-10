import React from "react";
import { useTranslation } from "react-i18next";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { Search, Plus, Languages, MessageSquare, UserPlus, Contact } from "lucide-react";

interface WorkspaceHeaderProps {
  showMenu: boolean;
  setShowMenu: (val: boolean) => void;
  toggleLanguage: () => void;
}

export const WorkspaceHeader: React.FC<WorkspaceHeaderProps> = ({ 
  showMenu, 
  setShowMenu, 
  toggleLanguage 
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-50 shrink-0 pt-safe bg-bg-color/80 backdrop-blur-xl border-b border-border-color/50">
      <div className="absolute inset-x-0 flex items-center justify-center pointer-events-none">
        <h1 className="text-[18px] font-bold text-text-main tracking-tight">{t("workspace.title")}</h1>
      </div>
      <div className="flex-1 flex gap-2 relative z-10">
        <IconButton
          icon={<Languages className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={toggleLanguage}
        />
      </div>
      <div className="flex gap-2 relative z-10">
        <IconButton
          icon={<Search className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={() => navigate("/search")}
        />
        <IconButton
          icon={<Contact className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={() => navigate("/workspace/contacts")}
        />
        <IconButton
          icon={<Plus className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={() => setShowMenu(!showMenu)}
        />
        {showMenu && (
          <>
            <div
              className="fixed inset-0 z-40"
              onClick={() => setShowMenu(false)}
            />
            <div className="absolute top-12 right-0 bg-[#4C4C4C] rounded-lg w-36 shadow-lg z-50 overflow-hidden text-white">
              <div
                className="px-4 py-3 border-b border-black/20 flex items-center gap-3 active:bg-black/20 cursor-pointer"
                onClick={() => {
                  setShowMenu(false);
                  navigate("/create-group");
                }}
              >
                <MessageSquare className="w-5 h-5" />
                <span className="text-[15px]">{t("workspace.create_group")}</span>
              </div>
              <div
                className="px-4 py-3 flex items-center gap-3 active:bg-black/20 cursor-pointer"
                onClick={() => {
                  setShowMenu(false);
                  navigate("/add-friend");
                }}
              >
                <UserPlus className="w-5 h-5" />
                <span className="text-[15px]">{t("workspace.add_friend")}</span>
              </div>
            </div>
          </>
        )}
      </div>
    </header>
  );
};
