import React from "react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { AccountPortfolioService } from "@sdkwork/im-h5-user";
import { getOrderAppSdkClient } from "@sdkwork/im-h5-core/sdk";
import {
  createWithdrawalRequestService,
  type WithdrawalRequestPort,
} from "@sdkwork/order-h5-withdraw";

type OrderComponentName = "OrderCenter" | "OrderDetail" | "CashierPage" | "VoucherCodePage";

/**
 * Lazy-loads an order page while preserving its exact props type. Named
 * property access (not a union index) keeps the component signature intact
 * so hosts can inject route-path props.
 */
function lazyOrderComponent<K extends OrderComponentName>(name: K) {
  return React.lazy(async () => {
    const ordersModule = await import("@sdkwork/im-h5-orders");
    return { default: ordersModule[name] };
  });
}

const OrderCenter = lazyOrderComponent("OrderCenter");
const OrderDetail = lazyOrderComponent("OrderDetail");
const CashierPage = lazyOrderComponent("CashierPage");
const VoucherCodePage = lazyOrderComponent("VoucherCodePage");

const WithdrawPage = React.lazy(async () => {
  const withdrawModule = await import("@sdkwork/order-h5-withdraw");
  return { default: withdrawModule.WithdrawPage };
});

/** 现金账户可用余额（组合钱包接口返回，无需单独请求）。 */
async function resolveCashBalance(): Promise<string> {
  const portfolio = await AccountPortfolioService.getPortfolio();
  return portfolio.cash.availableAmount;
}

/**
 * 提现申请服务：使用 bootstrap 已初始化的 order SDK 客户端
 * （含 dual-token 认证），withdrawals.requests 才能通过 401 校验。
 */
function resolveWithdrawalService(): WithdrawalRequestPort {
  return createWithdrawalRequestService({
    orderAppSdkClient: getOrderAppSdkClient(),
  });
}

export const ordersModule: ImH5CapabilityModule = {
  id: "orders",
  routes: [
    {
      ...IM_H5_ROUTE_DEFINITIONS.ordersCenter,
      render: () => (
        <OrderCenter
          orderDetailPath={IM_H5_ROUTE_DEFINITIONS.ordersDetail.path}
          orderCashierPath={IM_H5_ROUTE_DEFINITIONS.ordersCashier.path}
        />
      ),
    },
    {
      ...IM_H5_ROUTE_DEFINITIONS.ordersDetail,
      render: () => (
        <OrderDetail orderCashierPath={IM_H5_ROUTE_DEFINITIONS.ordersCashier.path} />
      ),
    },
    {
      ...IM_H5_ROUTE_DEFINITIONS.ordersCashier,
      render: () => (
        <CashierPage
          orderDetailPath={IM_H5_ROUTE_DEFINITIONS.ordersDetail.path}
          orderCenterPath={IM_H5_ROUTE_DEFINITIONS.ordersCenter.path}
        />
      ),
    },
    { ...IM_H5_ROUTE_DEFINITIONS.ordersVoucher, render: () => <VoucherCodePage /> },
    {
      ...IM_H5_ROUTE_DEFINITIONS.ordersWithdraw,
      render: () => (
        <WithdrawPage
          service={resolveWithdrawalService()}
          getCashBalance={resolveCashBalance}
        />
      ),
    },
  ],
};
