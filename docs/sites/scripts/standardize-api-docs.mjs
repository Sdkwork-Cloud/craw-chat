import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const apiRoot = path.join(docsRoot, "api-reference");

const markdownFiles = [];

function walk(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(fullPath);
      continue;
    }
    if (entry.name.endsWith(".md")) {
      markdownFiles.push(fullPath);
    }
  }
}

function normalizePath(filePath) {
  return filePath.replaceAll("\\", "/");
}

function pageKey(filePath) {
  const normalized = normalizePath(filePath);
  const marker = "/api-reference/";
  const index = normalized.lastIndexOf(marker);
  return index === -1 ? normalized : normalized.slice(index + marker.length).replace(/\.md$/, "");
}

function routeSuffix(route) {
  return route
    .replace(/^\/im\/v3\/api/, "")
    .replace(/^\/app\/v3\/api/, "")
    .replace(/^\/backend\/v3\/api/, "")
    .replace(/^\/api\/v1/, "");
}

function isRoute(route, suffix) {
  return routeSuffix(route) === suffix;
}

function inferSuccessLabel(method, route, block) {
  const firstResponse = block.match(/### Response `(\d+)`/);
  if (!firstResponse) {
    return method === "GET" && route.endsWith("/ws")
      ? "101 Switching Protocols"
      : "Response";
  }

  const status = firstResponse[1];
  const schemaMatch = block.match(
    new RegExp(`### Response \`${status}\`[\\s\\S]*?<ApiSchemaTable schema="([^"]+)" \\/>`),
  );
  if (schemaMatch) {
    return `${status} ${schemaMatch[1]}`;
  }

  if (status === "101") {
    return "101 Switching Protocols";
  }

  return status;
}

function sdkLabel(page, route, method) {
  const labels = {
    "im/session-and-realtime": "`@sdkwork/im-sdk` / presence and realtime helpers",
    "im/conversations": isRoute(route, "/chat/inbox") || isRoute(route, "/inbox")
      ? "`@sdkwork/im-sdk` / `sdk.generated.inbox.getInbox()`"
      : "`@sdkwork/im-sdk` / `sdk.conversations`",
    "im/membership-and-read-state": "`@sdkwork/im-sdk` / `sdk.conversations`",
    "im/messages": "`@sdkwork/im-sdk` / `sdk.messages`",
    "im/rooms": "`@sdkwork/im-sdk` / `sdk.rooms`",
    "im/media": "`@sdkwork/im-sdk` / `sdk.media`",
    "im/streams": "`@sdkwork/im-sdk` / generated stream transport",
    "im/calls": "`@sdkwork/im-sdk` / `sdk.calls`",
    "app/notifications": "`sdkwork-im-app-sdk` / `client.notification`",
    "app/automation": "`sdkwork-im-app-sdk` / `client.automation`",
    "app/provider-health": "`sdkwork-im-app-sdk` / provider health",
    "backend/audit": "`sdkwork-im-backend-sdk` / audit",
    "backend/ops": "`sdkwork-im-backend-sdk` / ops",
    "control-plane/protocol": "`sdkwork-im-backend-sdk` / control.protocol",
    "control-plane/providers": "`sdkwork-im-backend-sdk` / control.providers",
    "control-plane/social": "`sdkwork-im-backend-sdk` / control.social",
    "control-plane/social-runtime": "`sdkwork-im-backend-sdk` / control.socialRuntime",
    "control-plane/nodes": "`sdkwork-im-backend-sdk` / control.nodes",
  };
  if (page === "im/session-and-realtime") {
    if (route === "/healthz" || route === "/readyz") {
      return "Direct HTTP probe";
    }
    if (isRoute(route, "/device/sessions/resume")) {
      return "`@sdkwork/im-sdk` / `sdk.connect({ clientRouteId })`";
    }
    if (isRoute(route, "/device/sessions/disconnect")) {
      return "`@sdkwork/im-sdk` / realtime disconnect";
    }
    if (isRoute(route, "/presence/heartbeat")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.presence.heartbeat(...)`";
    }
    if (isRoute(route, "/presence/me")) {
      return "`@sdkwork/im-sdk` / `sdk.presence.getPresenceMe()`";
    }
    if (isRoute(route, "/realtime/subscriptions/sync")) {
      return "`@sdkwork/im-sdk` / `sdk.connect(...)`, `sdk.generated.realtime.syncRealtimeSubscriptions(...)`";
    }
    if (isRoute(route, "/realtime/events")) {
      return "`@sdkwork/im-sdk` / `sdk.sync.catchUp(...)`, `sdk.generated.realtime.listRealtimeEvents(...)`";
    }
    if (isRoute(route, "/realtime/events/ack")) {
      return "`@sdkwork/im-sdk` / `sdk.sync.ack(...)`, `context.ack()`, `sdk.generated.realtime.ackRealtimeEvents(...)`";
    }
    if (route.endsWith("/ws")) {
      return "`@sdkwork/im-sdk` / `sdk.connect(...)`";
    }
  }

  if (page === "app/portal-access") {
    if (isRoute(route, "/portal/home")) {
      return "`sdkwork-im-app-sdk` / `client.portal.home.retrieve()`";
    }
    if (isRoute(route, "/portal/access")) {
      return "`sdkwork-im-app-sdk` / `client.portal.access.retrieve()`";
    }
    if (isRoute(route, "/portal/workspace")) {
      return "`sdkwork-im-app-sdk` / `client.portal.workspace.retrieve()`";
    }
    if (isRoute(route, "/portal/dashboard")) {
      return "`sdkwork-im-app-sdk` / `client.portal.dashboard.retrieve()`";
    }
    if (isRoute(route, "/portal/conversations")) {
      return "`sdkwork-im-app-sdk` / `client.portal.conversationSnapshot.retrieve()`";
    }
    if (isRoute(route, "/portal/realtime")) {
      return "`sdkwork-im-app-sdk` / `client.portal.realtime.retrieve()`";
    }
    if (isRoute(route, "/portal/media")) {
      return "`sdkwork-im-app-sdk` / `client.portal.media.retrieve()`";
    }
    if (isRoute(route, "/portal/automation")) {
      return "`sdkwork-im-app-sdk` / `client.portal.automation.retrieve()`";
    }
    if (isRoute(route, "/portal/governance")) {
      return "`sdkwork-im-app-sdk` / `client.portal.governance.retrieve()`";
    }
  }

  if (page === "app/automation") {
    if (isRoute(route, "/automation/executions")) {
      return "`sdkwork-im-app-sdk` / `client.automation.executions.create(body)`";
    }
    if (routeSuffix(route) === "/automation/executions/{executionId}") {
      return "`sdkwork-im-app-sdk` / `client.automation.executions.retrieve(executionId)`";
    }
    if (isRoute(route, "/automation/agent_responses")) {
      return "`sdkwork-im-app-sdk` / `client.automation.agentResponses.create(body)`";
    }
    if (routeSuffix(route) === "/automation/agent_responses/{streamId}/frames") {
      return "`sdkwork-im-app-sdk` / `client.automation.agentResponses.frames.create(streamId, body)`";
    }
    if (routeSuffix(route) === "/automation/agent_responses/{streamId}/complete") {
      return "`sdkwork-im-app-sdk` / `client.automation.agentResponses.complete(streamId, body)`";
    }
    if (isRoute(route, "/automation/agent_tool_calls")) {
      return "`sdkwork-im-app-sdk` / `client.automation.agentToolCalls.create(body)`";
    }
    if (routeSuffix(route) === "/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete") {
      return "`sdkwork-im-app-sdk` / `client.automation.agentToolCalls.complete(executionId, toolCallId, body)`";
    }
  }

  if (page === "im/rooms") {
    if (isRoute(route, "/chat/rooms") && method === "POST") {
      return "`@sdkwork/im-sdk` / `sdk.rooms.create(...)`";
    }
    if (isRoute(route, "/chat/rooms/{roomId}") && method === "GET") {
      return "`@sdkwork/im-sdk` / `sdk.rooms.get(...)`";
    }
    if (routeSuffix(route) === "/chat/rooms/{roomId}/enter") {
      return "`@sdkwork/im-sdk` / `sdk.rooms.enter(...)`";
    }
    if (routeSuffix(route) === "/chat/rooms/{roomId}/leave") {
      return "`@sdkwork/im-sdk` / `sdk.rooms.leave(...)`";
    }
  }

  if (page === "im/streams") {
    if (isRoute(route, "/streams")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.open(...)`";
    }
    if (route.endsWith("/frames") && method === "GET") {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.listStreamFrames(...)`";
    }
    if (route.endsWith("/frames")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.appendStreamFrame(...)`";
    }
    if (route.endsWith("/checkpoint")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.checkpoint(...)`";
    }
    if (route.endsWith("/complete")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.complete(...)`";
    }
    if (route.endsWith("/abort")) {
      return "`@sdkwork/im-sdk` / `sdk.generated.stream.abort(...)`";
    }
  }

  if (!(page in labels)) {
    throw new Error(`Unconfigured SDK label mapping for ${page}.`);
  }

  return labels[page];
}

function securityLabel(route, page) {
  if (route === "/healthz" || route === "/readyz") {
    return "Open endpoint";
  }

  return "SDKWork dual token + AppContext";
}

function permissionLabel(page, method, route) {
  if (route === "/healthz" || route === "/readyz") {
    return "Not required";
  }

  switch (page) {
    case "im/session-and-realtime":
      if (route.endsWith("/ws")) {
        return "Authenticated principal; active client route is prepared before upgrade.";
      }
      return "Authenticated principal; Client route ownership and client route binding are enforced where required.";
    case "im/conversations":
      if (isRoute(route, "/chat/inbox") || isRoute(route, "/inbox")) {
        return "Authenticated principal.";
      }
      if (
        isRoute(route, "/chat/conversations") ||
        isRoute(route, "/chat/conversations/agent_dialogs") ||
        isRoute(route, "/chat/conversations/agent_handoffs") ||
        isRoute(route, "/chat/conversations/system_channels")
      ) {
        return "Authenticated principal.";
      }
      if (route.endsWith("/agent-handoff/accept")) {
        return "Active conversation member with `handoff.accept` authority.";
      }
      if (route.endsWith("/agent-handoff/resolve")) {
        return "Active conversation member with `handoff.resolve` authority.";
      }
      if (route.endsWith("/agent-handoff/close")) {
        return "Active conversation member with `handoff.close` authority.";
      }
      return "Active conversation member.";
    case "im/membership-and-read-state":
      if (method === "GET" || route.endsWith("/members/leave")) {
        return "Active conversation member.";
      }
      return "Conversation-bound write access.";
    case "im/messages":
      return method === "GET"
        ? "Active conversation member."
        : "Conversation-bound write access.";
    case "im/rooms":
      if (method === "GET") {
        return "Authenticated principal.";
      }
      if (routeSuffix(route) === "/chat/rooms/{roomId}/leave") {
        return "Active room member.";
      }
      if (routeSuffix(route) === "/chat/rooms/{roomId}/enter") {
        return "Authenticated principal; room capacity and enter policy are enforced.";
      }
      return "Authenticated principal.";
    case "im/media":
      if (route.endsWith("/attach")) {
        return "Authenticated principal with media asset ownership and target conversation write access.";
      }
      return "Authenticated principal with media asset ownership checks.";
    case "im/streams":
      if (isRoute(route, "/streams")) {
        return "Conversation `stream.open` capability.";
      }
      if (method === "GET") {
        return "Conversation member or stream owner scope.";
      }
      if (route.endsWith("/checkpoint")) {
        return "Conversation `stream.checkpoint` capability.";
      }
      if (route.endsWith("/complete")) {
        return "Conversation `stream.complete` capability.";
      }
      if (route.endsWith("/abort")) {
        return "Conversation `stream.abort` capability.";
      }
      return "Conversation `stream.append` capability.";
    case "im/calls":
      if (isRoute(route, "/calls/sessions")) {
        return "Conversation `call.create` capability when the session is bound to a conversation.";
      }
      if (route.endsWith("/invite")) {
        return "Conversation `call.invite` capability.";
      }
      if (route.endsWith("/accept")) {
        return "Conversation `call.accept` capability.";
      }
      if (route.endsWith("/reject")) {
        return "Conversation `call.reject` capability.";
      }
      if (route.endsWith("/end")) {
        return "Conversation `call.end` capability.";
      }
      if (route.endsWith("/signals")) {
        return "Conversation `call.signal` capability.";
      }
      if (route.endsWith("/credentials")) {
        return "Conversation `call.issue_credential` capability.";
      }
      return "Authenticated principal; conversation call scope is validated by IM.";
    case "app/notifications":
      return method === "POST"
        ? "Own recipient scope or `notification.write` for delegated sends."
        : "Current recipient scope.";
    case "app/automation":
      return method === "POST" ? "`automation.execute`" : "`automation.read`";
    case "backend/audit":
      return method === "POST" ? "`audit.write`" : "`audit.read`";
    case "backend/ops":
      return "`ops.read`";
    case "app/provider-health":
      return "Authenticated principal.";
    case "control-plane/protocol":
    case "control-plane/providers":
    case "control-plane/social":
    case "control-plane/social-runtime":
    case "control-plane/nodes":
      return method === "GET"
        ? "`control.read` or `control.write`"
        : "`control.write`";
    default:
      return "Authenticated principal.";
  }
}

function metaGrid(method, route, block, filePath) {
  const page = pageKey(filePath);
  const cards = [
    ["Security", securityLabel(route, page)],
    ["SDK", sdkLabel(page, route, method)],
    ["Permission", permissionLabel(page, method, route)],
    ["Success", `\`${inferSuccessLabel(method, route, block)}\``],
  ];

  return [
    '<div class="api-meta-grid">',
    ...cards.map(
      ([label, value]) =>
        `  <div class="api-meta-card"><strong>${label}</strong><span>${value}</span></div>`,
    ),
    "</div>",
  ].join("\n");
}

function appReadErrors() {
  return [
    ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
    [
      "403",
      "`conversation_permission_denied`, `permission_denied`",
      "The caller is not allowed to access the target resource.",
    ],
    ["404", "`*_not_found`", "The requested resource does not exist."],
    [
      "409",
      "`reconnect_required`, `disconnect_fence_conflict`, `conflict`",
      "Current runtime state blocks the read or handshake flow.",
    ],
    ["503", "`*_unavailable`", "A required subsystem or provider is unavailable."],
  ];
}

function appWriteErrors() {
  return [
    ["400", "`invalid_request`, `validation_error`", "The request payload or parameters are invalid."],
    ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
    [
      "403",
      "`conversation_permission_denied`, `permission_denied`",
      "The caller is not allowed to mutate the target resource.",
    ],
    ["404", "`*_not_found`", "The requested resource does not exist."],
    [
      "409",
      "`reconnect_required`, `disconnect_fence_conflict`, `conflict`",
      "Current runtime state blocks the mutation.",
    ],
    ["503", "`*_unavailable`", "A required subsystem or provider is unavailable."],
  ];
}

function errorRows(page, method, route) {
  if (route === "/healthz" || route === "/readyz") {
    return [];
  }

  switch (page) {
    case "backend/ops":
      return [
        ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
        ["403", "`permission_denied`", "The caller lacks `ops.read`."],
        ["503", "`*_unavailable`", "Operational diagnostics are temporarily unavailable."],
      ];
    case "backend/audit":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The audit anchor payload is invalid."],
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks `audit.write`."],
          ]
        : [
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks `audit.read`."],
          ];
    case "app/automation":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The automation execution request is invalid."],
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks `automation.execute`."],
            ["409", "`automation_execution_conflict`", "The execution id conflicts with an existing request."],
            ["503", "`automation_store_unavailable`, `journal_unavailable`", "Automation persistence is unavailable."],
          ]
        : [
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks `automation.read`."],
            ["404", "`automation_execution_not_found`", "The requested automation execution does not exist."],
            ["503", "`automation_store_unavailable`", "Automation persistence is unavailable."],
          ];
    case "app/notifications":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The notification request is invalid."],
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks delegated notification authority."],
            ["409", "`notification_conflict`", "The idempotent notification request conflicts with existing state."],
          ]
        : [
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller is not allowed to read the target notification scope."],
            ["404", "`notification_not_found`", "The requested notification task does not exist."],
          ];
    case "app/provider-health":
      return [
        ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
        ["503", "`*_unavailable`", "The provider health source is unavailable."],
      ];
    case "control-plane/protocol":
    case "control-plane/providers":
    case "control-plane/social":
    case "control-plane/social-runtime":
    case "control-plane/nodes":
      return method === "GET"
        ? [
            ["400", "`invalid_request`", "Query or path parameters are invalid."],
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks the required control-plane permission."],
            ["404", "`*_not_found`", "The requested control-plane resource does not exist."],
            ["409", "`*_conflict`", "Current control-plane state blocks the read."],
            ["503", "`*_unavailable`", "The governance snapshot or provider runtime is unavailable."],
          ]
        : [
            ["400", "`invalid_request`, `invalid_provider_policy`", "The mutation payload is invalid."],
            ["401", "`app_context_missing`, `app_context_invalid`", "AppContext projection is missing or invalid."],
            ["403", "`permission_denied`", "The caller lacks `control.write`."],
            ["404", "`*_not_found`, `provider_plugin_not_found`", "The requested node, plugin, or target resource does not exist."],
            ["409", "`*_conflict`, `provider_policy_conflict`", "Current control-plane state blocks the mutation."],
            ["503", "`*_unavailable`", "The governance snapshot or provider runtime is unavailable."],
          ];
    default:
      return method === "GET" ? appReadErrors() : appWriteErrors();
  }
}

