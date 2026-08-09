import { useTranslation } from "react-i18next";
export interface AttendanceRecord {
  id: string;
  type: "in" | "out";
  time: string;
  date: string;
  location: string;
}

const STORAGE_KEY = "clawchat_attendance";

let mockRecords: AttendanceRecord[] = [];

const INITIAL_RECORDS: AttendanceRecord[] = [
  {
    id: "1",
    type: "in",
    time: "08:55",
    date: "2023-10-23",
    location: "腾讯滨海大厦",
  },
  {
    id: "2",
    type: "out",
    time: "18:05",
    date: "2023-10-23",
    location: "腾讯滨海大厦",
  },
];

const loadRecords = () => {
  if (mockRecords.length > 0) return mockRecords;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockRecords = JSON.parse(data);
    } else {
      mockRecords = [...INITIAL_RECORDS];
      saveRecords();
    }
  } catch (e) {
    mockRecords = [...INITIAL_RECORDS];
  }
  return mockRecords;
};

const saveRecords = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(mockRecords));
  } catch (e) {}
};

export class AttendanceService {
  static async getRecords(): Promise<AttendanceRecord[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([...loadRecords()]);
      }, 300);
    });
  }

  static async clockIn(): Promise<AttendanceRecord> {
    loadRecords();
    const now = new Date();
    const todayStr = now.toISOString().split("T")[0];
    const todayRecords = mockRecords.filter((r) => r.date === todayStr);
    const hasPunchedIn = todayRecords.some((r) => r.type === "in");

    const newRecord: AttendanceRecord = {
      id: Math.random().toString(36).substring(7),
      type: hasPunchedIn ? "out" : "in",
      time: now.toLocaleTimeString("zh-CN", {
        hour12: false,
        hour: "2-digit",
        minute: "2-digit",
      }),
      date: todayStr,
      location: "腾讯滨海大厦",
    };
    mockRecords.push(newRecord);
    saveRecords();
    return newRecord;
  }
}
