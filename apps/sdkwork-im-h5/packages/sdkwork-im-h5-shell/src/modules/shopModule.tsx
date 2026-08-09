import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "ShoppingPage" | "ShoppingCartPage" | "CheckoutPage" | "CashierPage" | "ProductDetails" | "ShopDetails" | "CustomerServiceChat" | "CategoryPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/shop-mobile-react-shopping");
    return { default: mod[name] };
  });
}

const ShoppingPage = lazyComponent("ShoppingPage");
const ShoppingCartPage = lazyComponent("ShoppingCartPage");
const CheckoutPage = lazyComponent("CheckoutPage");
const CashierPage = lazyComponent("CashierPage");
const ProductDetails = lazyComponent("ProductDetails");
const ShopDetails = lazyComponent("ShopDetails");
const CustomerServiceChat = lazyComponent("CustomerServiceChat");
const CategoryPage = lazyComponent("CategoryPage");

export const shopModule: ImH5CapabilityModule = {
  id: "shop",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.shopShopping, render: () => <ShoppingPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopCart, render: () => <ShoppingCartPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopCheckout, render: () => <CheckoutPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopCashier, render: () => <CashierPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopProductDetail, render: () => <ProductDetails /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopDetail, render: () => <ShopDetails /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopChat, render: () => <CustomerServiceChat /> },
    { ...IM_H5_ROUTE_DEFINITIONS.shopCategory, render: () => <CategoryPage /> },
  ],
};
