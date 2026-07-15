import { isSdkworkChatDesktopRuntime } from '../runtime/desktopEnvironment';

export interface OpenGroupKnowledgebaseRequest {
  launchTicket: string;
}

const GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_PATTERN = /^gklt_[A-Za-z0-9_-]{43}$/u;

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

export function isImDesktopGroupKnowledgebaseHostAvailable(): boolean {
  return isSdkworkChatDesktopRuntime() && Boolean(resolveTauriInvoke());
}

export function isValidGroupKnowledgebaseLaunchTicket(value: string): boolean {
  return GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_PATTERN.test(value);
}

/**
 * Opens the independently installed Knowledgebase application. The native
 * command deliberately accepts only the opaque launch ticket so the IM
 * renderer cannot choose a destination URL or pass authorization context.
 */
export async function openImDesktopGroupKnowledgebase(
  request: OpenGroupKnowledgebaseRequest,
): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke || !isValidGroupKnowledgebaseLaunchTicket(request.launchTicket)) {
    return false;
  }

  await invoke('sdkwork_chat_pc_open_group_knowledgebase', {
    request: {
      launchTicket: request.launchTicket,
    },
  });
  return true;
}
