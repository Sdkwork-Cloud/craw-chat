import { useEffect } from "react";

import { registerImH5SessionChangeListener } from "@sdkwork/im-h5-core/session";

import {
  invalidateChatLiveConnection,
  subscribeInboxLiveRefresh,
} from "./services/chatRealtimeService";

export function ChatLifecycle() {
  useEffect(() => {
    // 本组件仅在已认证（AuthGate 渲染 children）后挂载：持有 inbox 级租约
    // 使共享实时连接拥有常驻 demand，断线后 core 的退避重连机制才会调度
    // 重连。仅调用 ensureImLiveConnection 没有租约，连接一旦断开 core 判定
    // 无 demand 会直接 teardown 且不再重连。
    // The no-op view observer keeps the handler set non-empty; page-level
    // subscribers (ChatList) already reload their data on reconnect.
    const unsubscribe = subscribeInboxLiveRefresh(() => {
      // no-op: the lease itself is what keeps the connection demanded.
    });
    return unsubscribe;
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
