import React, { useState, useEffect } from "react";
import { CapabilityUnavailablePage, PageLayout } from "@sdkwork/im-h5-commons";
import {
  AttendanceService,
  AttendanceRecord,
} from "../services/AttendanceService";
import { useTranslation } from "react-i18next";
import { Clock } from "lucide-react";
import { AttendanceHeader } from "../components/AttendanceHeader";
import { PunchButton } from "../components/PunchButton";
import { AttendanceHistory } from "../components/AttendanceHistory";

export const AttendanceApp = () => {
  const { t } = useTranslation();
const [records, setRecords] = useState<AttendanceRecord[]>([]);
  const [time, setTime] = useState(new Date());
  const [unavailable, setUnavailable] = useState(false);

  useEffect(() => {
    AttendanceService.getRecords()
      .then(setRecords)
      .catch((error) => {
        console.error(error);
        setUnavailable(true);
      });

    const timer = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const handleClockIn = async () => {
    await AttendanceService.clockIn();
    const latest = await AttendanceService.getRecords();
    setRecords(latest);
  };

  if (unavailable) {
    return (
      <CapabilityUnavailablePage
        icon={Clock}
        title={t("attendance.title")}
        message={t("attendance.unavailable")}
        onBack={() => window.history.back()}
      />
    );
  }

  const todayRecords = records.filter(
    (r) => r.date === new Date().toISOString().split("T")[0],
  );
  const hasPunchedIn = todayRecords.some((r) => r.type === "in");
  const hasPunchedOut = todayRecords.some((r) => r.type === "out");
  const isDoneToday = hasPunchedIn && hasPunchedOut;

  return (
    <PageLayout title={t('attendance.title')}>
      <div className="flex flex-col h-full bg-bg-color">
        <AttendanceHeader time={time} />

        <div className="flex-1 flex flex-col items-center pt-12 px-6">
          <PunchButton
            handleClockIn={handleClockIn}
            isDoneToday={isDoneToday}
            hasPunchedIn={hasPunchedIn}
          />

          <AttendanceHistory todayRecords={todayRecords} />
        </div>
      </div>
    </PageLayout>
  );
};