function buildErrorResponses(method, route, filePath) {
  const rows = errorRows(pageKey(filePath), method, route);
  if (!rows.length) {
    return "";
  }

  return `### Error Responses

| HTTP | \`code\` | Description |
| --- | --- | --- |
${rows.map(([status, code, description]) => `| \`${status}\` | ${code} | ${description} |`).join("\n")}`;
}

function insertOrReplaceMetaGrid(block, meta) {
  if (/\n<div class="api-meta-grid">[\s\S]*?(?=\n### )/.test(block)) {
    return block.replace(/\n<div class="api-meta-grid">[\s\S]*?(?=\n### )/, `\n${meta}\n`);
  }

  return block.replace(/\n### /, `\n\n${meta}\n\n### `);
}

function ensureNoBodyRequestSection(block) {
  const mentionsNoJsonBody =
    /No JSON request body is required/i.test(block) ||
    /does not accept a JSON request body/i.test(block) ||
    /does not require a JSON request body/i.test(block);

  if (!mentionsNoJsonBody || block.includes("### Request Body")) {
    return block;
  }

  return block.replace(
    /\n### Response /,
    `\n### Request Body\n\nNone. This operation does not accept a JSON request body.\n\n### Response `,
  );
}

function normalizeBlock(block, filePath) {
  const titleMatch = block.match(/## `([A-Z]+) ([^`]+)`/);
  if (!titleMatch) {
    return block;
  }

  const [, method, route] = titleMatch;
  let nextBlock = block;
  const meta = metaGrid(method, route, nextBlock, filePath);

  nextBlock = insertOrReplaceMetaGrid(nextBlock, meta);

  if (method === "POST") {
    nextBlock = ensureNoBodyRequestSection(nextBlock);
  }

  if (!nextBlock.includes("### Error Responses")) {
    const errors = buildErrorResponses(method, route, filePath);
    if (errors) {
      nextBlock = nextBlock.replace(/\n<\/section>\s*$/, `\n\n${errors}\n\n</section>\n`);
    }
  }

  return nextBlock;
}

walk(apiRoot);

const blockPattern =
  /<a id="([^"]+)"><\/a>[\s\S]*?<section class="api-op">[\s\S]*?(?=<a id="|$)/g;

for (const filePath of markdownFiles) {
  const original = fs.readFileSync(filePath, "utf8");
  const updated = original.replace(blockPattern, (block) => normalizeBlock(block, filePath));
  if (updated !== original) {
    fs.writeFileSync(filePath, updated);
  }
}

console.log(`Standardized ${markdownFiles.length} API markdown files.`);
