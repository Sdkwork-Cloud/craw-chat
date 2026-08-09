import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "HardwareList" | "HardwareBind" | "HardwareDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/aiot-mobile-react-hardware");
    return { default: mod[name] };
  });
}

const HardwareList = lazyComponent("HardwareList");
const HardwareBind = lazyComponent("HardwareBind");
const HardwareDetail = lazyComponent("HardwareDetail");

export const devicesModule: ImH5CapabilityModule = {
  id: "devices",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.hardwareList, render: () => <HardwareList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.hardwareBind, render: () => <HardwareBind /> },
    { ...IM_H5_ROUTE_DEFINITIONS.hardwareDetail, render: () => <HardwareDetail /> },
  ],
};
