import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "AttendanceApp";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-attendance");
    return { default: mod[name] };
  });
}

const AttendanceApp = lazyComponent("AttendanceApp");

export const attendanceModule: ImH5CapabilityModule = {
  id: "attendance",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceAttendance, render: () => <AttendanceApp /> },
  ],
};
