import {
  applyMailIamSessionTokens,
  clearMailIamSessionTokens,
  resetMailAppSdkClient,
  type MailIamSession,
} from '@sdkwork/mail-pc-core';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let mailPcRuntimeBootstrapped = false;
let mailSessionListenerRegistered = false;

function mapImSessionToMailIam(session: SdkworkChatSession): MailIamSession | null {
  if (!session.authToken || !session.accessToken || !session.context?.tenantId || !session.context?.userId) {
    return null;
  }

  return {
    accessToken: session.accessToken,
    authToken: session.authToken,
    refreshToken: session.refreshToken,
    sessionId: session.sessionId ?? session.context.sessionId,
    context: session.context,
    user: session.user
      ? {
          id: String(session.user.id ?? ''),
          userId: session.user.userId,
          name: session.user.name,
          nickname: session.user.nickname,
          displayName: session.user.displayName,
          email: session.user.email,
        }
      : undefined,
  };
}

export function syncImSessionToMailPc(session = readAppSdkSessionTokens()): void {
  const mailSession = session ? mapImSessionToMailIam(session) : null;
  if (!mailSession) {
    clearMailIamSessionTokens();
    resetMailAppSdkClient();
    return;
  }

  applyMailIamSessionTokens(mailSession);
  resetMailAppSdkClient();
  void resolveAppSdkBaseUrl();
}

export function bootstrapMailPcForIm(): void {
  syncImSessionToMailPc();
  mailPcRuntimeBootstrapped = true;

  if (!mailSessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToMailPc();
    });
    mailSessionListenerRegistered = true;
  }
}

export function isMailPcRuntimeBootstrapped(): boolean {
  return mailPcRuntimeBootstrapped;
}

export function resetMailPcIntegration(): void {
  mailPcRuntimeBootstrapped = false;
  clearMailIamSessionTokens();
  resetMailAppSdkClient();
}
