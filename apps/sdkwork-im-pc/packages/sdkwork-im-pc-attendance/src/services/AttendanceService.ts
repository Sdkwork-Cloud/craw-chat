export interface AttendanceStats {
  presentDays: number;
  lateTimes: number;
  earlyLeaveTimes: number;
  missedPunches: number;
}

export interface AttendanceRecord {
  id: string;
  date: string;
  type: string;
  status: 'pending' | 'resolved';
}

export interface AttendanceStatus {
  isCheckedIn: boolean;
  isCheckedOut: boolean;
  checkInTime: string | null;
  checkOutTime: string | null;
  location: string;
  inRange: boolean;
}

const PC_ATTENDANCE_CONTRACT_UNAVAILABLE = 'pc attendance contract is not available';

function failClosedAttendanceMutation(): never {
  throw new Error(PC_ATTENDANCE_CONTRACT_UNAVAILABLE);
}

class SdkworkAttendanceService {
  async getStats(): Promise<AttendanceStats> {
    return {
      presentDays: 0,
      lateTimes: 0,
      earlyLeaveTimes: 0,
      missedPunches: 0,
    };
  }

  async getRecords(): Promise<AttendanceRecord[]> {
    return [];
  }

  async getCurrentStatus(): Promise<AttendanceStatus> {
    return {
      isCheckedIn: false,
      isCheckedOut: false,
      checkInTime: null,
      checkOutTime: null,
      location: '',
      inRange: false,
    };
  }

  async punchIn(): Promise<{ success: boolean; time: string }> {
    failClosedAttendanceMutation();
  }

  async punchOut(): Promise<{ success: boolean; time: string }> {
    failClosedAttendanceMutation();
  }

  async submitAppeal(_recordId: string, _reason: string): Promise<boolean> {
    failClosedAttendanceMutation();
  }
}

export const attendanceService = new SdkworkAttendanceService();
