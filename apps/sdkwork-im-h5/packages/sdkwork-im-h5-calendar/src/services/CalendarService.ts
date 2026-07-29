export interface Schedule {
  id: number;
  title: string;
  time: string;
  type: string;
  color: string;
  date: string;
}

export class CalendarCapabilityUnavailableError extends Error {
  constructor() {
    super("Calendar is unavailable because its owner SDK is not composed.");
    this.name = "CalendarCapabilityUnavailableError";
  }
}

export const CalendarService = {
  async getSchedulesByDate(_date: Date): Promise<Schedule[]> {
    throw new CalendarCapabilityUnavailableError();
  },

  async getIndicatorsForMonth(_year: number, _month: number): Promise<string[]> {
    throw new CalendarCapabilityUnavailableError();
  },

  async addSchedule(_schedule: Omit<Schedule, "id">): Promise<Schedule> {
    throw new CalendarCapabilityUnavailableError();
  },

  async deleteSchedule(_id: number): Promise<void> {
    throw new CalendarCapabilityUnavailableError();
  },
};
