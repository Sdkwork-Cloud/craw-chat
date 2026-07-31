import assert from "node:assert/strict";
import test from "node:test";

import { IM_H5_ROUTE_REGISTRY } from "./routes";

const MOUNTED_ROUTE_PATHS = [
  "/",
  "/chat/:conversationId",
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
] as const;

test("route registry matches every route mounted by the H5 shell", () => {
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
