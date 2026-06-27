import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");

export const overviewItems = [
  { text: "API Overview", link: "/api-reference/index" },
  { text: "Gateway OpenAPI", link: "/api-reference/gateway-openapi" },
  { text: "Authentication and Errors", link: "/api-reference/auth-and-errors" },
  { text: "IM Standard API Overview", link: "/api-reference/im-api" },
  { text: "App API Overview", link: "/api-reference/app-api" },
  { text: "Backend API Overview", link: "/api-reference/backend-api" },
  { text: "Control Module Overview", link: "/api-reference/control-plane-api" },
];

export const groupedPages = [
  {
    text: "IM Standard API",
    pages: [
      { text: "Realtime Presence", link: "/api-reference/im/session-and-realtime" },
      { text: "Conversations and Handoff", link: "/api-reference/im/conversations" },
      { text: "Rooms", link: "/api-reference/im/rooms" },
      { text: "Membership and Read State", link: "/api-reference/im/membership-and-read-state" },
      { text: "Messages", link: "/api-reference/im/messages" },
      { text: "Media", link: "/api-reference/im/media" },
      { text: "Streams", link: "/api-reference/im/streams" },
      { text: "Calls", link: "/api-reference/im/calls" },
    ],
  },
  {
    text: "App API",
    pages: [
      { text: "Portal Access", link: "/api-reference/app/portal-access" },
      { text: "Notifications", link: "/api-reference/app/notifications" },
      { text: "Automation", link: "/api-reference/app/automation" },
      { text: "Provider Health", link: "/api-reference/app/provider-health" },
    ],
  },
  {
    text: "Backend API",
    pages: [
      { text: "Audit", link: "/api-reference/backend/audit" },
      { text: "Operations", link: "/api-reference/backend/ops" },
      { text: "Protocol Governance", link: "/api-reference/control-plane/protocol" },
      { text: "Provider Governance", link: "/api-reference/control-plane/providers" },
      { text: "Social Graph Control", link: "/api-reference/control-plane/social" },
      { text: "Social Runtime", link: "/api-reference/control-plane/social-runtime" },
      { text: "Node Operations", link: "/api-reference/control-plane/nodes" },
    ],
  },
];

