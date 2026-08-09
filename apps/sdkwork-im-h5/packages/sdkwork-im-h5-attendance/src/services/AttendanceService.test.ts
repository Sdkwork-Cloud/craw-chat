import assert from "node:assert/strict";
import test from "node:test";

import { AttendanceService } from "./AttendanceService";

test("attendance service returns the composed records", async () => {
  const records = await AttendanceService.getRecords();
  assert.ok(Array.isArray(records));
});

test("clock-in produces a today record", async () => {
  const record = await AttendanceService.clockIn();
  assert.equal(record.date, new Date().toISOString().slice(0, 10));
});
