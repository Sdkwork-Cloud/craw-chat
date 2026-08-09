import {
  createSdkworkIamH5AuthController,
  type SdkworkIamH5AuthController,
} from '@sdkwork/iam-h5-auth';
import { getSdkworkChatIamRuntime } from '@sdkwork/im-pc-core';
import type { SdkworkIamService } from '@sdkwork/iam-service';

/**
 * Builds the mobile auth controller over the PC app's IAM runtime service.
 *
 * Mobile browsers landing on the PC app login are served the mobile
 * login/register surface (zip-design). Successful sessions flow through the
 * same appbase runtime: `service.auth.sessions.create` commits the session via
 * the PC session bridge and dispatches `SDKWORK_IM_SESSION_CHANGED_EVENT`,
 * which unlocks AuthGate — no separate session wiring is needed.
 */
export function createImPcMobileAuthController(): SdkworkIamH5AuthController {
  const runtime = getSdkworkChatIamRuntime();
  return createSdkworkIamH5AuthController({
    service: runtime.service as unknown as SdkworkIamService,
  });
}
