import assert from "node:assert/strict";
import test from "node:test";

import {
  AttendanceCapabilityUnavailableError,
  AttendanceService,
} from "./AttendanceService";

test("attendance service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(AttendanceService.getRecords(), AttendanceCapabilityUnavailableError);
  await assert.rejects(AttendanceService.clockIn(), AttendanceCapabilityUnavailableError);
});
