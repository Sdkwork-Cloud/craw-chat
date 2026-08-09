import React from "react";

import { NotaryDraftLifecycle } from "@sdkwork/im-h5-notary";

import type { ImH5CapabilityModule, ImH5RouteContribution } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type NotaryComponentName =
  | "CreateNotaryProcess"
  | "NotaryAddParty"
  | "NotaryDetail"
  | "NotaryFiles"
  | "NotaryLayout"
  | "NotaryMe"
  | "NotaryMessageDetail"
  | "NotaryMessages"
  | "NotaryPartySignature"
  | "NotaryPartyVideoQR"
  | "NotaryRecords"
  | "NotarySearchList"
  | "NotarySessionChat"
  | "NotaryVideoCall"
  | "WorkspaceNotary";

function lazyNotaryComponent(name: NotaryComponentName) {
  return React.lazy(async () => {
    const notaryModule = await import("@sdkwork/im-h5-notary");
    return { default: notaryModule[name] };
  });
}

const CreateNotaryProcess = lazyNotaryComponent("CreateNotaryProcess");
const NotaryAddParty = lazyNotaryComponent("NotaryAddParty");
const NotaryDetail = lazyNotaryComponent("NotaryDetail");
const NotaryFiles = lazyNotaryComponent("NotaryFiles");
const NotaryLayout = lazyNotaryComponent("NotaryLayout");
const NotaryMe = lazyNotaryComponent("NotaryMe");
const NotaryMessageDetail = lazyNotaryComponent("NotaryMessageDetail");
const NotaryMessages = lazyNotaryComponent("NotaryMessages");
const NotaryPartySignature = lazyNotaryComponent("NotaryPartySignature");
const NotaryPartyVideoQR = lazyNotaryComponent("NotaryPartyVideoQR");
const NotaryRecords = lazyNotaryComponent("NotaryRecords");
const NotarySearchList = lazyNotaryComponent("NotarySearchList");
const NotarySessionChat = lazyNotaryComponent("NotarySessionChat");
const NotaryVideoCall = lazyNotaryComponent("NotaryVideoCall");
const WorkspaceNotary = lazyNotaryComponent("WorkspaceNotary");

function nestedRoute(
  metadata: ImH5RouteContribution,
  relativePath: string,
): ImH5RouteContribution {
  return { ...metadata, relativePath };
}

export const notaryModule: ImH5CapabilityModule = {
  id: "notary",
  lifecycle: NotaryDraftLifecycle,
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.notaryWorkspace, render: () => <WorkspaceNotary /> },
    {
      ...IM_H5_ROUTE_DEFINITIONS.notaryRecords,
      render: () => <NotaryLayout />,
      children: [
        { ...IM_H5_ROUTE_DEFINITIONS.notaryRecordsList, index: true, render: () => <NotaryRecords /> },
        nestedRoute({ ...IM_H5_ROUTE_DEFINITIONS.notaryFiles, render: () => <NotaryFiles /> }, "files"),
        nestedRoute({ ...IM_H5_ROUTE_DEFINITIONS.notaryMessages, render: () => <NotaryMessages /> }, "messages"),
        nestedRoute({ ...IM_H5_ROUTE_DEFINITIONS.notaryAccount, render: () => <NotaryMe /> }, "me"),
      ],
    },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryCreate, render: () => <CreateNotaryProcess /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notarySearch, render: () => <NotarySearchList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryAddParty, render: () => <NotaryAddParty /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryDetail, render: () => <NotaryDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryMessageDetail, render: () => <NotaryMessageDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notarySessionChat, render: () => <NotarySessionChat /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryPartySignature, render: () => <NotaryPartySignature /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryPartyVideo, render: () => <NotaryVideoCall /> },
    { ...IM_H5_ROUTE_DEFINITIONS.notaryPartyVideoQr, render: () => <NotaryPartyVideoQR /> },
  ],
};
