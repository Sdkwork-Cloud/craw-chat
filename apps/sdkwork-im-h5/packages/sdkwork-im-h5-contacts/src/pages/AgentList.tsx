import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  Search,
  Plus,
  Bot,
  Code,
  PenTool,
  Image as ImageIcon,
  BrainCircuit,
} from "lucide-react";
import { IconButton, cn, Tabs } from "@sdkwork/im-h5-commons";
import { motion } from "motion/react";
import { AgentService, type Agent } from "../services/AgentService";
import { useTranslation } from "react-i18next";

const ICON_MAP: Record<string, any> = {
  Bot: Bot,
  PenTool: PenTool,
  BrainCircuit: BrainCircuit,
  Code: Code,
  ImageIcon: ImageIcon,
};

export const AgentList: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("");
  const [categories, setCategories] = useState<string[]>([]);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const loadData = async () => {
      setIsLoading(true);
      const cats = await AgentService.getCategories();
      setCategories(cats);
      if (cats.length > 0) setActiveTab(cats[0]);

      const data = await AgentService.getAgents();
      setAgents(data);
      setIsLoading(false);
    };
    loadData();
  }, []);

  const handleTabClick = (
    tab: string,
    event: React.MouseEvent<HTMLDivElement>,
  ) => {
  setActiveTab(tab);
    const container = scrollContainerRef.current;
    const element = event.currentTarget;
    if (container && element) {
      const containerWidth = container.offsetWidth;
      const elementOffset = element.offsetLeft;
      const elementWidth = element.offsetWidth;

      // Calculate the scroll position to center the clicked tab
      const scrollPos = elementOffset - containerWidth / 2 + elementWidth / 2;

      container.scrollTo({
        left: scrollPos,
        behavior: "smooth",
      });
    }
  };

  const AgentCard: React.FC<{ agent: Agent }> = ({ agent }) => {
  const { t } = useTranslation();
  
    const Icon = ICON_MAP[agent.iconName] || Bot;
    return (
      <div
        onClick={() => navigate(`/chat/${agent.id}`)}
        className="flex items-start pl-4 bg-chat-other-bg hover:bg-hover-bg active:bg-active-bg transition-colors cursor-pointer"
      >
        {/* Avatar / Icon */}
        <div className="relative shrink-0 mr-3 py-4">
          {agent.avatar ? (
            <img
              src={agent.avatar}
              alt={agent.name}
              className="w-[52px] h-[52px] rounded-full object-cover border border-border-color/50"
            />
          ) : (
            <div
              className={cn(
                "w-[52px] h-[52px] rounded-full flex items-center justify-center",
                agent.color,
              )}
            >
              <Icon className="w-7 h-7" />
            </div>
          )}
        </div>

        {/* Content & Action Wrapper */}
        <div className="flex-1 flex items-start pr-4 py-4 border-b border-border-color min-w-0">
          {/* Content */}
          <div className="flex-1 min-w-0 pr-2">
            <div className="flex items-center mb-1">
              <h3 className="text-[16px] font-bold text-text-main truncate">
                {agent.name}
              </h3>
              {agent.isOfficial && (
                <span className="px-1.5 py-0.5 rounded bg-[#E8F3FF] dark:bg-blue-500/20 text-[#1664FF] dark:text-blue-400 text-[10px] font-medium shrink-0 ml-1.5">
                  {t('contacts.official')}
                </span>
              )}
            </div>
            <p className="text-[14px] text-text-main leading-[1.4] line-clamp-2 mb-1.5">
              {agent.desc}
            </p>
            <p className="text-[12px] text-text-sub truncate flex items-center gap-1">
              <svg
                className="w-3.5 h-3.5 opacity-70"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
              </svg>
              {agent.users} · {agent.author}
            </p>
          </div>

          {/* Action Button */}
          <div className="shrink-0 ml-2 flex items-center h-[52px]">
            {agent.id === "a3" ? (
              <div className="w-7 h-7 rounded-full bg-black/5 dark:bg-white/10 flex items-center justify-center">
                <div className="w-3 h-3 border-b-2 border-r-2 border-text-sub rotate-45 -translate-y-0.5" />
              </div>
            ) : (
              <div className="w-7 h-7 rounded-full bg-[#1664FF] flex items-center justify-center text-white">
                <Plus className="w-5 h-5" strokeWidth={2.5} />
              </div>
            )}
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header with Tabs */}
      <header className="bg-bg-color sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center relative border-b border-border-color/50">
           <Tabs
             tabs={categories.map(c => ({ id: c, name: c }))}
             activeTab={activeTab}
             onChange={setActiveTab}
             className="px-4 pr-16 gap-6"
             itemClassName="py-3 text-[15px] font-medium text-text-sub"
             activeItemClassName="font-semibold text-text-main"
           />

          {/* Right Search Icon with Fade */}
          <div className="absolute right-0 top-0 bottom-0 flex items-center justify-end w-20 bg-gradient-to-l from-bg-color via-bg-color to-transparent pr-4 pointer-events-none">
            <div
              className="pointer-events-auto flex items-center justify-center w-8 h-8 rounded-full active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
              onClick={() => navigate("/agent-search")}
            >
              <Search className="w-5 h-5 text-text-main" strokeWidth={2.5} />
            </div>
          </div>
        </div>
      </header>

      {/* List Content */}
      <div className="flex-1 overflow-y-auto pb-[84px]">
        <motion.div
          key={activeTab}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.2 }}
        >
          {isLoading ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
              <span className="text-[14px]">{t('common.loading')}</span>
            </div>
          ) : (
            agents.map((agent) => (
              <AgentCard key={agent.id} agent={agent} />
            ))
          )}
        </motion.div>
      </div>

      {/* Floating Create Button */}
      <div className="absolute bottom-[calc(68px+env(safe-area-inset-bottom))] left-1/2 -translate-x-1/2 z-20">
        <div
          onClick={() => navigate("/agent/create")}
          className="flex items-center gap-1.5 bg-[#1664FF] text-white px-5 py-3 rounded-full shadow-lg shadow-blue-500/20 cursor-pointer active:scale-95 transition-transform"
        >
          <Plus className="w-5 h-5" strokeWidth={2.5} />
          <span className="text-[15px] font-medium">{t('contacts.create_agent')}</span>
        </div>
      </div>
    </div>
  );
};
