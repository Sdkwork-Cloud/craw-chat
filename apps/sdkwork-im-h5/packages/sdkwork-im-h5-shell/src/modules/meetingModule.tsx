import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "MeetingApp" | "CreateMeeting" | "MeetingDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/rtc-mobile-react-meeting");
    return { default: mod[name] };
  });
}

const MeetingApp = lazyComponent("MeetingApp");
const CreateMeeting = lazyComponent("CreateMeeting");
const MeetingDetail = lazyComponent("MeetingDetail");

export const meetingModule: ImH5CapabilityModule = {
  id: "meeting",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceMeeting, render: () => <MeetingApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceMeetingCreate, render: () => <CreateMeeting /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceMeetingDetail, render: () => <MeetingDetail /> },
  ],
};
