import { isBlank, trim } from '@sdkwork/utils';

import { isSdkworkChatDesktopRuntime } from '../runtime/desktopEnvironment';

export interface ImDesktopKnowledgeWindowRequest {
  url: string;
  title: string;
  label: string;
}

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

export function isImDesktopKnowledgeWindowHostAvailable(): boolean {
  return isSdkworkChatDesktopRuntime() && Boolean(resolveTauriInvoke());
}

export async function openImDesktopKnowledgeWindow(
  request: ImDesktopKnowledgeWindowRequest,
): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke || isBlank(request.url)) {
    return false;
  }

  await invoke('sdkwork_chat_pc_open_knowledge_window', {
    request: {
      url: trim(request.url),
      title: isBlank(request.title) ? undefined : trim(request.title),
      label: isBlank(request.label) ? undefined : trim(request.label),
    },
  });
  return true;
}
