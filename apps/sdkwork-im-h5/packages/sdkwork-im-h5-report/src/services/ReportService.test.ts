import assert from "node:assert/strict";
import test from "node:test";

import { ReportCapabilityUnavailableError, ReportService } from "./ReportService";

test("report operations fail closed until the owner SDK is composed", async () => {
  await assert.rejects(ReportService.getReports(), ReportCapabilityUnavailableError);
  await assert.rejects(
    ReportService.submitReport({ date: "now", reporter: "user", summary: "test", type: "daily" }),
    ReportCapabilityUnavailableError,
  );
});
