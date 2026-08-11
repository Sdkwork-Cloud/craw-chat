import assert from "node:assert/strict";
import test from "node:test";

import { ReportCapabilityUnavailableError, ReportService } from "./ReportService";

test("report service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(ReportService.getReports(), ReportCapabilityUnavailableError);
  await assert.rejects(
    ReportService.submitReport({ type: "日报", reporter: "me", date: "today", summary: "s" }),
    ReportCapabilityUnavailableError,
  );
});
