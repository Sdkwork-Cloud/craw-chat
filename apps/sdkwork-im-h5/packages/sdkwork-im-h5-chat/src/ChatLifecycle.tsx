import { useEffect } from "react";

import { registerImH5SessionChangeListener } from "@sdkwork/im-h5-core/session";

import {
  ensureChatLiveConnection,
  invalidateChatLiveConnection,
} from "./services/chatRealtimeService";

export function ChatLifecycle() {
  useEffect(() => {
    // 本组件仅在已认证（AuthGate 渲染 children）后挂载：主动建立常驻
    // 实时连接，不依赖任何页面的懒订阅；失败由 core 的退避重连兜底。
    void ensureChatLiveConnection().catch(() => {
      // fire-and-forget：重连由 core/realtime 的恢复机制接管。
    });
  }, []);

  useEffect(
    () =>
      registerImH5SessionChangeListener(() => {
        // 会话切换（登录成功 / token 刷新 / 重登 / 登出）：失效旧连接。
        // 已认证时 core 保留注册并立即重建；登出时认证 provider 判定为
        // 非活跃，清空注册并断开。
        invalidateChatLiveConnection();
      }),
    [],
  );
  return null;
}
