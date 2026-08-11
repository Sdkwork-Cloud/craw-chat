import assert from "node:assert/strict";
import test from "node:test";

import {
  CalendarCapabilityUnavailableError,
  CalendarService,
} from "./CalendarService";

test("calendar service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(CalendarService.getSchedulesByDate(new Date()), CalendarCapabilityUnavailableError);
  await assert.rejects(CalendarService.getIndicatorsForMonth(2026, 7), CalendarCapabilityUnavailableError);
  await assert.rejects(
    CalendarService.addSchedule({ title: "t", time: "09:00", type: "meeting", color: "bg-blue-500", date: "2026-08-11" }),
    CalendarCapabilityUnavailableError,
  );
  await assert.rejects(CalendarService.deleteSchedule(1), CalendarCapabilityUnavailableError);
});
