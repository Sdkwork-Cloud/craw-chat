import {
  createSdkworkIamH5AuthController,
  type SdkworkIamH5AuthController,
  type SdkworkIamH5VerificationCodeClient,
  type SdkworkIamH5VerifyType,
} from '@sdkwork/iam-h5-auth';
import { i18n, showToast } from '@sdkwork/im-h5-commons';
import { getImAppAuthRuntime } from './iamRuntime';

/**
 * Maps the mobile verify-type to the IAM verification channel vocabulary
 * (`sms`/`email`) expected by `verificationCodeRequests.create`.
 */
function resolveVerificationChannel(verifyType: SdkworkIamH5VerifyType): "sms" | "email" {
  return verifyType === "PHONE" ? "sms" : "email";
}

/**
 * Reads the optional `devCode` echo from the appbase envelope
 * (`{code, data: {accepted, devCode}}`) — present only when the backend runs
 * in dev fixed-code mode.
 */
function readDevCode(response: unknown): string | undefined {
  if (!response || typeof response !== "object") {
    return undefined;
  }
  const record = response as Record<string, unknown>;
  const data = record.data && typeof record.data === "object"
    ? record.data as Record<string, unknown>
    : record;
  const devCode = typeof data.devCode === "string" ? data.devCode.trim() : "";
  return devCode || undefined;
}

/**
 * Verification-code delivery port injected into the H5 auth controller.
 *
 * The request goes through the IAM app-api `verificationCodeRequests.create`
 * endpoint (generated SDK surface — no raw HTTP).
 *
 * DEV-ONLY TOOL (not a business demo): when the backend runs in dev
 * fixed-code mode it echoes the demo code in the envelope; the toast lets a
 * developer complete the login without a real messaging channel. Production
 * deployments never return `devCode`, so this branch is inert there. It is
 * kept strictly gated by the backend echo — the frontend never fabricates or
 * accepts a hard-coded demo code on its own.
 */
const verificationCodeClient: SdkworkIamH5VerificationCodeClient = {
  async send({ scene, target, verifyType }) {
    const runtime = getImAppAuthRuntime().runtime;
    const response = await runtime.service.auth.verificationCodeRequests.create({
      scene,
      target: target.trim(),
      channel: resolveVerificationChannel(verifyType),
    });
    const devCode = readDevCode(response);
    if (devCode) {
      showToast(i18n.t("commons.verification_dev_code", "演示验证码：{{code}}", { code: devCode }));
    }
  },
};

/**
 * Builds the mobile auth controller over the appbase IAM runtime service.
 *
 * Login, registration, code flows and password recovery all go through
 * `IamRuntime.service` (SdkworkIamService); successful sessions are committed
 * by the runtime and surface through `onSessionChanged` →
 * `IM_H5_IAM_SESSION_CHANGED_EVENT`, which unlocks AuthGate without extra
 * wiring. No raw HTTP or manual auth headers are used (APP_H5_ARCHITECTURE_SPEC §7).
 */
export function createImH5AuthController(): SdkworkIamH5AuthController {
  const runtime = getImAppAuthRuntime().runtime;
  return createSdkworkIamH5AuthController({
    service: runtime.service,
    verificationCodeClient,
  });
}
