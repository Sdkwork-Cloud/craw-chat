import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "CalendarWorkspace";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-calendar");
    return { default: mod[name] };
  });
}

const CalendarWorkspace = lazyComponent("CalendarWorkspace");

export const calendarModule: ImH5CapabilityModule = {
  id: "calendar",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.calendarWorkspace, render: () => <CalendarWorkspace /> },
  ],
};
