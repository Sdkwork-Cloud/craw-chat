import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "KnowledgeBaseApp" | "CreateKnowledgeBase" | "KnowledgeBaseDocumentList" | "CreateDocument" | "KnowledgeBaseDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/knowledgebase-mobile-react-knowledge");
    return { default: mod[name] };
  });
}

const KnowledgeBaseApp = lazyComponent("KnowledgeBaseApp");
const CreateKnowledgeBase = lazyComponent("CreateKnowledgeBase");
const KnowledgeBaseDocumentList = lazyComponent("KnowledgeBaseDocumentList");
const CreateDocument = lazyComponent("CreateDocument");
const KnowledgeBaseDetail = lazyComponent("KnowledgeBaseDetail");

export const knowledgeModule: ImH5CapabilityModule = {
  id: "knowledge",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceKnowledge, render: () => <KnowledgeBaseApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceKnowledgeCreate, render: () => <CreateKnowledgeBase /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceKnowledgeDetail, render: () => <KnowledgeBaseDocumentList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceKnowledgeDocCreate, render: () => <CreateDocument /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceKnowledgeDocDetail, render: () => <KnowledgeBaseDetail /> },
  ],
};
