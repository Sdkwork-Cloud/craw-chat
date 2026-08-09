import assert from "node:assert/strict";
import test from "node:test";

import { ReportService } from "./ReportService";

test("report service returns the composed reports", async () => {
  const reports = await ReportService.getReports();
  assert.ok(Array.isArray(reports));
});
