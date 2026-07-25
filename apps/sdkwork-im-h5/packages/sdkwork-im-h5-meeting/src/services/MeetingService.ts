import { useTranslation } from "react-i18next";
import { ApiClient } from "@sdkwork/im-h5-commons";

export interface MeetingAttendee {
  id: string;
  name: string;
  avatar: string;
  role?: string;
  status?: "accepted" | "tentative" | "declined" | "pending";
}

export interface MeetingRecord {
  id: string;
  title: string;
  description?: string;
  startTime: string; // ISO string
  endTime: string; // ISO string
  date: string;
  time: string; // Formatting
  room: string;
  status: "upcoming" | "ongoing" | "finished" | "cancelled";
  organizerId: string;
  organizerName?: string;
  attendees: MeetingAttendee[];
  meetingUrl?: string;
  attachments?: { name: string; url: string }[];
}

export interface CreateMeetingRequest {
  title: string;
  description?: string;
  startTime: string;
  endTime: string;
  roomId?: string;
  attendeeIds: string[];
}

export interface UpdateMeetingRequest extends Partial<CreateMeetingRequest> {
  id: string;
  status?: MeetingRecord["status"];
}

const STORAGE_KEY = "sdkwork_im_h5_meetings";

let MOCK_MEETINGS: MeetingRecord[] = [];

const INITIAL_MEETINGS: MeetingRecord[] = [
  {
    id: "1",
    title: "Q3 战略复盘会",
    startTime: new Date().toISOString(),
    endTime: new Date(Date.now() + 7200000).toISOString(),
    time: "14:00 - 16:00",
    date: "今天",
    room: "腾讯大厦-3F-会议室A",
    status: "upcoming",
    organizerId: "u1",
    organizerName: "Admin",
    attendees: [
      {
        id: "u2",
        name: "张三",
        avatar: "https://picsum.photos/150",
        status: "accepted",
      },
      {
        id: "u3",
        name: "李四",
        avatar: "https://picsum.photos/150",
        status: "pending",
      },
    ],
  },
  {
    id: "2",
    title: "产研周会",
    time: "10:00 - 11:30",
    startTime: new Date(Date.now() - 7200000).toISOString(),
    endTime: new Date().toISOString(),
    date: "今天",
    room: "腾讯大厦-4F-会议室B",
    status: "finished",
    organizerId: "u1",
    attendees: [
      { id: "u4", name: "赵六", avatar: "https://picsum.photos/150" },
    ],
  },
];

const loadMeetings = () => {
  if (MOCK_MEETINGS.length > 0) return MOCK_MEETINGS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_MEETINGS = JSON.parse(data);
    } else {
      MOCK_MEETINGS = [...INITIAL_MEETINGS];
      saveMeetings();
    }
  } catch (e) {
    MOCK_MEETINGS = [...INITIAL_MEETINGS];
  }
  return MOCK_MEETINGS;
};

const saveMeetings = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_MEETINGS));
  } catch (e) {}
};

export class MeetingService {
  /**
   * Get list of meetings for current user
   * @param status Optional filter by status
   */
  static async getMeetings(
    status?: "upcoming" | "history",
  ): Promise<MeetingRecord[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadMeetings()]), 300),
    );
  }

  /**
   * Get detailed info for a single meeting
   */
  static async getMeetingDetail(id: string): Promise<MeetingRecord> {
    const meetings = loadMeetings();
    return meetings.find((m) => m.id === id) || meetings[0];
  }

  /**
   * Create a new meeting
   */
  static async createMeeting(
    data: CreateMeetingRequest,
  ): Promise<MeetingRecord> {
    loadMeetings();
    const newMeeting: MeetingRecord = {
      id: Math.random().toString(36).substr(2, 9),
      ...data,
      date: "今天",
      time: "待定",
      room: "待定",
      status: "upcoming",
      organizerId: "me",
      attendees: data.attendeeIds.map((id) => ({
        id,
        name: `用户 ${id}`,
        avatar: "https://picsum.photos/150",
      })),
    };
    MOCK_MEETINGS = [newMeeting, ...MOCK_MEETINGS];
    saveMeetings();
    return newMeeting;
  }

  /**
   * Update an existing meeting
   */
  static async updateMeeting(
    data: UpdateMeetingRequest,
  ): Promise<MeetingRecord> {
    loadMeetings();
    const index = MOCK_MEETINGS.findIndex((m) => m.id === data.id);
    if (index !== -1) {
      MOCK_MEETINGS[index] = { ...MOCK_MEETINGS[index], ...data } as any;
      saveMeetings();
      return MOCK_MEETINGS[index];
    }
    return this.getMeetingDetail(data.id);
  }

  /**
   * Cancel a meeting
   */
  static async cancelMeeting(id: string): Promise<boolean> {
    loadMeetings();
    MOCK_MEETINGS = MOCK_MEETINGS.filter((m) => m.id !== id);
    saveMeetings();
    return true;
  }
}
