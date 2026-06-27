type SdkworkChatTauriBridge = {
  __TAURI__?: {
    core?: {
      invoke?: unknown;
    };
    window?: {
      appWindow?: unknown;
      getCurrentWindow?: unknown;
    };
  };
};

export function isSdkworkChatDesktopRuntime(): boolean {
  const tauri = (globalThis as SdkworkChatTauriBridge).__TAURI__;

  return Boolean(
    typeof tauri?.core?.invoke === 'function'
      || typeof tauri?.window?.getCurrentWindow === 'function'
      || tauri?.window?.appWindow,
  );
}
