import { useEffect, useMemo, useState } from "react";
import {
  useSdkworkMembershipController,
  useSdkworkMembershipControllerState,
  type SdkworkMembershipSummary,
} from "@sdkwork/membership-pc-membership";
import {
  hasSdkworkMembershipSession,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
} from "@sdkwork/im-pc-core";

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
    const refreshMembership = () => {
      if (hasSdkworkMembershipSession()) {
        void controller.refresh().catch(() => undefined);
      }
    };

    window.addEventListener("focus", refreshMembership);
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, refreshMembership);
    return () => {
      window.removeEventListener("focus", refreshMembership);
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, refreshMembership);
    };
  }, [controller]);

  const memberSummary = useMemo(() => {
    if (!hasSdkworkMembershipSession()) {
      return null;
    }

    return {
      membershipTierKey: tierOverride ?? resolveImMembershipTierKey(state.dashboard.summary),
      pointBalance: state.dashboard.summary.pointBalance,
    };
  }, [state.dashboard.summary, tierOverride]);

  return {
    memberSummary,
    refreshMembership: () => controller.refresh(),
    setMembershipTierKey: setTierOverride,
  };
}
