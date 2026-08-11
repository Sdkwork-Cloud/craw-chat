import assert from "node:assert/strict";
import test from "node:test";

import {
  RecruitmentCapabilityUnavailableError,
  RecruitmentService,
} from "./RecruitmentService";

test("recruitment service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(RecruitmentService.getCandidates(), RecruitmentCapabilityUnavailableError);
  await assert.rejects(RecruitmentService.updateCandidateStage("1", "一面"), RecruitmentCapabilityUnavailableError);
  await assert.rejects(RecruitmentService.deleteCandidate("1"), RecruitmentCapabilityUnavailableError);
});
