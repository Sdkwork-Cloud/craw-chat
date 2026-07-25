import { useTranslation } from "react-i18next";
import { format, isSameDay, addDays } from "date-fns";

export interface Schedule {
  id: number;
  title: string;
  time: string;
  type: string;
  color: string;
  date: string; // ISO date string YYYY-MM-DD
}

const STORAGE_KEY = "sdkwork_im_h5_calendar_schedules";

const generateMockSchedules = (): Schedule[] => {
  const today = new Date();
  return [
    {
      id: 1,
      title: "晨会：产研同步",
      time: "09:30 - 10:00",
      type: "meeting",
      color: "bg-blue-500",
      date: format(today, "yyyy-MM-dd"),
    },
    {
      id: 2,
      title: "公证业务线设计评审",
      time: "14:00 - 15:30",
      type: "review",
      color: "bg-emerald-500",
      date: format(today, "yyyy-MM-dd"),
    },
    {
      id: 3,
      title: "与客户沟通续签",
      time: "16:00 - 17:00",
      type: "call",
      color: "bg-purple-500",
      date: format(today, "yyyy-MM-dd"),
    },
    {
      id: 4,
      title: "季度周报整理",
      time: "10:00 - 11:30",
      type: "work",
      color: "bg-orange-500",
      date: format(addDays(today, 1), "yyyy-MM-dd"),
    },
    {
      id: 5,
      title: "团队团建：聚餐",
      time: "18:30 - 21:00",
      type: "event",
      color: "bg-rose-500",
      date: format(addDays(today, 2), "yyyy-MM-dd"),
    },
  ];
};

let MOCK_SCHEDULES: Schedule[] = [];

const loadSchedules = () => {
  if (MOCK_SCHEDULES.length > 0) return MOCK_SCHEDULES;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_SCHEDULES = JSON.parse(data);
    } else {
      MOCK_SCHEDULES = generateMockSchedules();
      saveSchedules();
    }
  } catch (e) {
    MOCK_SCHEDULES = generateMockSchedules();
  }
  return MOCK_SCHEDULES;
};

const saveSchedules = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_SCHEDULES));
  } catch (e) {
    console.error("Failed to save calendar data", e);
  }
};

export const CalendarService = {
  async getSchedulesByDate(date: Date): Promise<Schedule[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const schedules = loadSchedules();
        const dateStr = format(date, "yyyy-MM-dd");
        resolve(schedules.filter((s) => s.date === dateStr));
      }, 300);
    });
  },

  async getIndicatorsForMonth(year: number, month: number): Promise<string[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const schedules = loadSchedules();
        const startStr = format(new Date(year, month, 1), "yyyy-MM-dd");
        const endStr = format(new Date(year, month + 1, 0), "yyyy-MM-dd");

        const datesWithSchedules = schedules
          .filter((s) => s.date >= startStr && s.date <= endStr)
          .map((s) => s.date);

        resolve(Array.from(new Set(datesWithSchedules)));
      }, 100);
    });
  },

  async addSchedule(schedule: Omit<Schedule, "id">): Promise<Schedule> {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadSchedules();
        const newSchedule = { ...schedule, id: Date.now() };
        MOCK_SCHEDULES = [...MOCK_SCHEDULES, newSchedule];
        saveSchedules();
        resolve(newSchedule);
      }, 200);
    });
  },

  async deleteSchedule(id: number): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadSchedules();
        MOCK_SCHEDULES = MOCK_SCHEDULES.filter((s) => s.id !== id);
        saveSchedules();
        resolve();
      }, 200);
    });
  },
};
