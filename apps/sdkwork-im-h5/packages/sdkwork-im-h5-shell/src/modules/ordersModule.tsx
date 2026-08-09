import React from "react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type OrderComponentName = "OrderCenter" | "OrderDetail" | "CashierPage" | "VoucherCodePage";

function lazyOrderComponent(name: OrderComponentName) {
  return React.lazy(async () => {
    const ordersModule = await import("@sdkwork/im-h5-orders");
    return { default: ordersModule[name] };
  });
}

const OrderCenter = lazyOrderComponent("OrderCenter");
const OrderDetail = lazyOrderComponent("OrderDetail");
const CashierPage = lazyOrderComponent("CashierPage");
const VoucherCodePage = lazyOrderComponent("VoucherCodePage");

export const ordersModule: ImH5CapabilityModule = {
  id: "orders",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.ordersCenter, render: () => <OrderCenter /> },
    { ...IM_H5_ROUTE_DEFINITIONS.ordersDetail, render: () => <OrderDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.ordersCashier, render: () => <CashierPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.ordersVoucher, render: () => <VoucherCodePage /> },
  ],
};
