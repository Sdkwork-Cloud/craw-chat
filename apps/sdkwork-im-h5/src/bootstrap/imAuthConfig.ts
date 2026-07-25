import type {
  SdkworkAuthAppearanceConfig,
  SdkworkAuthRuntimeConfig,
} from '@sdkwork/auth-pc-react';

const IM_H5_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: false,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: false,
  phoneRegistrationVerificationRequired: false,
} as const;

export function resolveImAuthRuntimeConfig(): SdkworkAuthRuntimeConfig {
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: [],
    registerMethods: ['email', 'phone'],
    verificationPolicy: IM_H5_VERIFICATION_POLICY,
  };
}

export function resolveImAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-im-h5-auth-aside-panel',
    bodyClassName: 'sdkwork-im-h5-auth-body',
    contentContainerClassName: 'sdkwork-im-h5-auth-content',
    pageClassName: 'sdkwork-im-h5-auth-page',
    qrFrameClassName: 'sdkwork-im-h5-auth-qr-frame',
    shellClassName: 'sdkwork-im-h5-auth-card-shell',
    slotProps: {
      background: {
        className: 'sdkwork-im-h5-auth-background',
      },
      page: {
        className: 'sdkwork-im-h5-auth-page',
      },
      shell: {
        className: 'sdkwork-im-h5-auth-card-shell',
      },
    },
    theme: {
      asideCardBackgroundColor: 'var(--sdkwork-im-h5-auth-aside-card-bg)',
      asideCardBorderColor: 'var(--sdkwork-im-h5-auth-aside-card-border)',
      asidePanelBackgroundColor: 'var(--sdkwork-im-h5-auth-aside-bg)',
      asidePanelBorderColor: 'var(--sdkwork-im-h5-auth-aside-border)',
      asidePanelColor: 'var(--sdkwork-im-h5-auth-aside-text)',
      badgeBackgroundColor: 'var(--sdkwork-im-h5-auth-aside-badge-bg)',
      badgeTextColor: 'var(--sdkwork-im-h5-auth-aside-badge-text)',
      contentBackgroundColor: 'var(--sdkwork-im-h5-auth-content-bg)',
      contentBorderColor: 'transparent',
      contentTextColor: 'var(--sdkwork-im-h5-auth-content-text)',
      descriptionColor: 'var(--sdkwork-im-h5-auth-muted-text)',
      dividerColor: 'var(--sdkwork-im-h5-auth-divider)',
      fieldBackgroundColor: 'var(--sdkwork-im-h5-auth-field-bg)',
      fieldBorderColor: 'transparent',
      fieldPlaceholderColor: '#9ca3af',
      fieldTextColor: 'var(--sdkwork-im-h5-auth-content-text)',
      formMutedTextColor: 'var(--sdkwork-im-h5-auth-muted-text)',
      iconMutedColor: 'var(--sdkwork-im-h5-auth-muted-text)',
      labelColor: 'var(--sdkwork-im-h5-auth-content-text)',
      pageBackgroundColor: 'var(--sdkwork-im-h5-auth-bg)',
      qrFrameBackgroundColor: 'var(--sdkwork-im-h5-auth-qr-bg)',
      qrFrameBorderColor: 'transparent',
      shellBackgroundColor: 'var(--sdkwork-im-h5-auth-content-bg)',
      shellBorderColor: 'transparent',
      tabActiveBackgroundColor: 'transparent',
      tabActiveTextColor: 'var(--sdkwork-im-h5-auth-content-text)',
      tabBackgroundColor: 'transparent',
      tabInactiveTextColor: 'var(--sdkwork-im-h5-auth-muted-text)',
      titleColor: 'var(--sdkwork-im-h5-auth-content-text)',
    },
  };
}
