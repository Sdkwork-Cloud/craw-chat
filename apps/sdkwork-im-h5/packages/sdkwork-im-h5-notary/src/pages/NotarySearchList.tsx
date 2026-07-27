import React, { useState, useMemo, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, Check, FileCheck } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { notaryService } from "../services/notaryService";
import { useTranslation } from "react-i18next";

export const NotarySelectionParams = {
  selectedId: "",
  selectedNotaryObj: null as any,
  onSelect: (id: string, obj: any) => {},
};

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#".split("");

export const NotarySearchList: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [searchQuery, setSearchQuery] = useState("");
  const [notaries, setNotaries] = useState<any[]>([]);

  useEffect(() => {
    notaryService
      .getNotarySearchList()
      .then((data) => setNotaries(data as any[]));
  }, []);

  const filteredNotaries = useMemo(() => {
    return notaries.filter(
      (n) =>
        n.name.includes(searchQuery) ||
        n.org.includes(searchQuery) ||
        n.loc.includes(searchQuery),
    );
  }, [searchQuery, notaries]);

  const groupedNotaries = useMemo(() => {
    const groups: Record<string, typeof notaries> = {};
    filteredNotaries.forEach((n) => {
      const groupKey = n.initial || "#";
      if (!groups[groupKey]) groups[groupKey] = [];
      groups[groupKey].push(n);
    });
    // sort keys
    return Object.keys(groups)
      .sort()
      .map((key) => ({
        initial: key,
        notaries: groups[key],
      }));
  }, [filteredNotaries]);

  const handleSelect = (notary: any) => {
  if (NotarySelectionParams.onSelect) {
      NotarySelectionParams.onSelect(notary.id, notary);
    }
    navigate(-1);
  };

  const scrollToSection = (letter: string) => {
  const el = document.getElementById(`section-${letter}`);
    if (el) {
      el.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color fixed inset-0 z-[100] animate-in slide-in-from-right">
      <header className="h-[44px] flex items-center justify-between glass-header sticky top-0 shrink-0 pt-safe px-1 z-10">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-text-main text-[17px] pointer-events-none">
          {t("notary.search.title")}
        </div>
        <div className="flex justify-end z-10 flex-1 pr-4"></div>
      </header>

      <div className="px-4 py-3 bg-bg-color shrink-0 z-10">
        <div className="flex items-center bg-input-bg rounded-xl px-3 py-2 text-[15px]">
          <Search className="w-5 h-5 text-text-sub mr-2 shrink-0" />
          <input
            type="text"
            placeholder={t("notary.search.placeholder")}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="flex-1 bg-transparent border-none outline-none text-text-main placeholder-text-sub"
          />
        </div>
      </div>

      <div className="flex-1 overflow-y-auto relative bg-[#F4F6F9] dark:bg-black">
        {groupedNotaries.map((group) => (
          <div key={group.initial} id={`section-${group.initial}`}>
            <div className="px-4 py-1.5 text-[13px] font-bold text-text-sub bg-[#F4F6F9] dark:bg-black sticky top-0 z-10">
              {group.initial}
            </div>
            <div className="bg-chat-other-bg">
              {group.notaries.map((notary, idx) => (
                <div
                  key={notary.id}
                  onClick={() => handleSelect(notary)}
                  className={cn(
                    "flex items-center pl-4 py-3 active:bg-active-bg transition-colors cursor-pointer",
                    NotarySelectionParams.selectedId === notary.id
                      ? "bg-primary-blue/5"
                      : "",
                  )}
                >
                  <div className="w-10 h-10 bg-gradient-to-tr from-primary-blue to-blue-400 rounded-md flex items-center justify-center text-white mr-3 shrink-0 shadow-sm relative">
                    <FileCheck className="w-5 h-5" />
                    {notary.active && (
                      <div className="absolute -bottom-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-bg-color" />
                    )}
                  </div>
                  <div
                    className={cn(
                      "flex-1 min-w-0 pr-4 flex items-center justify-between pb-3",
                      idx !== group.notaries.length - 1
                        ? "border-b border-border-color"
                        : "",
                    )}
                  >
                    <div className="flex flex-col flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-0.5">
                        <span className="text-[16px] font-medium text-text-main truncate">
                          {notary.name}
                        </span>
                      </div>
                      <div className="text-[13px] text-text-sub truncate">
                        {notary.org}
                      </div>
                    </div>
                    {NotarySelectionParams.selectedId === notary.id && (
                      <Check className="w-5 h-5 text-primary-blue shrink-0 ml-2" />
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}

        {filteredNotaries.length === 0 && (
          <div className="py-20 flex flex-col items-center justify-center text-text-sub bg-chat-other-bg h-full">
            <Search className="w-12 h-12 opacity-20 mb-3" />
            <p>{t("notary.search.not_found")}</p>
          </div>
        )}
      </div>

      {/* Alphabet Index */}
      {!searchQuery && (
        <div className="absolute right-1 top-1/2 -translate-y-1/2 flex flex-col items-center justify-center p-1 z-20 touch-none">
          {ALPHABET.map((letter) => (
            <div
              key={letter}
              onClick={() => scrollToSection(letter)}
              className="text-[10px] font-medium text-text-sub hover:text-primary-blue py-[1.5px] cursor-pointer"
            >
              {letter}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
