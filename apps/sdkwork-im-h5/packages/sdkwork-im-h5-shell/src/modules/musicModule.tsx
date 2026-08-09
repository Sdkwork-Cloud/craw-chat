import React from "react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "MusicPlayerPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/music-mobile-react-playback");
    return { default: mod[name] };
  });
}

const MusicPlayerPage = lazyComponent("MusicPlayerPage");

export const musicModule: ImH5CapabilityModule = {
  id: "music",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.musicPlayer, render: () => <MusicPlayerPage /> },
  ],
};
