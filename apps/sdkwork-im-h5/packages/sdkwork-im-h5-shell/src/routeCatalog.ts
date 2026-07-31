import type { ImH5RouteMetadata } from "@sdkwork/im-h5-core/routes";

function defineRoute(
  route: Omit<ImH5RouteMetadata, "surface" | "auth" | "presentation"> & {
    readonly presentation: ImH5RouteMetadata["presentation"]["h5Mobile"];
  },
): ImH5RouteMetadata {
  const { presentation, ...metadata } = route;
  return {
    ...metadata,
    surface: "app",
    auth: "required",
    presentation: { h5Mobile: presentation },
  };
}

export const IM_H5_ROUTE_DEFINITIONS = {
  chatInbox: defineRoute({ id: "app.communication.chat.inbox", legacyRouteId: "im-h5-chat-inbox", moduleId: "chat", domain: "communication", capability: "chat", screen: "inbox", path: "/", titleKey: "common.tabs.chat", layoutGroup: "main", presentation: "tab" }),
  chatConversation: defineRoute({ id: "app.communication.chat.conversation", legacyRouteId: "im-h5-chat-conversation", moduleId: "chat", domain: "communication", capability: "chat", screen: "conversation", path: "/chat/:conversationId", titleKey: "chat.conversation.title", presentation: "stack" }),
  notaryWorkspace: defineRoute({ id: "app.notary.workspace.index", legacyRouteId: "im-h5-workspace", moduleId: "notary", domain: "notary", capability: "workspace", screen: "index", path: "/workspace", titleKey: "common.tabs.workspace", layoutGroup: "main", presentation: "tab" }),
  notaryWorkspaceDetail: defineRoute({ id: "app.notary.workspace.notary", legacyRouteId: "im-h5-workspace-notary", moduleId: "notary", domain: "notary", capability: "workspace", screen: "notary", path: "/workspace/notary", titleKey: "notary.workspace.title", presentation: "stack" }),
  notaryRecords: defineRoute({ id: "app.notary.records.index", legacyRouteId: "im-h5-notary-records", moduleId: "notary", domain: "notary", capability: "records", screen: "index", path: "/notary", titleKey: "notary.records.title", layoutGroup: "notary", presentation: "tab" }),
  notaryRecordsList: defineRoute({ id: "app.notary.records.list", moduleId: "notary", domain: "notary", capability: "records", screen: "list", path: "/notary", titleKey: "notary.records.title", layoutGroup: "notary", presentation: "tab" }),
  notaryFiles: defineRoute({ id: "app.notary.files.list", legacyRouteId: "im-h5-notary-files", moduleId: "notary", domain: "notary", capability: "files", screen: "list", path: "/notary/files", titleKey: "notary.files.title", layoutGroup: "notary", presentation: "tab" }),
  notaryMessages: defineRoute({ id: "app.notary.messages.list", legacyRouteId: "im-h5-notary-messages", moduleId: "notary", domain: "notary", capability: "messages", screen: "list", path: "/notary/messages", titleKey: "notary.messages.title", layoutGroup: "notary", presentation: "tab" }),
  notaryAccount: defineRoute({ id: "app.notary.account.index", legacyRouteId: "im-h5-notary-me", moduleId: "notary", domain: "notary", capability: "account", screen: "index", path: "/notary/me", titleKey: "notary.me.title", layoutGroup: "notary", presentation: "tab" }),
  notaryCreate: defineRoute({ id: "app.notary.cases.create", legacyRouteId: "im-h5-notary-create", moduleId: "notary", domain: "notary", capability: "cases", screen: "create", path: "/notary/create", titleKey: "notary.create.title", presentation: "stack" }),
  notarySearch: defineRoute({ id: "app.notary.cases.search", legacyRouteId: "im-h5-notary-search", moduleId: "notary", domain: "notary", capability: "cases", screen: "search", path: "/notary/search", titleKey: "notary.search.title", presentation: "stack" }),
  notaryAddParty: defineRoute({ id: "app.notary.parties.create", legacyRouteId: "im-h5-notary-add-party", moduleId: "notary", domain: "notary", capability: "parties", screen: "create", path: "/notary/add-party", titleKey: "notary.parties.addTitle", presentation: "stack" }),
  notaryDetail: defineRoute({ id: "app.notary.cases.detail", legacyRouteId: "im-h5-notary-detail", moduleId: "notary", domain: "notary", capability: "cases", screen: "detail", path: "/notary/detail/:id", titleKey: "notary.detail.title", presentation: "stack" }),
  notaryMessageDetail: defineRoute({ id: "app.notary.messages.detail", legacyRouteId: "im-h5-notary-message-detail", moduleId: "notary", domain: "notary", capability: "messages", screen: "detail", path: "/notary/messages/:messageId", titleKey: "notary.messages.detailTitle", presentation: "stack" }),
  notarySessionChat: defineRoute({ id: "app.notary.session.chat", legacyRouteId: "im-h5-notary-session-chat", moduleId: "notary", domain: "notary", capability: "session", screen: "chat", path: "/notary/chat/:caseId", titleKey: "notary.session.chatTitle", presentation: "stack" }),
  notaryPartySignature: defineRoute({ id: "app.notary.parties.signature", legacyRouteId: "im-h5-notary-party-signature", moduleId: "notary", domain: "notary", capability: "parties", screen: "signature", path: "/notary/cases/:caseId/parties/:partyId/signature", titleKey: "notary.parties.signatureTitle", presentation: "stack" }),
  notaryPartyVideo: defineRoute({ id: "app.notary.parties.video", legacyRouteId: "im-h5-notary-party-video", moduleId: "notary", domain: "notary", capability: "parties", screen: "video", path: "/notary/cases/:caseId/parties/:partyId/video", titleKey: "notary.parties.videoTitle", presentation: "stack" }),
  notaryPartyVideoQr: defineRoute({ id: "app.notary.parties.video-qr", legacyRouteId: "im-h5-notary-party-video-qr", moduleId: "notary", domain: "notary", capability: "parties", screen: "video-qr", path: "/notary/cases/:caseId/parties/:partyId/video-qr", titleKey: "notary.parties.videoQrTitle", presentation: "stack" }),
} as const;

export const IM_H5_APP_ROUTE_METADATA: readonly ImH5RouteMetadata[] = Object.values(
  IM_H5_ROUTE_DEFINITIONS,
).filter((route, index, routes) =>
  routes.findIndex((candidate) => candidate.path === route.path) === index,
);
