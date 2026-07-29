import assert from "node:assert/strict";
import test from "node:test";

import {
  RecruitmentCapabilityUnavailableError,
  RecruitmentService,
} from "./RecruitmentService";

test("recruitment operations fail closed until the owner SDK is composed", async () => {
  for (const operation of [
    () => RecruitmentService.getCandidates(),
    () => RecruitmentService.updateCandidateStage("candidate-id", "interview"),
    () => RecruitmentService.deleteCandidate("candidate-id"),
  ]) {
    await assert.rejects(operation, RecruitmentCapabilityUnavailableError);
  }
});
