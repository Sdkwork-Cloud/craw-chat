import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { AccountPortfolioService } from "@sdkwork/im-h5-user";
import { getOrderAppSdkClient } from "@sdkwork/im-h5-core/sdk";
import {
  createSubscriptionPurchaseService,
  type SubscriptionPurchasePort,
} from "@sdkwork/order-h5-subscription";

type ComponentName = "VipSubscriptionPage" | "TokenBankPurchasePage" | "CouponRedemptionPage";

/**
 * Lazy-loads a subscription page while preserving its exact props type.
 * Named property access (not a union index) keeps the component signature
 * intact so hosts can inject service/cashier-path props.
 */
function lazyComponent<K extends ComponentName>(name: K) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/order-h5-subscription");
    return { default: mod[name] };
  });
}

const VipSubscriptionPage = lazyComponent("VipSubscriptionPage");
const TokenBankPurchasePage = lazyComponent("TokenBankPurchasePage");
const CouponRedemptionPage = lazyComponent("CouponRedemptionPage");

/** Token Bank 可用余额（组合钱包接口返回，无需单独请求）。 */
async function resolveTokenBankBalance(): Promise<string> {
  const portfolio = await AccountPortfolioService.getPortfolio();
  return portfolio.tokenBank.availableAmount;
}

/**
 * 共享订阅购买服务：使用 bootstrap 已初始化的 order SDK 客户端
 * （含 dual-token 认证），充值/订阅接口才能通过 401 校验。
 *
 * 惰性创建：bootstrap 的 initSdkClients() 在应用启动时执行并注入
 * tokenManager；模块在渲染时才解析客户端，确保拿到的是已初始化实例。
 */
function resolveSubscriptionService(): SubscriptionPurchasePort {
  return createSubscriptionPurchaseService({
    orderAppSdkClient: getOrderAppSdkClient(),
  });
}

export const membershipModule: ImH5CapabilityModule = {
  id: "membership",
  routes: [
    {
      ...IM_H5_ROUTE_DEFINITIONS.membershipVip,
      render: () => (
        <VipSubscriptionPage
          service={resolveSubscriptionService()}
          cashierPath={IM_H5_ROUTE_DEFINITIONS.ordersCashier.path}
        />
      ),
    },
    {
      ...IM_H5_ROUTE_DEFINITIONS.membershipRecharge,
      render: () => (
        <TokenBankPurchasePage
          service={resolveSubscriptionService()}
          getBalance={resolveTokenBankBalance}
          cashierPath={IM_H5_ROUTE_DEFINITIONS.ordersCashier.path}
        />
      ),
    },
    {
      ...IM_H5_ROUTE_DEFINITIONS.membershipCoupon,
      render: () => (
        <CouponRedemptionPage
          service={resolveSubscriptionService()}
          onBalanceChanged={() => {
            void resolveTokenBankBalance().catch(() => undefined);
          }}
        />
      ),
    },
  ],
};
