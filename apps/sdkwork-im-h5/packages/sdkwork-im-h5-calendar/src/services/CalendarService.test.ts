import assert from "node:assert/strict";
import test from "node:test";

import { CalendarService } from "./CalendarService";

test("calendar service returns schedules for a date", async () => {
  const schedules = await CalendarService.getSchedulesByDate(new Date());
  assert.ok(Array.isArray(schedules));
});
