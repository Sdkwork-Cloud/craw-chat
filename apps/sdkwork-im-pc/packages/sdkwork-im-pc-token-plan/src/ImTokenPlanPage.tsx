import { SdkworkSubscriptionCatalogPage } from "@sdkwork/membership-pc-subscription/catalog";
import { getImHostedMembershipCheckoutService } from "@sdkwork/im-pc-core";
import { imTokenPlanCatalogHostComponents } from "./ImTokenPlanCheckoutModal";
import { useImTokenPlanMemberSummary } from "./tokenPlanMemberSummary";

type TokenPlanNoticeTone = "error" | "info" | "success";

export interface ImTokenPlanPageProps {
  onNotify?: (message: string, tone: TokenPlanNoticeTone) => void;
}

/** Full-screen IM adapter over the shared Membership catalog and default checkout. */
export function ImTokenPlanPage({ onNotify }: ImTokenPlanPageProps) {
  const { memberSummary, refreshMembership, setMembershipTierKey } = useImTokenPlanMemberSummary();

  return (
    <div className="flex h-full min-h-0 w-full overflow-y-auto bg-[#0e0e11]">
      <div className="mx-auto w-full max-w-7xl">
        <SdkworkSubscriptionCatalogPage
          checkoutPort={getImHostedMembershipCheckoutService()}
          components={imTokenPlanCatalogHostComponents}
          memberSummary={memberSummary}
          onMembershipTierUpdated={(membershipTierKey: string) => {
            setMembershipTierKey(membershipTierKey);
            void refreshMembership().catch(() => undefined);
          }}
          onNotify={onNotify}
        />
      </div>
    </div>
  );
}
