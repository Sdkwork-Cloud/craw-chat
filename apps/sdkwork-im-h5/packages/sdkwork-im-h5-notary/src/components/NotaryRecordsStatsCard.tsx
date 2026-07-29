import React, { useEffect, useState } from "react";
import { AlertCircle, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  notaryService,
  type NotaryRecordsStatistics,
} from "../services/notaryService";

export const NotaryRecordsStatsCard: React.FC = () => {
  const { t } = useTranslation();
  const [statistics, setStatistics] = useState<NotaryRecordsStatistics | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    let active = true;
    void notaryService.getRecordsStatistics().then(
      (value) => {
        if (active) {
          setStatistics(value);
        }
      },
      () => {
        if (active) {
          setError(true);
        }
      },
    );
    return () => {
      active = false;
    };
  }, []);

  if (error) {
    return (
      <div className="flex h-16 items-center justify-center gap-2 text-[13px] text-text-sub">
        <AlertCircle className="h-4 w-4" />
        {t("notary.records.statistics_unavailable", "Statistics unavailable")}
      </div>
    );
  }

  if (!statistics) {
    return (
      <div className="flex h-16 items-center justify-center text-text-sub">
        <Loader2 className="h-5 w-5 animate-spin" />
      </div>
    );
  }

  const metrics = [
    {
      label: t("notary.records.pending_review"),
      value: statistics.pendingReview,
    },
    {
      label: t("notary.records.completed_today"),
      value: statistics.completedToday,
    },
    {
      label: t("notary.records.anomalies_intercepted"),
      value: statistics.anomaliesIntercepted,
    },
    {
      label: t("notary.records.total_this_month"),
      value: statistics.monthlyTotal,
    },
  ];

  return (
    <div className="grid grid-cols-2 gap-3 p-4">
      {metrics.map((metric) => (
        <div
          key={metric.label}
          className="flex h-[84px] flex-col justify-between rounded-lg border border-border-color bg-chat-other-bg p-3.5"
        >
          <span className="text-[12px] font-medium text-text-sub">{metric.label}</span>
          <span className="font-mono text-[24px] font-bold leading-none text-text-main">
            {metric.value.toLocaleString()}
          </span>
        </div>
      ))}
    </div>
  );
};
