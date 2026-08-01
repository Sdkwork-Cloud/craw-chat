import assert from "node:assert/strict";
import test from "node:test";

import { IM_H5_ROUTE_REGISTRY } from "./routes";

const MOUNTED_ROUTE_PATHS = [
  "/",
  "/chat/:conversationId",
  "/chat/:id/profile",
  "/create-group",
  "/call/voice/:id",
  "/call/video/:id",
  "/workspace/contacts",
  "/search",
  "/add-friend",
  "/contacts/friend-requests",
  "/contacts/org",
  "/workspace/drive",
  "/workspace/drive/share/:token",
  "/workspace",
  "/workspace/notary",
  "/notary",
  "/notary/files",
  "/notary/messages",
  "/notary/me",
  "/notary/create",
  "/notary/search",
  "/notary/add-party",
  "/notary/detail/:id",
  "/notary/messages/:messageId",
  "/notary/chat/:caseId",
  "/notary/cases/:caseId/parties/:partyId/signature",
  "/notary/cases/:caseId/parties/:partyId/video",
  "/notary/cases/:caseId/parties/:partyId/video-qr",
  "/orders",
  "/orders/:orderId",
  "/orders/:orderId/cashier",
  "/orders/voucher",
] as const;

test("route registry matches every built-in H5 shell route contribution", () => {
  assert.deepEqual(
    IM_H5_ROUTE_REGISTRY.map((route) => route.path),
    MOUNTED_ROUTE_PATHS,
  );
  assert.equal(
    new Set(IM_H5_ROUTE_REGISTRY.map((route) => route.id)).size,
    IM_H5_ROUTE_REGISTRY.length,
  );
  for (const route of IM_H5_ROUTE_REGISTRY) {
    assert.match(route.id, /^(app|console|admin)\.[a-z0-9-]+\.[a-z0-9-]+\.[a-zA-Z0-9-]+$/u);
  }
});
