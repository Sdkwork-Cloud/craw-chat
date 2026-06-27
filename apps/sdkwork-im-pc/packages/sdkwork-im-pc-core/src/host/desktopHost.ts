/**
 * Desktop/Tauri host adapter — isolates native runtime globals from UI packages.
 */

export type TauriUnlisten = () => void;
export type TauriListen = (event: string, handler: () => void) => Promise<TauriUnlisten>;

export function resolveTauriListen(): TauriListen | null {
  return (globalThis as {
    __TAURI__?: {
      event?: {
        listen?: TauriListen;
      };
    };
  }).__TAURI__?.event?.listen ?? null;
}

export function isDesktopHostRuntime(): boolean {
  return resolveTauriListen() !== null;
}
