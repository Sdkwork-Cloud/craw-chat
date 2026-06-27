export interface CalendarEvent {
  id: string;
  title: string;
  date: string;
  time: string;
  color: string;
  location?: string;
  description?: string;
  guests?: string[];
}

export interface CalendarList {
  id: string;
  name: string;
  color: string;
  checked: boolean;
}

export interface CalendarService {
  getEvents(startDate: string, endDate: string): Promise<CalendarEvent[]>;
  createEvent(event: Omit<CalendarEvent, 'id'>): Promise<CalendarEvent>;
  updateEvent(id: string, updates: Partial<CalendarEvent>): Promise<CalendarEvent>;
  deleteEvent(id: string): Promise<void>;
  getCalendars(): Promise<CalendarList[]>;
  createCalendar(calendar: Omit<CalendarList, 'id'>): Promise<CalendarList>;
  updateCalendar(id: string, updates: Partial<CalendarList>): Promise<CalendarList>;
  deleteCalendar(id: string): Promise<void>;
}

const PC_CALENDAR_CONTRACT_UNAVAILABLE = 'pc calendar contract is not available';

function failClosedCalendarMutation(): never {
  throw new Error(PC_CALENDAR_CONTRACT_UNAVAILABLE);
}

class SdkworkCalendarService implements CalendarService {
  async getEvents(_startDate: string, _endDate: string): Promise<CalendarEvent[]> {
    return [];
  }

  async createEvent(_event: Omit<CalendarEvent, 'id'>): Promise<CalendarEvent> {
    failClosedCalendarMutation();
  }

  async updateEvent(_id: string, _updates: Partial<CalendarEvent>): Promise<CalendarEvent> {
    failClosedCalendarMutation();
  }

  async deleteEvent(_id: string): Promise<void> {
    failClosedCalendarMutation();
  }

  async getCalendars(): Promise<CalendarList[]> {
    return [];
  }

  async createCalendar(_calendar: Omit<CalendarList, 'id'>): Promise<CalendarList> {
    failClosedCalendarMutation();
  }

  async updateCalendar(_id: string, _updates: Partial<CalendarList>): Promise<CalendarList> {
    failClosedCalendarMutation();
  }

  async deleteCalendar(_id: string): Promise<void> {
    failClosedCalendarMutation();
  }
}

export const calendarService = new SdkworkCalendarService();
