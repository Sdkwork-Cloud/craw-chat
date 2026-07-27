import React, { useState, useRef, useEffect, useCallback } from "react";
import {
  FileText,
  Loader2,
  ChevronLeft
} from "lucide-react";
import { useNavigate } from "react-router";
import { IconButton, Tabs } from "@sdkwork/im-h5-commons";
import { notaryService } from "../services/notaryService";
import { useTranslation } from "react-i18next";
import { NotaryRecordsStatsCard } from "../components/NotaryRecordsStatsCard";
import { NotaryRecordListItem } from "../components/NotaryRecordListItem";

export const NotaryRecords: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<string>("all");
  const [items, setItems] = useState<any[]>([]);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);

  const [tabs, setTabs] = useState<{ id: string; label: string }[]>([]);

  const loadMoreRef = useRef<HTMLDivElement>(null);

  const touchStartX = useRef<number | null>(null);
  const touchStartY = useRef<number | null>(null);

  useEffect(() => {
    notaryService.getRecordTabs().then((data: any) => {
      setTabs(data.map((tab: any) => ({
        ...tab,
        label: t(`notary.records.tabs.${tab.id}`, tab.label)
      })));
    });
  }, [t]);

  useEffect(() => {
    let mounted = true;
    setLoading(true);
    setPage(1);
    setItems([]);
    setHasMore(true);

    notaryService.getNotaryRecords(activeTab, 1).then((data: any) => {
      if (mounted) {
        setItems(data.records || []);
        setHasMore(data.hasMore);
        setLoading(false);
      }
    });
    return () => {
      mounted = false;
    };
  }, [activeTab]);

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    touchStartY.current = e.touches[0].clientY;
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    if (touchStartX.current === null || touchStartY.current === null) return;
    const touchEndX = e.changedTouches[0].clientX;
    const touchEndY = e.changedTouches[0].clientY;

    const diffX = touchStartX.current - touchEndX;
    const diffY = touchStartY.current - touchEndY;

    if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 50) {
      const currentIndex = tabs.findIndex((t) => t.id === activeTab);
      if (diffX > 0) {
        if (currentIndex < tabs.length - 1)
          setActiveTab(tabs[currentIndex + 1].id);
      } else {
        if (currentIndex > 0) setActiveTab(tabs[currentIndex - 1].id);
      }
    }
    touchStartX.current = null;
    touchStartY.current = null;
  };

  const loadMoreData = useCallback(() => {
    if (isLoadingMore || !hasMore || loading) return;
    setIsLoadingMore(true);
    const nextPage = page + 1;
    notaryService.getNotaryRecords(activeTab, nextPage).then((data: any) => {
      setItems((prev) => [...prev, ...(data.records || [])]);
      setHasMore(data.hasMore);
      setPage(nextPage);
      setIsLoadingMore(false);
    });
  }, [isLoadingMore, hasMore, loading, activeTab, page]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        const target = entries[0];
        if (target.isIntersecting) {
          loadMoreData();
        }
      },
      { threshold: 0.1 },
    );

    if (loadMoreRef.current) observer.observe(loadMoreRef.current);
    return () => observer.disconnect();
  }, [loadMoreData]);

  const filteredRecords = items; // Backend mock already filters

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe z-20">
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
        <div className="absolute left-1/2 -translate-x-1/2 flex flex-col items-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t("notary.records.title")}</h1>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex-1 overflow-y-auto relative z-10 pb-[90px]">
        {/* Statistics */}
        <NotaryRecordsStatsCard />

        {/* Tabs */}
        <div className="sticky top-0 glass-header z-10 transition-colors border-b border-border-color">
          <Tabs
             tabs={tabs}
             activeTab={activeTab}
             onChange={setActiveTab}
             className="px-4 gap-6"
             itemClassName="py-3 text-[15px] font-medium"
             activeItemClassName="text-primary-blue"
          />
        </div>

        {/* List */}
        <div
          className="flex flex-col min-h-[50vh]"
          onTouchStart={handleTouchStart}
          onTouchEnd={handleTouchEnd}
        >
          {loading && (
            <div className="p-4 text-center text-text-sub">
              <Loader2 className="w-5 h-5 mx-auto animate-spin" />
            </div>
          )}
          {!loading &&
            filteredRecords.map((record, idx) => (
              <NotaryRecordListItem
                key={record.id}
                record={record}
                isLast={idx === filteredRecords.length - 1}
                onClick={() => navigate(`/notary/detail/${record.id}`)}
              />
            ))}

          {/* Infinite Scroll Trigger */}
          <div
            ref={loadMoreRef}
            className="h-16 flex items-center justify-center"
          >
            {isLoadingMore ? (
              <div className="flex items-center gap-2 text-text-sub">
                <Loader2 className="w-4 h-4 animate-spin" />
                <span className="text-[13px]">{t("notary.records.loading_more")}</span>
              </div>
            ) : hasMore ? (
              <span className="text-[12px] text-text-sub opacity-50">
                {t("notary.records.swipe_up")}
              </span>
            ) : items.length > 0 ? (
              <span className="text-[12px] text-text-sub opacity-50 relative z-0">
                {t("notary.records.end_of_list")}
              </span>
            ) : (
              <div className="flex flex-col items-center justify-center pt-20 pb-10 text-text-sub opacity-70 relative z-0">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t("notary.records.no_records")}</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
