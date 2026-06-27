import { isSdkworkChatDesktopRuntime } from '../runtime/desktopEnvironment';

type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

function resolveTauriInvoke(): TauriInvoke | undefined {
  const invoke = (globalThis as {
    __TAURI__?: {
      core?: {
        invoke?: TauriInvoke;
      };
    };
  }).__TAURI__?.core?.invoke;

  return typeof invoke === 'function' ? invoke : undefined;
}

export function isDesktopSecureSessionStorageEnabled(): boolean {
  return isSdkworkChatDesktopRuntime() && Boolean(resolveTauriInvoke());
}

export async function readDesktopSecureSessionRawValue(): Promise<string | null> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return null;
  }

  const value = await invoke('sdkwork_im_pc_session_read');
  return typeof value === 'string' && value.trim().length > 0 ? value : null;
}

export async function writeDesktopSecureSessionRawValue(value: string): Promise<void> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    throw new Error('desktop secure session storage is unavailable');
  }

  await invoke('sdkwork_im_pc_session_write', { value });
}

export async function clearDesktopSecureSessionRawValue(): Promise<void> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return;
  }

  await invoke('sdkwork_im_pc_session_clear');
}
