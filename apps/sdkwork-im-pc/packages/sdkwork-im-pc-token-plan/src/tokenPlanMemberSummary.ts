import { useEffect, useMemo, useState } from "react";
import {
  useSdkworkMembershipController,
  useSdkworkMembershipControllerState,
  type SdkworkMembershipSummary,
} from "@sdkwork/membership-pc-membership";
import {
  createSdkworkWalletController,
  useSdkworkWalletControllerState,
} from "@sdkwork/account-pc-wallet";
import {
  hasSdkworkMembershipSession,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
} from "@sdkwork/im-pc-core";

const imTokenPlanWalletController = createSdkworkWalletController();

export function getImTokenPlanWalletController() {
  return imTokenPlanWalletController;
}

export function resolveImMembershipTierKey(summary: SdkworkMembershipSummary): string {
  if (!summary.isAuthenticated || summary.status === "guest" || !summary.isMember) {
    return "none";
  }

  if (summary.currentLevelValue !== null && summary.currentLevelValue >= 2) {
    return "peak";
  }

  return "pro";
}

/** Bridges the shared membership controller into the IM Token Plan host. */
export function useImTokenPlanMemberSummary() {
  const controller = useSdkworkMembershipController();
  const state = useSdkworkMembershipControllerState(controller);
  const walletController = getImTokenPlanWalletController();
  const walletState = useSdkworkWalletControllerState(walletController);
  const [tierOverride, setTierOverride] = useState<string | null>(null);

  useEffect(() => {
    if (!hasSdkworkMembershipSession()) {
      return;
    }

    if (!state.isBootstrapped && !state.isLoading && !state.lastError) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading, state.lastError]);

  useEffect(() => {
    if (!hasSdkworkMembershipSession()) {
      return;
    }

    if (!walletState.isBootstrapped && !walletState.isLoading && !walletState.lastError) {
      void walletController.bootstrap().catch(() => undefined);
    }
  }, [walletController, walletState.isBootstrapped, walletState.isLoading, walletState.lastError]);

  useEffect(() => {
    const refreshMembership = () => {
      if (hasSdkworkMembershipSession()) {
        void Promise.allSettled([
          controller.refresh(),
          walletController.refresh(),
        ]);
      }
    };

    window.addEventListener("focus", refreshMembership);
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, refreshMembership);
    return () => {
      window.removeEventListener("focus", refreshMembership);
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, refreshMembership);
    };
  }, [controller, walletController]);

  const memberSummary = useMemo(() => {
    if (!hasSdkworkMembershipSession()) {
      return null;
    }

    return {
      membershipTierKey: tierOverride ?? resolveImMembershipTierKey(state.dashboard.summary),
      pointBalance: walletState.overview.account.tokenBankAvailable,
    };
  }, [state.dashboard.summary, tierOverride, walletState.overview.account.tokenBankAvailable]);

  return {
    memberSummary,
    refreshMembership: async () => {
      await Promise.all([
        controller.refresh(),
        walletController.refresh(),
      ]);
    },
    setMembershipTierKey: setTierOverride,
  };
}
