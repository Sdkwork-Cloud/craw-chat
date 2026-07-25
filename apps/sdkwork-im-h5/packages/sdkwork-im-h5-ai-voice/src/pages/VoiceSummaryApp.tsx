import React, { useState, useEffect } from "react";
import {
  PageLayout,
  IconButton,
  showToast,
} from "@sdkwork/im-h5-commons";
import { Search, FileAudio } from "lucide-react";
import {
  VoiceSummaryService,
  VoiceSummaryRecord,
} from "../services/VoiceSummaryService";
import { useTranslation } from "react-i18next";
import { VoiceSummaryStats } from "../components/VoiceSummaryStats";
import { VoiceSummaryItem } from "../components/VoiceSummaryItem";
import { FloatingRecordButton } from "../components/FloatingRecordButton";

export const VoiceSummaryApp = () => {
  const { t } = useTranslation();
const [summaries, setSummaries] = useState<VoiceSummaryRecord[]>([]);
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [isRecording, setIsRecording] = useState(false);
  const [showSearch, setShowSearch] = useState(false);
  const [searchWord, setSearchWord] = useState("");

  useEffect(() => {
    VoiceSummaryService.getSummaries().then(setSummaries);
  }, []);

  const handlePlayToggle = (e: React.MouseEvent, id: string) => {
  e.stopPropagation();
    if (playingId === id) setPlayingId(null);
    else setPlayingId(id);
  };

  const filteredSummaries = summaries.filter(
    (s) =>
      s.title.includes(searchWord) ||
      s.summary.includes(searchWord) ||
      s.keywords.some((k) => k.includes(searchWord)),
  );

  const handleRecordToggle = () => {
  if (isRecording) {
      setIsRecording(false);
      showToast(t('voice_summary.recording_saved'));
      // mock new record
      setTimeout(() => {
        setSummaries((prev) => [
          {
            id: Math.random().toString(),
            title: t('voice_summary.new_record_prefix') + new Date().toLocaleTimeString(),
            date: "刚刚",
            duration: "00:05",
            summary: t('voice_summary.mock_summary'),
            keywords: [t('voice_summary.mock_tag_1'), t('voice_summary.mock_tag_2'), t('voice_summary.mock_tag_3')],
          },
          ...prev,
        ]);
        showToast(t('voice_summary.analysis_complete'));
      }, 1500);
    } else {
      setIsRecording(true);
      showToast(t('voice_summary.recording_started'));
    }
  };

  return (
    <PageLayout title={t('voice_summary.title')}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <VoiceSummaryStats totalCount={summaries.length} />

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex justify-between items-center mb-3 mt-4 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {t('voice_summary.all_records')} ({filteredSummaries.length})
            </h2>
            <div className="flex gap-2 items-center">
              {showSearch && (
                <input
                  type="text"
                  value={searchWord}
                  onChange={(e) => setSearchWord(e.target.value)}
                  placeholder={t('voice_summary.search_placeholder')}
                  className="bg-white dark:bg-[#2c2d2e] px-3 py-1 text-[13px] rounded-md outline-none text-text-main shadow-sm w-32"
                />
              )}
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
                onClick={() => setShowSearch(!showSearch)}
              />
            </div>
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {filteredSummaries.length > 0 ? (
              filteredSummaries.map((summary) => (
                <VoiceSummaryItem
                  key={summary.id}
                  summary={summary}
                  playingId={playingId}
                  onPlayToggle={handlePlayToggle}
                />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileAudio className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('voice_summary.empty_state')}</span>
              </div>
            )}
          </div>
        </div>

        <FloatingRecordButton
          isRecording={isRecording}
          onRecordToggle={handleRecordToggle}
        />
      </div>
    </PageLayout>
  );
};
