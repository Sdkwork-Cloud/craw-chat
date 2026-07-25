import React, { useState, useEffect } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { Plus, Video } from "lucide-react";
import { MeetingService, MeetingRecord } from "../services/MeetingService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { MeetingHeader } from "../components/MeetingHeader";
import { MeetingTabs } from "../components/MeetingTabs";
import { MeetingItemCard } from "../components/MeetingItemCard";

export const MeetingApp = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [activeTab, setActiveTab] = useState("upcoming");
  const [meetings, setMeetings] = useState<MeetingRecord[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    MeetingService.getMeetings().then((data) => {
      setMeetings(data);
      setIsLoading(false);
    });
  }, []);

  const filteredMeetings = meetings.filter((m) =>
    activeTab === "upcoming"
      ? m.status === "upcoming" || m.status === "ongoing"
      : m.status === "finished",
  );

  const upcomingCount = meetings.filter(
    (m) => m.status === "upcoming" || m.status === "ongoing",
  ).length;

  return (
    <PageLayout title={t('meeting.title')}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <MeetingHeader count={upcomingCount} />

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <MeetingTabs activeTab={activeTab} setActiveTab={setActiveTab} />

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">{t('meeting.loading')}</span>
              </div>
            ) : filteredMeetings.length > 0 ? (
              filteredMeetings.map((meeting) => (
                <MeetingItemCard key={meeting.id} meeting={meeting} />
              ))
            ) : (
              <div className="flex flex-col items-center py-20 text-text-sub opacity-70">
                <Video className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('meeting.empty')}</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/meeting/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
