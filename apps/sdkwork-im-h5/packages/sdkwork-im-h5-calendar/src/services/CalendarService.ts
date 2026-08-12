/**
 * Calendar capability — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * generated schedules and legacy `clawchat_*` storage are removed: every
 * method throws a typed `CalendarCapabilityUnavailableError` so any page
 * that reaches this surface shows a typed unavailable state instead of
 * fabricated schedule data.
 */

export interface Schedule {
  id: number;
  title: string;
  time: string;
  type: string;
  color: string;
  date: string; // ISO date string YYYY-MM-DD
}

export class CalendarCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "CalendarCapabilityUnavailableError";
  }
}

export const CalendarService = {
  async getSchedulesByDate(_date: Date): Promise<Schedule[]> {
    throw new CalendarCapabilityUnavailableError("Calendar schedules");
  },

  async getIndicatorsForMonth(_year: number, _month: number): Promise<string[]> {
    throw new CalendarCapabilityUnavailableError("Calendar month indicators");
  },

  async addSchedule(_schedule: Omit<Schedule, "id">): Promise<Schedule> {
    throw new CalendarCapabilityUnavailableError("Calendar schedule creation");
  },

  async deleteSchedule(_id: number): Promise<void> {
    throw new CalendarCapabilityUnavailableError("Calendar schedule deletion");
  },
};
