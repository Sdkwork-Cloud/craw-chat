import assert from "node:assert/strict";
import test from "node:test";

import { RecruitmentService } from "./RecruitmentService";

test("recruitment service returns the composed candidate list", async () => {
  const candidates = await RecruitmentService.getCandidates();
  assert.ok(Array.isArray(candidates));
});
