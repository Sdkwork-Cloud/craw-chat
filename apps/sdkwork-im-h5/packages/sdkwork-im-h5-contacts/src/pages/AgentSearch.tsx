import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  Search,
  X,
  Bot,
  Code,
  PenTool,
  Image as ImageIcon,
  BrainCircuit,
  Flame,
} from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { AgentService, type Agent } from "../services/AgentService";
import { useTranslation } from "react-i18next";

const ICON_MAP: Record<string, any> = {
  Bot: Bot,
  PenTool: PenTool,
  BrainCircuit: BrainCircuit,
  Code: Code,
  ImageIcon: ImageIcon,
};

export const AgentSearch: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const [allAgents, setAllAgents] = useState<Agent[]>([]);
  const [hotSearches, setHotSearches] = useState<string[]>([]);

  useEffect(() => {
    // Auto-focus input on mount
    inputRef.current?.focus();
    AgentService.getAgents().then(setAllAgents);
    AgentService.getHotSearches().then(setHotSearches);
  }, []);

  const filteredAgents = query
    ? allAgents.filter(
        (a) =>
          a.name.toLowerCase().includes(query.toLowerCase()) ||
          a.desc.toLowerCase().includes(query.toLowerCase()),
      )
    : [];

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[56px] flex items-center px-3 glass-header sticky top-0 z-10 shrink-0 pt-safe gap-3">
        <div className="flex-1 flex items-center bg-chat-other-bg rounded-full h-9 px-3 border border-border-color transition-colors focus-within:border-primary-blue focus-within:bg-bg-color">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={t('contacts.search_agents')}
            className="flex-1 bg-transparent border-none outline-none px-2 text-[15px] text-text-main placeholder:text-text-sub min-w-0"
          />
          {query && (
            <div
              onClick={() => setQuery("")}
              className="p-1 cursor-pointer shrink-0"
            >
              <X className="w-3.5 h-3.5 text-white bg-black/20 dark:bg-white/20 rounded-full p-0.5" />
            </div>
          )}
        </div>
        <button
          onClick={() => navigate(-1)}
          className="text-[16px] text-text-main font-medium whitespace-nowrap shrink-0 active:opacity-70"
        >
          {t('common.cancel')}
        </button>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {!query ? (
          <div className="p-4">
            <div className="flex items-center gap-1.5 mb-4">
              <Flame className="w-4 h-4 text-accent-red" />
              <h3 className="text-[14px] font-bold text-text-main">{t('contacts.hot_search')}</h3>
            </div>
            <div className="flex flex-wrap gap-2.5">
              {hotSearches.map((tag) => (
                <span
                  key={tag}
                  onClick={() => setQuery(tag)}
                  className="px-3 py-1.5 bg-chat-other-bg border border-border-color rounded-full text-[13px] text-text-main cursor-pointer active:bg-active-bg transition-colors"
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        ) : (
          <div className="py-2">
            {filteredAgents.length > 0 ? (
              filteredAgents.map((agent) => {
                const Icon = ICON_MAP[agent.iconName] || Bot;
                return (
                  <div
                    key={agent.id}
                    onClick={() => navigate(`/chat/${agent.id}`)}
                    className="flex items-start pl-4 py-3 bg-chat-other-bg hover:bg-hover-bg active:bg-active-bg transition-colors cursor-pointer"
                  >
                    <div className="relative shrink-0 mr-3">
                      {agent.avatar ? (
                        <img
                          src={agent.avatar}
                          alt={agent.name}
                          className="w-10 h-10 rounded-full object-cover border border-border-color/50"
                        />
                      ) : (
                        <div
                          className={cn(
                            "w-10 h-10 rounded-full flex items-center justify-center",
                            agent.color || "bg-blue-500 text-white",
                          )}
                        >
                          <Icon className="w-5 h-5" />
                        </div>
                      )}
                    </div>
                    <div className="flex-1 min-w-0 pr-4 border-b border-border-color pb-3">
                      <div className="flex items-center mb-0.5">
                        <h3 className="text-[15px] font-bold text-text-main truncate">
                          {agent.name}
                        </h3>
                        {agent.isOfficial && (
                          <span className="px-1.5 py-0.5 rounded bg-[#E8F3FF] dark:bg-blue-500/20 text-[#1664FF] dark:text-blue-400 text-[9px] font-medium shrink-0 ml-1.5">
                            {t('contacts.official')}
                          </span>
                        )}
                      </div>
                      <p className="text-[13px] text-text-sub truncate">
                        {agent.desc}
                      </p>
                    </div>
                  </div>
                );
              })
            ) : (
              <div className="p-10 flex flex-col items-center justify-center text-text-sub">
                <Search className="w-10 h-10 mb-3 opacity-20" />
                <p className="text-[14px]">{t('contacts.no_agents_found')}</p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};
