import assert from "node:assert/strict";
import test from "node:test";

import { ApprovalCapabilityUnavailableError, ApprovalService } from "./ApprovalService";

test("approval operations fail closed until the owner SDK is composed", async () => {
  for (const operation of [
    ApprovalService.getApprovals(),
    ApprovalService.getApprovalDetail("approval-id"),
    ApprovalService.submitApproval({
      approverIds: ["user-id"],
      content: "content",
      title: "title",
      type: "leave",
    }),
    ApprovalService.handleApproval({ action: "approve", comment: "ok", id: "approval-id" }),
  ]) {
    await assert.rejects(operation, ApprovalCapabilityUnavailableError);
  }
});
