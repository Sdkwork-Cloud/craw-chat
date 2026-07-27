import React, { useEffect, useState } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { useNavigate, useParams } from "react-router";
import { MeetingService, MeetingRecord } from "../services/MeetingService";
import { Play } from "lucide-react";
import { useTranslation } from "react-i18next";
import { MeetingDetailInfo } from "../components/MeetingDetailInfo";
import { MeetingDetailAttendees } from "../components/MeetingDetailAttendees";

export const MeetingDetail = () => {
  const { t } = useTranslation();
  
const { id } = useParams();
  const navigate = useNavigate();
  
  const [meeting, setMeeting] = useState<MeetingRecord | null>(null);

  useEffect(() => {
    if (id) {
      MeetingService.getMeetingDetail(id).then(setMeeting);
    }
  }, [id]);

  if (!meeting)
    return (
      <PageLayout title={t('meeting.detail.title')}>
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">{t('meeting.loading')}</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title={t('meeting.detail.title')}>
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
        <MeetingDetailInfo meeting={meeting} />

        <MeetingDetailAttendees meeting={meeting} />

        {meeting.description && (
          <div className="bg-white dark:bg-[#1a1b1c] p-4 mb-2">
            <h3 className="text-[15px] font-medium text-text-main mb-3">
              {t('meeting.detail.description')}
            </h3>
            <div className="text-[14px] text-text-main leading-relaxed whitespace-pre-wrap">
              {meeting.description}
            </div>
          </div>
        )}

        {meeting.status !== "finished" && meeting.status !== "cancelled" && (
          <div className="p-6 mt-4">
            <button
              className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90 flex items-center justify-center gap-2"
              onClick={() => navigate(`/call/video/${meeting.id}`)}
            >
              <Play className="w-5 h-5" />
              {t('meeting.join')}
            </button>
          </div>
        )}
      </div>
    </PageLayout>
  );
};