export const pageOperationGroups = {
  "/api-reference/app/portal-access": [
    {
      text: "Public Portal Snapshots",
      anchors: ["get-home", "get-access"],
    },
    {
      text: "Authenticated Portal Views",
      anchors: [
        "get-workspace",
        "get-dashboard",
        "get-conversations",
        "get-realtime",
        "get-media",
        "get-automation",
        "get-governance",
      ],
    },
  ],
  "/api-reference/im/session-and-realtime": [
    { text: "Health and Probes", anchors: ["get-healthz", "get-readyz"] },
    { text: "Presence", anchors: ["heartbeat-presence", "get-presence-me"] },
    {
      text: "Realtime Delivery",
      anchors: [
        "sync-realtime-subscriptions",
        "list-realtime-events",
        "ack-realtime-events",
        "realtime-websocket",
      ],
    },
  ],
  "/api-reference/im/conversations": [
    {
      text: "Inbox and Provisioning",
      anchors: [
        "get-inbox",
        "create-conversation",
        "create-agent-dialog",
        "create-agent-handoff",
        "create-system-channel",
      ],
    },
    {
      text: "Conversation State",
      anchors: ["get-conversation-summary"],
    },
    {
      text: "Agent Handoff Flow",
      anchors: [
        "get-agent-handoff-state",
        "accept-agent-handoff",
        "resolve-agent-handoff",
        "close-agent-handoff",
      ],
    },
  ],
  "/api-reference/im/rooms": [
    {
      text: "Room Lifecycle",
      anchors: ["create-room", "get-room", "enter-room", "leave-room"],
    },
  ],
  "/api-reference/im/membership-and-read-state": [
    {
      text: "Membership",
      anchors: [
        "list-members",
        "add-member",
        "remove-member",
        "transfer-owner",
        "change-member-role",
        "leave-conversation",
      ],
    },
    { text: "Read Cursor", anchors: ["get-read-cursor", "update-read-cursor"] },
  ],
  "/api-reference/im/messages": [
    { text: "Timeline Read", anchors: ["get-timeline"] },
    {
      text: "Message Submission and Mutation",
      anchors: [
        "post-message",
        "publish-system-channel-message",
        "edit-message",
        "recall-message",
      ],
    },
  ],
  "/api-reference/im/streams": [
    {
      text: "Stream Lifecycle",
      anchors: ["open-stream", "checkpoint-stream", "complete-stream", "abort-stream"],
    },
    { text: "Frame Transport", anchors: ["append-stream-frame", "list-stream-frames"] },
  ],
  "/api-reference/im/calls": [
    {
      text: "Call Lifecycle",
      anchors: [
        "create-call-session",
        "retrieve-call-session",
        "invite-call-session",
        "accept-call-session",
        "reject-call-session",
        "end-call-session",
      ],
    },
    { text: "Signals and Credentials", anchors: ["post-call-signal", "issue-call-credential"] },
  ],
  "/api-reference/app/notifications": [
    { text: "Submission", anchors: ["request-notification"] },
    { text: "Read Models", anchors: ["list-notifications", "get-notification"] },
  ],
  "/api-reference/app/automation": [
    { text: "Executions", anchors: ["request-automation-execution", "get-automation-execution"] },
    {
      text: "Agent Responses",
      anchors: [
        "start-agent-response",
        "append-agent-response-frame",
        "complete-agent-response",
      ],
    },
    {
      text: "Agent Tool Calls",
      anchors: ["request-agent-tool-call", "complete-agent-tool-call"],
    },
  ],
  "/api-reference/backend/audit": [
    { text: "Write", anchors: ["record-audit-anchor"] },
    {
      text: "Read and Export",
      anchors: ["list-audit-records", "verify-audit-chain", "export-audit-bundle"],
    },
  ],
  "/api-reference/backend/ops": [
    { text: "Health and Topology", anchors: ["get-ops-health", "get-ops-cluster", "get-ops-lag", "get-ops-replay-status"] },
    { text: "Runtime and Provider Mirrors", anchors: ["get-ops-runtime-dir", "get-ops-provider-bindings", "get-ops-provider-binding-drift"] },
    { text: "Diagnostics", anchors: ["get-ops-diagnostics"] },
  ],
  "/api-reference/app/provider-health": [
    {
      text: "Provider Probes",
      anchors: [
        "get-media-provider-health",
        "get-principal-profile-provider-health",
      ],
    },
  ],
  "/api-reference/control-plane/protocol": [
    { text: "Probe", anchors: ["get-control-healthz"] },
    { text: "Registry and Governance", anchors: ["get-protocol-registry", "get-protocol-governance"] },
  ],
  "/api-reference/control-plane/providers": [
    { text: "Registry and Effective Bindings", anchors: ["get-provider-registry", "get-provider-bindings", "upsert-provider-binding-policy"] },
    { text: "Policy History and Diff", anchors: ["get-provider-policy-history", "get-provider-policy-diff"] },
    { text: "Preview and Rollback", anchors: ["preview-provider-policy", "rollback-provider-policy"] },
  ],
  "/api-reference/control-plane/social": [
    { text: "Direct Chat Binding", anchors: ["bind-direct-chat", "get-direct-chat-snapshot"] },
    {
      text: "External Collaboration",
      anchors: [
        "establish-external-connection",
        "get-external-connection-snapshot",
        "bind-external-member-link",
        "get-external-member-link-snapshot",
      ],
    },
    {
      text: "Friend Graph",
      anchors: [
        "submit-friend-request",
        "get-friend-request-snapshot",
        "activate-friendship",
        "get-friendship-snapshot",
      ],
    },
    {
      text: "Shared Channel Policy and Blocking",
      anchors: [
        "apply-shared-channel-policy",
        "get-shared-channel-policy-snapshot",
        "block-user",
        "get-user-block-snapshot",
      ],
    },
  ],
  "/api-reference/control-plane/social-runtime": [
    {
      text: "Pending Queue Control",
      anchors: [
        "get-pending-shared-channel-sync-inventory",
        "claim-pending-shared-channel-sync-targeted",
        "release-pending-shared-channel-sync-targeted",
        "reclaim-stale-pending-shared-channel-sync",
        "republish-pending-shared-channel-sync-targeted",
        "takeover-pending-shared-channel-sync-targeted",
      ],
    },
    {
      text: "Delivery and Dead Letter",
      anchors: [
        "get-delivery-state-shared-channel-sync-inventory",
        "get-delivered-shared-channel-sync-inventory",
        "get-dead-letter-shared-channel-sync-inventory",
        "requeue-dead-letter-shared-channel-sync",
        "requeue-dead-letter-shared-channel-sync-targeted",
      ],
    },
    {
      text: "Repair",
      anchors: [
        "repair-social-runtime-snapshot",
        "repair-shared-channel-sync",
      ],
    },
  ],
  "/api-reference/control-plane/nodes": [
    { text: "Lifecycle", anchors: ["drain-node", "activate-node"] },
    { text: "Route Migration", anchors: ["migrate-node-routes"] },
  ],
};

