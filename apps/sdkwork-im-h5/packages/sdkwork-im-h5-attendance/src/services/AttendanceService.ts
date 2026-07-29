export interface AttendanceRecord {
  id: string;
  type: "in" | "out";
  time: string;
  date: string;
  location: string;
}

export class AttendanceCapabilityUnavailableError extends Error {
  constructor() {
    super("Attendance is unavailable because its owner SDK is not composed.");
    this.name = "AttendanceCapabilityUnavailableError";
  }
}

export class AttendanceService {
  static async getRecords(): Promise<AttendanceRecord[]> {
    throw new AttendanceCapabilityUnavailableError();
  }

  static async clockIn(): Promise<AttendanceRecord> {
    throw new AttendanceCapabilityUnavailableError();
  }
}
