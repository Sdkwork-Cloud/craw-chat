import React, { useCallback, useEffect, useRef, useState } from "react";
import { ChevronLeft, FileText, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { IconButton, Tabs } from "@sdkwork/im-h5-commons";
import { NotaryRecordListItem } from "../components/NotaryRecordListItem";
import { NotaryRecordsStatsCard } from "../components/NotaryRecordsStatsCard";
import {
  appendBoundedUnique,
  NOTARY_CLIENT_WINDOW_LIMIT,
  notaryService,
  type NotaryRecord,
  type NotaryRecordFilter,
} from "../services/notaryService";

export const NotaryRecords: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<NotaryRecordFilter>("ALL");
  const [items, setItems] = useState<NotaryRecord[]>([]);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [loading, setLoading] = useState(true);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [loadError, setLoadError] = useState(false);
  const [reloadVersion, setReloadVersion] = useState(0);
  const [tabs] = useState(() => notaryService.getRecordTabs());
  const loadMoreRef = useRef<HTMLDivElement>(null);
  const touchStartX = useRef<number | null>(null);
  const touchStartY = useRef<number | null>(null);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setItems([]);
    setNextCursor(undefined);
    setLoadError(false);

    void notaryService.getNotaryRecords(activeTab).then(
      (page) => {
        if (active) {
          setItems(page.records);
          setNextCursor(page.pageInfo.hasMore ? page.pageInfo.nextCursor : undefined);
        }
      },
      () => {
        if (active) {
          setLoadError(true);
        }
      },
    ).finally(() => {
      if (active) {
        setLoading(false);
      }
    });

    return () => {
      active = false;
    };
  }, [activeTab, reloadVersion]);

  const loadMoreData = useCallback(() => {
    if (isLoadingMore || !nextCursor || loading || loadError) {
      return;
    }
    setIsLoadingMore(true);
    void notaryService.getNotaryRecords(activeTab, nextCursor).then(
      (page) => {
        const merged = appendBoundedUnique(
          items,
          page.records,
          (record) => record.id,
        );
        setItems(merged);
        setNextCursor(
          page.pageInfo.hasMore && merged.length < NOTARY_CLIENT_WINDOW_LIMIT
            ? page.pageInfo.nextCursor
            : undefined,
        );
      },
      () => {
        setLoadError(true);
      },
    ).finally(() => {
      setIsLoadingMore(false);
    });
  }, [activeTab, isLoadingMore, items, loadError, loading, nextCursor]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          loadMoreData();
        }
      },
      { threshold: 0.1 },
    );
    const target = loadMoreRef.current;
    if (target) {
      observer.observe(target);
    }
    return () => observer.disconnect();
  }, [loadMoreData]);

  const handleTouchStart = (event: React.TouchEvent) => {
    touchStartX.current = event.touches[0].clientX;
    touchStartY.current = event.touches[0].clientY;
  };

  const handleTouchEnd = (event: React.TouchEvent) => {
    if (touchStartX.current === null || touchStartY.current === null) {
      return;
    }
    const diffX = touchStartX.current - event.changedTouches[0].clientX;
    const diffY = touchStartY.current - event.changedTouches[0].clientY;
    if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 50) {
      const currentIndex = tabs.findIndex((tab) => tab.id === activeTab);
      const nextIndex = diffX > 0 ? currentIndex + 1 : currentIndex - 1;
      if (tabs[nextIndex]) {
        setActiveTab(tabs[nextIndex].id);
      }
    }
    touchStartX.current = null;
    touchStartY.current = null;
  };

  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="glass-header relative z-20 flex h-[56px] shrink-0 items-center justify-between px-1 pt-safe">
        <div className="z-10 flex flex-1 items-center">
          <IconButton
            icon={<ChevronLeft className="h-6 w-6 text-text-main" strokeWidth={2.5} />}
            onClick={() => navigate(-1)}
          />
        </div>
        <h1 className="pointer-events-none absolute left-1/2 -translate-x-1/2 text-[17px] font-medium text-text-main">
          {t("notary.records.title")}
        </h1>
        <div className="flex-1" />
      </header>

      <div className="relative z-10 flex-1 overflow-y-auto pb-[90px]">
        <NotaryRecordsStatsCard />
        <div className="glass-header sticky top-0 z-10 border-b border-border-color">
          <Tabs
            tabs={tabs.map((tab) => ({ id: tab.id, label: t(tab.labelKey) }))}
            activeTab={activeTab}
            onChange={(id) => setActiveTab(id as NotaryRecordFilter)}
            className="gap-6 px-4"
            itemClassName="py-3 text-[15px] font-medium"
            activeItemClassName="text-primary-blue"
          />
        </div>

        <div
          className="flex min-h-[50vh] flex-col"
          onTouchStart={handleTouchStart}
          onTouchEnd={handleTouchEnd}
        >
          {loading && (
            <div className="p-4 text-center text-text-sub">
              <Loader2 className="mx-auto h-5 w-5 animate-spin" />
            </div>
          )}
          {!loading && items.map((record, index) => (
            <NotaryRecordListItem
              key={record.id}
              record={record}
              isLast={index === items.length - 1}
              onClick={() => navigate(`/notary/detail/${record.id}`)}
            />
          ))}

          <div ref={loadMoreRef} className="flex h-16 items-center justify-center">
            {isLoadingMore ? (
              <div className="flex items-center gap-2 text-text-sub">
                <Loader2 className="h-4 w-4 animate-spin" />
                <span className="text-[13px]">{t("notary.records.loading_more")}</span>
              </div>
            ) : loadError ? (
              <button
                type="button"
                className="text-[13px] font-medium text-primary-blue"
                onClick={() => setReloadVersion((version) => version + 1)}
              >
                {t("notary.records.retry", "Retry")}
              </button>
            ) : nextCursor ? (
              <span className="text-[12px] text-text-sub opacity-50">
                {t("notary.records.swipe_up")}
              </span>
            ) : items.length > 0 ? (
              <span className="text-[12px] text-text-sub opacity-50">
                {t("notary.records.end_of_list")}
              </span>
            ) : !loading ? (
              <div className="flex flex-col items-center justify-center pb-10 pt-20 text-text-sub opacity-70">
                <FileText className="mb-3 h-12 w-12 opacity-40" />
                <span className="text-[14px]">{t("notary.records.no_records")}</span>
              </div>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  );
};
