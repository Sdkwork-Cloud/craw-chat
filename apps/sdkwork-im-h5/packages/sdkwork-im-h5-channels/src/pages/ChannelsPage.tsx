import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { X, Loader2 } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { VerticalFeed } from "../components/VerticalFeed";
import { WaterfallFeed } from "../components/WaterfallFeed";
import { PromptsTab } from "../components/PromptsTab";
import { MeTab } from "../components/MeTab";
import { BottomTabbar } from "../components/BottomTabbar";
import { RemixActionSheet } from "../components/RemixActionSheet";
import { WorkDetailModal } from "../components/WorkDetailModal";
import { ChannelService } from "../services/ChannelService";
import { CreativeWork } from "../types";

export const ChannelsPage: React.FC = () => {
  const { t } = useTranslation();
const [activeBottomTab, setActiveBottomTab] = useState<"home" | "explore" | "prompts" | "me">("home");
  const [feedTab, setFeedTab] = useState<"关注" | "朋友" | "推荐">("推荐");
  const [remixWork, setRemixWork] = useState<CreativeWork | null>(null);
  const [selectedWork, setSelectedWork] = useState<CreativeWork | null>(null);
  const [works, setWorks] = useState<CreativeWork[]>([]);
  const [waterfallWorks, setWaterfallWorks] = useState<CreativeWork[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);
  
  const loadData = async () => {
    setLoading(true);
    const [feedData, waterfallData] = await Promise.all([
      ChannelService.getFeedWorks(),
      ChannelService.getWaterfallWorks()
    ]);
    setWorks(feedData);
    setWaterfallWorks(waterfallData);
    setLoading(false);
  };

  return (
    <div className="flex flex-col h-full bg-black text-white relative overflow-hidden">
      {/* Dynamic Content Area (Underneath the absolute tabbar) */}
      <div className="flex-1 w-full h-full relative">
        {loading ? (
           <div className="w-full h-full flex items-center justify-center">
              <Loader2 className="w-8 h-8 text-white/50 animate-spin" />
           </div>
        ) : activeBottomTab === "home" ? (
          <VerticalFeed 
            feedTab={feedTab} 
            setFeedTab={setFeedTab} 
            onRemix={setRemixWork}
            works={works}
          />
        ) : activeBottomTab === "explore" ? (
          <WaterfallFeed works={waterfallWorks} onWorkClick={setSelectedWork} />
        ) : activeBottomTab === "prompts" ? (
          <PromptsTab />
        ) : (
          <MeTab works={works} />
        )}
      </div>

      {/* Back Button (Global overlay) */}
      <div className="absolute top-safe left-4 z-[40] mt-3 pointer-events-auto">
        <IconButton
          icon={<X className="w-6 h-6 text-white drop-shadow-md" />}
          className="w-10 h-10 bg-black/20 backdrop-blur-sm border border-white/10 hover:bg-black/40 transition-colors"
          onClick={() => window.history.back()}
        />
      </div>

      <BottomTabbar 
        activeBottomTab={activeBottomTab} 
        setActiveBottomTab={setActiveBottomTab} 
      />
      
      <RemixActionSheet remixWork={remixWork} setRemixWork={setRemixWork} />
      <WorkDetailModal work={selectedWork} onClose={() => setSelectedWork(null)} onRemix={setRemixWork} />
    </div>
  );
};
