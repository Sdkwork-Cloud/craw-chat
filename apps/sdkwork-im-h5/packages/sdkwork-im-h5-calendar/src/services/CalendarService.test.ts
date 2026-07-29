import assert from "node:assert/strict";
import test from "node:test";

import { CalendarCapabilityUnavailableError, CalendarService } from "./CalendarService";

test("calendar operations fail closed until the owner SDK is composed", async () => {
  const schedule = {
    color: "blue",
    date: "2026-07-29",
    time: "09:00",
    title: "test",
    type: "meeting",
  };
  for (const operation of [
    CalendarService.getSchedulesByDate(new Date()),
    CalendarService.getIndicatorsForMonth(2026, 6),
    CalendarService.addSchedule(schedule),
    CalendarService.deleteSchedule(1),
  ]) {
    await assert.rejects(operation, CalendarCapabilityUnavailableError);
  }
});
