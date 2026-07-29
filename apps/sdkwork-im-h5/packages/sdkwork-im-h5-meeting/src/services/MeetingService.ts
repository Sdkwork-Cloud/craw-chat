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
  startTime: string;
  endTime: string;
  date: string;
  time: string;
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

export class MeetingCapabilityUnavailableError extends Error {
  constructor() {
    super("Meeting is unavailable because its owner SDK is not composed.");
    this.name = "MeetingCapabilityUnavailableError";
  }
}

export class MeetingService {
  static async getMeetings(_status?: "upcoming" | "history"): Promise<MeetingRecord[]> {
    throw new MeetingCapabilityUnavailableError();
  }

  static async getMeetingDetail(_id: string): Promise<MeetingRecord> {
    throw new MeetingCapabilityUnavailableError();
  }

  static async createMeeting(_data: CreateMeetingRequest): Promise<MeetingRecord> {
    throw new MeetingCapabilityUnavailableError();
  }

  static async updateMeeting(_data: UpdateMeetingRequest): Promise<MeetingRecord> {
    throw new MeetingCapabilityUnavailableError();
  }

  static async cancelMeeting(_id: string): Promise<boolean> {
    throw new MeetingCapabilityUnavailableError();
  }
}
