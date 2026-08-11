import { useTranslation } from "react-i18next";
import React from "react";
import { Plus } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface BottomTabbarProps {
  activeBottomTab: "home" | "explore" | "prompts" | "me";
  setActiveBottomTab: (t: "home" | "explore" | "prompts" | "me") => void;
}

export const BottomTabbar: React.FC<BottomTabbarProps> = ({ activeBottomTab, setActiveBottomTab }) => {
  const { t } = useTranslation();
return (
    <div className="h-[52px] bg-black border-t border-white/5 flex items-center justify-around px-2 pb-safe absolute bottom-0 left-0 right-0 z-[45]">
      <TabItem 
         label={t('channels.auto_prop_13319f', 'Home')} 
         active={activeBottomTab === "home"} 
         onClick={() => setActiveBottomTab("home")} 
      />
      <TabItem 
         label={t('channels.auto_prop_a99ff', 'Discover')} 
         active={activeBottomTab === "explore"} 
         onClick={() => setActiveBottomTab("explore")} 
      />
      <div className="px-3 flex items-center justify-center cursor-pointer active:scale-95 transition-transform relative z-50">
        <div className="w-[42px] h-[28px] bg-white rounded-[8px] flex items-center justify-center">
          <Plus className="w-[18px] h-[18px] text-black" strokeWidth={4} />
        </div>
      </div>
      <TabItem 
         label={t('channels.auto_prop_185e9a3', 'Prompts')} 
         active={activeBottomTab === "prompts"} 
         onClick={() => setActiveBottomTab("prompts")} 
      />
      <TabItem 
         label={t('channels.auto_prop_6211', 'Me')} 
         active={activeBottomTab === "me"} 
         onClick={() => setActiveBottomTab("me")} 
      />
    </div>
  );
};

const TabItem = ({ label, active, onClick }: { label: string, active: boolean, onClick: () => void }) => (
  <div 
    onClick={onClick} 
    className="flex flex-col items-center justify-center px-4 py-1 cursor-pointer active:opacity-50 transition-opacity"
  >
    <span className={cn(
      "text-[15px] font-medium transition-colors",
      active ? "text-white" : "text-white/60"
    )}>
      {label}
    </span>
  </div>
);
