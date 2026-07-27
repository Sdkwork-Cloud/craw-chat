import React from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, Plus, X } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface CommunityHeaderProps {
  isSearching: boolean;
  setIsSearching: (val: boolean) => void;
  searchText: string;
  setSearchText: (val: string) => void;
  isPlusMenuOpen: boolean;
  setIsPlusMenuOpen: (val: boolean) => void;
}

export const CommunityHeader: React.FC<CommunityHeaderProps> = ({
  isSearching,
  setIsSearching,
  searchText,
  setSearchText,
  isPlusMenuOpen,
  setIsPlusMenuOpen,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-white dark:bg-[#1E1E1E]">
      {isSearching ? (
        <div className="flex items-center w-full gap-2 transition-all">
          <div className="flex-1 bg-black/5 dark:bg-white/10 h-9 rounded-full flex items-center px-3 gap-2">
            <Search className="w-4 h-4 text-text-sub" />
            <input
              autoFocus
              type="text"
              placeholder={t("community.auto_prop_340eaf60", "搜索圈子...")}
              className="flex-1 bg-transparent border-none outline-none text-[15px] text-text-main"
              value={searchText}
              onChange={(e) => setSearchText(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.currentTarget.blur();
                }
              }}
            />
            {searchText && (
              <IconButton
                icon={<X className="w-4 h-4 text-text-sub" />}
                className="w-6 h-6 p-0 bg-transparent shrink-0"
                onClick={() => setSearchText("")}
              />
            )}
          </div>
          <button
            className="text-[15px] text-text-main shrink-0 whitespace-nowrap active:opacity-50 px-2"
            onClick={() => {
              setIsSearching(false);
              setSearchText("");
            }}
          >
            {t("community.auto_a9472", "取消")}
          </button>
        </div>
      ) : (
        <>
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            className="bg-transparent w-10 h-10 -ml-2"
            onClick={() => navigate(-1)}
          />
          <h1 className="text-[17px] font-semibold text-text-main">
            {t("community.auto_28f5e16e", "圈子社群")}
          </h1>
          <div className="flex items-center -mr-2 relative">
            <IconButton
              icon={<Search className="w-5 h-5 text-text-main" />}
              className="bg-transparent w-10 h-10"
              onClick={() => setIsSearching(true)}
            />
            <IconButton
              icon={<Plus className="w-6 h-6 text-text-main" />}
              className="bg-transparent w-10 h-10"
              onClick={() => setIsPlusMenuOpen(!isPlusMenuOpen)}
            />

            {isPlusMenuOpen && (
              <>
                <div
                  className="fixed inset-0 z-40"
                  onClick={() => setIsPlusMenuOpen(false)}
                />
                <div className="absolute right-2 top-12 z-50 w-40 bg-white dark:bg-[#2C2C2E] rounded-xl shadow-xl border border-black/5 dark:border-white/5 py-1 animate-in zoom-in-95 duration-200 origin-top-right">
                  <div
                    className="px-4 py-3 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={() => {
                      setIsPlusMenuOpen(false);
                      navigate("/community/create");
                    }}
                  >
                    {t("community.auto_26c221c7", "创建圈子")}
                  </div>
                  <div
                    className="px-4 py-3 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={() => {
                      setIsPlusMenuOpen(false);
                      navigate("/me/communities?tab=created");
                    }}
                  >
                    {t("community.auto_9e75ebc", "我创建的圈子")}
                  </div>
                  <div
                    className="px-4 py-3 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={() => {
                      setIsPlusMenuOpen(false);
                      navigate("/me/communities?tab=joined");
                    }}
                  >
                    {t("community.auto_b0d0676", "我加入的圈子")}
                  </div>
                </div>
              </>
            )}
          </div>
        </>
      )}
    </header>
  );
};
