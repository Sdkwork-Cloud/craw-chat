import assert from "node:assert/strict";
import test from "node:test";

import { AttendanceCapabilityUnavailableError, AttendanceService } from "./AttendanceService";

test("attendance operations fail closed until the owner SDK is composed", async () => {
  await assert.rejects(AttendanceService.getRecords(), AttendanceCapabilityUnavailableError);
  await assert.rejects(AttendanceService.clockIn(), AttendanceCapabilityUnavailableError);
});
