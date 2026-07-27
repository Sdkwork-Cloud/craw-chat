import React, { useState, useEffect } from "react";
import {
  ChevronLeft,
  Plus,
  Search,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { CalendarService, type Schedule } from "../services/CalendarService";
import { format } from "date-fns";

import { CalendarGrid } from "../components/CalendarGrid";
import { CalendarScheduleList } from "../components/CalendarScheduleList";
import { CalendarAddModal } from "../components/CalendarAddModal";
import { CalendarHeader } from "../components/CalendarHeader";

export const CalendarWorkspace: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [currentDate, setCurrentDate] = useState(new Date());
  const [schedules, setSchedules] = useState<Schedule[]>([]);
  const [loading, setLoading] = useState(true);
  const [indicators, setIndicators] = useState<string[]>([]);

  const [isAdding, setIsAdding] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newTime, setNewTime] = useState("");

  const loadData = async () => {
    setLoading(true);
    const data = await CalendarService.getSchedulesByDate(currentDate);
    setSchedules(data);
    setLoading(false);
  };

  useEffect(() => {
    loadData();
  }, [currentDate]);

  useEffect(() => {
    const loadIndicators = async () => {
      const year = currentDate.getFullYear();
      const month = currentDate.getMonth();
      const dates = await CalendarService.getIndicatorsForMonth(year, month);
      setIndicators(dates);
    };
    loadIndicators();
  }, [currentDate.getFullYear(), currentDate.getMonth(), schedules.length]); 

  const handleAddSchedule = async () => {
    if (!newTitle.trim()) {
      showToast(t('calendar.enter_title'));
      return;
    }
    await CalendarService.addSchedule({
      title: newTitle,
      time: newTime || t('calendar.all_day'),
      type: "event",
      color: "bg-blue-500",
      date: format(currentDate, "yyyy-MM-dd"),
    });
    showToast(t('calendar.add_success'));
    setIsAdding(false);
    setNewTitle("");
    setNewTime("");
    loadData();
  };

  const handleDeleteSchedule = async (id: number) => {
    await CalendarService.deleteSchedule(id);
    showToast(t('calendar.delete_success'));
    loadData();
  };

  const year = currentDate.getFullYear();
  const month = currentDate.getMonth();

  return (
    <div className="flex flex-col h-full bg-bg-color font-sans relative animate-in slide-in-from-right z-10 w-full absolute inset-0">
      <CalendarHeader
        year={year}
        month={month}
        onBack={() => navigate(-1)}
        onAdd={() => setIsAdding(true)}
      />

      <div className="flex-1 overflow-y-auto pb-safe flex flex-col">
        <CalendarGrid
          currentDate={currentDate}
          setCurrentDate={setCurrentDate}
          indicators={indicators}
        />
        <CalendarScheduleList
          currentDate={currentDate}
          schedules={schedules}
          loading={loading}
          setIsAdding={setIsAdding}
          handleDeleteSchedule={handleDeleteSchedule}
        />
      </div>

      <CalendarAddModal
        isAdding={isAdding}
        setIsAdding={setIsAdding}
        newTitle={newTitle}
        setNewTitle={setNewTitle}
        newTime={newTime}
        setNewTime={setNewTime}
        currentDate={currentDate}
        handleAddSchedule={handleAddSchedule}
      />
    </div>
  );
};
