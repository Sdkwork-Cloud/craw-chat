import React, { type ReactNode } from "react";
import { Navigate, Route, Routes, useParams } from "react-router-dom";
import {
  ChatConversationPage,
  ChatInboxPage,
} from "@sdkwork/im-h5-chat";
import {
  CreateNotaryProcess,
  NotaryAddParty,
  NotaryDetail,
  NotaryDraftLifecycle,
  NotaryFiles,
  NotaryLayout,
  NotaryMe,
  NotaryMessageDetail,
  NotaryMessages,
  NotaryPartySignature,
  NotaryPartyVideoQR,
  NotaryRecords,
  NotarySearchList,
  NotarySessionChat,
  NotaryVideoCall,
  WorkspaceNotary,
} from "@sdkwork/im-h5-notary";

import { TabBar } from "./components/navigation/TabBar";

export interface ImAppProps {
  children?: ReactNode;
}

export interface ParsedConversationRoute {
  conversationId: string;
  conversationPath: string;
}

export const IM_APP_HOME_PATH = "/";

export function parseConversationRoute(pathname: string): ParsedConversationRoute | null {
  const match = /^\/chat\/([^/?#]+)/u.exec(pathname.trim());
  if (!match?.[1]) {
    return null;
  }
  const conversationId = decodeURIComponent(match[1]);
  return conversationId
    ? { conversationId, conversationPath: "/chat/" + conversationId }
    : null;
}

export function ImApp({ children }: ImAppProps) {
  return (
    <>
      <NotaryDraftLifecycle />
      <Routes>
      <Route
        path="/"
        element={(
          <MainShell>
            <ChatInboxPage />
          </MainShell>
        )}
      />
      <Route path="/chat/:conversationId" element={<ConversationRoute />} />
      <Route
        path="/workspace"
        element={(
          <MainShell>
            <WorkspaceNotary />
          </MainShell>
        )}
      />
      <Route path="/workspace/notary" element={<WorkspaceNotary />} />

      <Route path="/notary" element={<NotaryLayout />}>
        <Route index element={<NotaryRecords />} />
        <Route path="files" element={<NotaryFiles />} />
        <Route path="messages" element={<NotaryMessages />} />
        <Route path="me" element={<NotaryMe />} />
      </Route>
      <Route path="/notary/create" element={<CreateNotaryProcess />} />
      <Route path="/notary/search" element={<NotarySearchList />} />
      <Route path="/notary/add-party" element={<NotaryAddParty />} />
      <Route path="/notary/detail/:id" element={<NotaryDetail />} />
      <Route path="/notary/messages/:messageId" element={<NotaryMessageDetail />} />
      <Route path="/notary/chat/:caseId" element={<NotarySessionChat />} />
      <Route
        path="/notary/cases/:caseId/parties/:partyId/signature"
        element={<NotaryPartySignature />}
      />
      <Route
        path="/notary/cases/:caseId/parties/:partyId/video"
        element={<NotaryVideoCall />}
      />
      <Route
        path="/notary/cases/:caseId/parties/:partyId/video-qr"
        element={<NotaryPartyVideoQR />}
      />

      {children}
      <Route path="*" element={<Navigate to={IM_APP_HOME_PATH} replace />} />
      </Routes>
    </>
  );
}

function MainShell({ children }: { children: ReactNode }) {
  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-bg-color">
      <div className="min-h-0 flex-1">{children}</div>
      <TabBar />
    </div>
  );
}

function ConversationRoute() {
  const { conversationId } = useParams();
  if (!conversationId) {
    return <Navigate to={IM_APP_HOME_PATH} replace />;
  }
  return <ChatConversationPage conversationId={conversationId} />;
}

export default ImApp;
