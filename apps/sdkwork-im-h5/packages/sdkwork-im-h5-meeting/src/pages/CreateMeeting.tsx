import React, { useState } from "react";
import {
  showPrompt,
  PageLayout,
  showToast,
  cn,
} from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import {
  MeetingService,
  CreateMeetingRequest,
} from "../services/MeetingService";
import { Users } from "lucide-react";
import { useTranslation } from "react-i18next";
import { MeetingFormItem } from "../components/MeetingFormItem";

export const CreateMeeting = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [formData, setFormData] = useState<CreateMeetingRequest>({
    title: "",
    description: "",
    startTime: new Date().toISOString().slice(0, 16),
    endTime: new Date(Date.now() + 3600000).toISOString().slice(0, 16),
    roomId: "",
    attendeeIds: [],
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    if (!formData.title) return showToast(t('meeting.create.topicPlaceholder'));
    setLoading(true);
    try {
      await MeetingService.createMeeting(formData);
      showToast(t('common.success', { defaultValue: '成功' }));
      navigate(-1);
    } catch (e) {
      const error = e as Error;
      showToast(error.message || t('common.error', { defaultValue: '失败' }));
    } finally {
      setLoading(false);
    }
  };

  return (
    <PageLayout title={t('meeting.create.title')}>
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-8">
        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <MeetingFormItem label={t('meeting.create.topicLabel')} required>
            <input
              type="text"
              placeholder={t('meeting.create.topicPlaceholder')}
              className="w-full text-[16px] bg-transparent outline-none py-1"
              value={formData.title}
              onChange={(e) =>
                setFormData((s) => ({ ...s, title: e.target.value }))
              }
            />
          </MeetingFormItem>

          <MeetingFormItem label={t('meeting.create.startTime')} required>
            <input
              type="datetime-local"
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.startTime}
              onChange={(e) =>
                setFormData((s) => ({ ...s, startTime: e.target.value }))
              }
            />
          </MeetingFormItem>

          <MeetingFormItem label={t('meeting.create.endTime')} required>
            <input
              type="datetime-local"
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.endTime}
              onChange={(e) =>
                setFormData((s) => ({ ...s, endTime: e.target.value }))
              }
            />
          </MeetingFormItem>

          <MeetingFormItem label={t('meeting.create.descriptionLabel')}>
            <textarea
              placeholder={t('meeting.create.descriptionPlaceholder')}
              className="w-full text-[16px] bg-transparent outline-none py-1 min-h-[80px]"
              value={formData.description}
              onChange={(e) =>
                setFormData((s) => ({ ...s, description: e.target.value }))
              }
            />
          </MeetingFormItem>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <MeetingFormItem
            label={t('meeting.create.roomLabel')}
            onClick={async () => {
              const room = await showPrompt(
                t('meeting.create.roomPlaceholder'),
                formData.roomId,
              );
              if (room) setFormData((s) => ({ ...s, roomId: room }));
            }}
          >
            <div className="flex justify-between items-center w-full">
              <span
                className={formData.roomId ? "text-text-main" : "text-text-sub"}
              >{formData.roomId || t('common.pleaseSelect', { defaultValue: '请选择' })}</span>
            </div>
          </MeetingFormItem>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30 p-4">
          <div className="text-[15px] text-text-main font-medium mb-3">
            {t('meeting.create.attendeesLabel')}
          </div>
          <div className="flex gap-2 flex-wrap items-center">
            {formData.attendeeIds?.map((attendee, i) => (
              <div key={i} className="relative group">
                <div className="w-12 h-12 rounded-full bg-primary-blue/10 text-primary-blue flex flex-col items-center justify-center text-[10px] whitespace-nowrap overflow-hidden text-ellipsis shadow-sm ring-1 ring-primary-blue/20">
                  {attendee.slice(0, 2)}
                </div>
                <div
                  className="absolute -top-1 -right-1 bg-red-500 rounded-full w-4 h-4 flex items-center justify-center text-white cursor-pointer"
                  onClick={() =>
                    setFormData((s) => ({
                      ...s,
                      attendeeIds: s.attendeeIds?.filter(
                        (_, index) => index !== i,
                      ),
                    }))
                  }
                >
                  <span className="text-[10px] font-bold leading-none">
                    &times;
                  </span>
                </div>
              </div>
            ))}
            <div
              className="w-12 h-12 rounded-full bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color shrink-0"
              onClick={async () => {
                const name = await showPrompt(t('meeting.create.attendeesPlaceholder'));
                if (name && name.trim()) {
                  setFormData((s) => ({
                    ...s,
                    attendeeIds: [...(s.attendeeIds || []), name.trim()],
                  }));
                  showToast(t('common.addSuccess', { defaultValue: `已添加联系人: ${name}` }));
                }
              }}
            >
              <Users className="w-5 h-5 text-text-sub" />
            </div>
          </div>
        </div>

        <div className="p-6 mt-8">
          <button
            className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90"
            onClick={handleSubmit}
            disabled={loading}
          >
            {loading ? t('meeting.loading') : t('meeting.create.submit')}
          </button>
        </div>
      </div>
    </PageLayout>
  );
};
