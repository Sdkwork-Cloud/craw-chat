import assert from "node:assert/strict";
import test from "node:test";

import { ApprovalService } from "./ApprovalService";

test("approval service returns the composed approval list", async () => {
  const approvals = await ApprovalService.getApprovals();
  assert.ok(Array.isArray(approvals));
});

test("approval detail resolves for an existing id", async () => {
  const approvals = await ApprovalService.getApprovals();
  const detail = await ApprovalService.getApprovalDetail(approvals[0].id);
  assert.equal(detail.id, approvals[0].id);
});

test("submitting an approval produces an item with the request fields", async () => {
  const item = await ApprovalService.submitApproval({
    approverIds: ["user-id"],
    content: "content",
    title: "title",
    type: "leave",
  });
  assert.equal(item.title, "title");
  assert.equal(item.status, "pending");
});
