import { useEffect } from "react";

import { registerImH5SessionChangeListener } from "@sdkwork/im-h5-core/session";

import { disposeChatLiveConnection } from "./services/chatRealtimeService";

export function ChatLifecycle() {
  useEffect(
    () => registerImH5SessionChangeListener(disposeChatLiveConnection),
    [],
  );
  return null;
}
