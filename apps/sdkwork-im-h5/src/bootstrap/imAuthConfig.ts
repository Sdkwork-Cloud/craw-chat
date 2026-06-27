import type {
  SdkworkAuthAppearanceConfig,
  SdkworkAuthRuntimeConfig,
} from "@sdkwork/auth-pc-react";

const IM_H5_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: false,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: false,
  phoneRegistrationVerificationRequired: false,
};

export function resolveImAuthRuntimeConfig(): SdkworkAuthRuntimeConfig {
  return {
    leftRailMode: "qr-only",
    loginMethods: ["password"],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: [],
    registerMethods: ["email", "phone"],
    verificationPolicy: IM_H5_VERIFICATION_POLICY,
  };
}

export function resolveImAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    asidePanelClassName: "sdkwork-im-h5-auth-aside-panel",
    bodyClassName: "sdkwork-im-h5-auth-body",
    contentContainerClassName: "sdkwork-im-h5-auth-content",
    pageClassName: "sdkwork-im-h5-auth-page",
    qrFrameClassName: "sdkwork-im-h5-auth-qr-frame",
    shellClassName: "sdkwork-im-h5-auth-card-shell",
    slotProps: {
      background: {
        className: "sdkwork-im-h5-auth-background",
      },
      page: {
        className: "sdkwork-im-h5-auth-page",
      },
      shell: {
        className: "sdkwork-im-h5-auth-card-shell",
      },
    },
    theme: {
      asideCardBackgroundColor: "#1f2937",
      asideCardBorderColor: "#374151",
      asidePanelBackgroundColor: "#111827",
      asidePanelBorderColor: "#1f2937",
      asidePanelColor: "#f9fafb",
      badgeBackgroundColor: "#374151",
      badgeTextColor: "#f9fafb",
      contentBackgroundColor: "#ffffff",
      contentBorderColor: "transparent",
      contentTextColor: "#17202a",
      descriptionColor: "#6b7280",
      dividerColor: "#e5e7eb",
      fieldBackgroundColor: "#f3f4f6",
      fieldBorderColor: "transparent",
      fieldPlaceholderColor: "#9ca3af",
      fieldTextColor: "#17202a",
      formMutedTextColor: "#6b7280",
      iconMutedColor: "#6b7280",
      labelColor: "#17202a",
      pageBackgroundColor: "#f4f6f8",
      qrFrameBackgroundColor: "#ffffff",
      qrFrameBorderColor: "transparent",
      shellBackgroundColor: "#ffffff",
      shellBorderColor: "transparent",
      tabActiveBackgroundColor: "transparent",
      tabActiveTextColor: "#17202a",
      tabBackgroundColor: "transparent",
      tabInactiveTextColor: "#6b7280",
      titleColor: "#17202a",
    },
  };
}

export function resolveImAuthLocale(): string | null {
  if (typeof navigator === "undefined") {
    return null;
  }
  const language = navigator.language.trim();
  return language || null;
}
