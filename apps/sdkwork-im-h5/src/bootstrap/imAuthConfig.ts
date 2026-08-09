import type { SdkworkAuthRuntimeConfig } from '@sdkwork/auth-pc-react';

export interface ImAuthRuntimeConfigOptions {
  qrLoginEnabled?: boolean;
  passwordLoginEnabled?: boolean;
  emailLoginEnabled?: boolean;
  phoneLoginEnabled?: boolean;
}

const SDKWORK_IM_H5_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: false,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: false,
  phoneRegistrationVerificationRequired: false,
} as const;

function readEnvValue(...keys: string[]): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  };

  for (const key of keys) {
    const value = meta.env?.[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }

  return undefined;
}

function resolveDevelopmentPrefill(): SdkworkAuthRuntimeConfig['developmentPrefill'] {
  const account = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_ACCOUNT',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_ACCOUNT',
  );
  const email = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_EMAIL',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_EMAIL',
  );
  const phone = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_PHONE',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PHONE',
  );
  const password = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_PASSWORD',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PASSWORD',
  );

  const enabled = Boolean(account || email || phone || password);

  if (!enabled) {
    return undefined;
  }

  return {
    account: account || email || phone,
    email,
    enabled: true,
    loginMethod: 'password',
    password,
    phone,
  };
}

export function resolveImAuthRuntimeConfig(
  options: ImAuthRuntimeConfigOptions = {},
): SdkworkAuthRuntimeConfig {
  const developmentPrefill = resolveDevelopmentPrefill();
  const loginMethods: SdkworkAuthRuntimeConfig['loginMethods'] = ['password'];
  if (options.phoneLoginEnabled ?? true) {
    loginMethods.push('phoneCode');
  }
  if (options.emailLoginEnabled ?? true) {
    loginMethods.push('emailCode');
  }
  return {
    loginMethods,
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: options.qrLoginEnabled ?? false,
    recoveryMethods: ['phone', 'email'],
    registerMethods: ['phone', 'email'],
    verificationPolicy: SDKWORK_IM_H5_VERIFICATION_POLICY,
    ...(developmentPrefill ? { developmentPrefill } : {}),
  };
}
