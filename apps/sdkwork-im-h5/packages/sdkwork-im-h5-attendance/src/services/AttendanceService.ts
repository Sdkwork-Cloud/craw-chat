/**
 * Attendance capability — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * in-memory records, fake clock-in and `clawchat_*` storage are removed:
 * every method throws a typed `AttendanceCapabilityUnavailableError` so any
 * page that reaches this surface shows a typed unavailable state instead of
 * fabricated attendance data.
 */

export interface AttendanceRecord {
  id: string;
  type: "in" | "out";
  time: string;
  date: string;
  location: string;
}

export class AttendanceCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "AttendanceCapabilityUnavailableError";
  }
}

export class AttendanceService {
  static async getRecords(): Promise<AttendanceRecord[]> {
    throw new AttendanceCapabilityUnavailableError("Attendance records");
  }

  static async clockIn(): Promise<AttendanceRecord> {
    throw new AttendanceCapabilityUnavailableError("Attendance clock-in");
  }
}
