import assert from "node:assert/strict";
import test from "node:test";

import {
  ApprovalCapabilityUnavailableError,
  ApprovalService,
} from "./ApprovalService";

test("approval service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(ApprovalService.getApprovals(), ApprovalCapabilityUnavailableError);
  await assert.rejects(ApprovalService.getApprovalDetail("1"), ApprovalCapabilityUnavailableError);
  await assert.rejects(
    ApprovalService.submitApproval({ approverIds: ["user-id"], content: "content", title: "title", type: "leave" }),
    ApprovalCapabilityUnavailableError,
  );
  await assert.rejects(
    ApprovalService.handleApproval({ id: "1", action: "approve", comment: "ok" }),
    ApprovalCapabilityUnavailableError,
  );
});