export function markdownPathFor(pageLink) {
  return path.join(
    docsRoot,
    `${pageLink.replace(/^\//, "").replaceAll("/", path.sep)}.md`,
  );
}

export function formatOperationText(operationTitle) {
  const match = operationTitle.match(/^([A-Z]+)\s+(.+)$/);
  if (!match) {
    return operationTitle;
  }

  const [, method, route] = match;
  const cleanRoute = route
    .replace(/^\/im\/v3\/api/, "")
    .replace(/^\/app\/v3\/api/, "")
    .replace(/^\/backend\/v3\/api/, "")
    .replace(/^\/api\/v1/, "") || "/";
  return `${method} ${cleanRoute}`;
}

export function readOperations(pageLink) {
  const content = fs.readFileSync(markdownPathFor(pageLink), "utf8");

  return [...content.matchAll(/<a id="([^"]+)"><\/a>[\s\S]*?## `([^`]+)`/g)].map(
    ([, anchor, operationTitle]) => ({
      anchor,
      operationTitle,
      text: formatOperationText(operationTitle),
      sourceLink: `${pageLink}#${anchor}`,
    }),
  );
}

export function operationPageLink(pageLink, anchor) {
  const pageTail = pageLink.replace(/^\/api-reference\//, "");
  return `/api-reference/operations/${pageTail}/${anchor}`;
}

export function operationMarkdownPath(pageLink, anchor) {
  return path.join(
    docsRoot,
    "api-reference",
    "operations",
    ...pageLink.replace(/^\/api-reference\//, "").split("/"),
    `${anchor}.md`,
  );
}

function buildPageItems(page) {
  const operations = readOperations(page.link);
  const operationByAnchor = new Map(operations.map((operation) => [operation.anchor, operation]));
  const consumed = new Set();
  const configuredGroups = pageOperationGroups[page.link] ?? [];

  const items = [{ text: "Overview", link: page.link }];

  for (const group of configuredGroups) {
    const groupItems = group.anchors
      .map((anchor) => operationByAnchor.get(anchor))
      .filter(Boolean)
      .map((operation) => {
        consumed.add(operation.anchor);
        return { text: operation.text, link: operationPageLink(page.link, operation.anchor) };
      });

    if (groupItems.length > 0) {
      items.push({ text: group.text, items: groupItems });
    }
  }

  const remaining = operations
    .filter((operation) => !consumed.has(operation.anchor))
    .map((operation) => ({
      text: operation.text,
      link: operationPageLink(page.link, operation.anchor),
    }));

  if (remaining.length > 0) {
    items.push({
      text: configuredGroups.length > 0 ? "Other Operations" : "Operations",
      items: remaining,
    });
  }

  return items;
}

function buildGroupedSidebar() {
  return groupedPages.map((group) => ({
    text: group.text,
    items: group.pages.map((page) => ({
      text: page.text,
      items: buildPageItems(page),
    })),
  }));
}

function collectOperationLinks(items) {
  const links = [];

  for (const item of items) {
    if (item.link?.startsWith("/api-reference/operations/")) {
      links.push(item.link);
    }
    if (item.items) {
      links.push(...collectOperationLinks(item.items));
    }
  }

  return links;
}

const groupedSidebar = buildGroupedSidebar();

export const apiReferenceSidebar = [
  {
    text: "Overview",
    items: overviewItems,
  },
  ...groupedSidebar,
];

export const apiReferenceOperationLinks = collectOperationLinks(groupedSidebar);
