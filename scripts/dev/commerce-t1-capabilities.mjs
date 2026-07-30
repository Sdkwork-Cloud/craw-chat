/** Canonical T1 commerce capability repos and app-api authorities (post sdkwork-commerce dissolution). */

export const COMMERCE_T1_REPOSITORY_IDS = Object.freeze([
  'sdkwork-account',
  'sdkwork-catalog',
  'sdkwork-inventory',
  'sdkwork-invoice',
  'sdkwork-membership',
  'sdkwork-merchandise',
  'sdkwork-order',
  'sdkwork-payment',
  'sdkwork-promotion',
  'sdkwork-shop',
]);

export const COMMERCE_T1_APP_API_AUTHORITIES = Object.freeze([
  'sdkwork-account-app-api',
  'sdkwork-catalog-app-api',
  'sdkwork-inventory-app-api',
  'sdkwork-invoice-app-api',
  'sdkwork-membership-app-api',
  'sdkwork-merchandise-app-api',
  'sdkwork-order-app-api',
  'sdkwork-payment-app-api',
  'sdkwork-promotion-app-api',
  'sdkwork-shop-app-api',
]);

export const COMMERCE_T1_APP_SDK_PACKAGES = Object.freeze({
  catalog: '@sdkwork/catalog-app-sdk',
  shop: '@sdkwork/shop-app-sdk',
  order: '@sdkwork/order-app-sdk',
  membership: '@sdkwork/membership-app-sdk',
});

export const COMMERCE_T1_APP_SDK_WORKSPACE_PATHS = Object.freeze({
  catalog:
    '../sdkwork-catalog/sdks/sdkwork-catalog-app-sdk/sdkwork-catalog-app-sdk-typescript/src/index.ts',
  shop:
    '../sdkwork-shop/sdks/sdkwork-shop-app-sdk/sdkwork-shop-app-sdk-typescript/src/index.ts',
  order:
    '../sdkwork-order/sdks/sdkwork-order-app-sdk/sdkwork-order-app-sdk-typescript/src/index.ts',
  membership:
    '../sdkwork-membership/sdks/sdkwork-membership-app-sdk/sdkwork-membership-app-sdk-typescript/src/index.ts',
});

