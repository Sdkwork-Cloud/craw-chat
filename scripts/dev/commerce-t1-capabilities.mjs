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
    '../sdkwork-catalog/sdks/sdkwork-catalog-app-sdk/sdkwork-catalog-app-sdk-typescript/generated/server-openapi',
  shop:
    '../sdkwork-shop/sdks/sdkwork-shop-app-sdk/sdkwork-shop-app-sdk-typescript/generated/server-openapi',
  order:
    '../sdkwork-order/sdks/sdkwork-order-app-sdk/sdkwork-order-app-sdk-typescript/generated/server-openapi',
  membership: 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-membership-transport',
});

export const COMMERCE_T1_DATABASE_URL_ENV_KEYS = Object.freeze(
  COMMERCE_T1_REPOSITORY_IDS.map((repoId) => {
    const capability = repoId.replace(/^sdkwork-/, '').replace(/-/g, '_').toUpperCase();
    return `SDKWORK_${capability}_DATABASE_URL`;
  }),
);

export const COMMERCE_T1_SPLIT_OVERRIDE_ENV_KEY_GROUPS = Object.freeze(
  COMMERCE_T1_APP_API_AUTHORITIES.map((authority) => {
    const capability = authority
      .replace(/^sdkwork-/, '')
      .replace(/-app-api$/, '')
      .replace(/-/g, '_')
      .toUpperCase();
    return Object.freeze([
      `SDKWORK_IM_${capability}_APP_API_UPSTREAM`,
      `SDKWORK_${capability}_APP_API_UPSTREAM`,
      `SDKWORK_${capability}_APP_API_BASE_URL`,
    ]);
  }),
);
