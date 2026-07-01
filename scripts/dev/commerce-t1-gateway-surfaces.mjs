/** Commerce T1 app-api dependency surfaces for sdkwork-api-cloud-gateway IM config bundles. */

export const COMMERCE_T1_GATEWAY_APP_API_SURFACES = Object.freeze([
  {
    serviceId: 'sdkwork-account-app-api',
    workspace: 'sdkwork-account',
    sdkFamily: 'sdkwork-account-app-sdk',
    apiAuthority: 'sdkwork-account-app-api',
    segments: ['accounts', 'addresses', 'billing', 'wallet'],
  },
  {
    serviceId: 'sdkwork-catalog-app-api',
    workspace: 'sdkwork-catalog',
    sdkFamily: 'sdkwork-catalog-app-sdk',
    apiAuthority: 'sdkwork-catalog-app-api',
    segments: ['catalog'],
  },
  {
    serviceId: 'sdkwork-inventory-app-api',
    workspace: 'sdkwork-inventory',
    sdkFamily: 'sdkwork-inventory-app-sdk',
    apiAuthority: 'sdkwork-inventory-app-api',
    segments: ['shops/current/inventory'],
  },
  {
    serviceId: 'sdkwork-invoice-app-api',
    workspace: 'sdkwork-invoice',
    sdkFamily: 'sdkwork-invoice-app-sdk',
    apiAuthority: 'sdkwork-invoice-app-api',
    segments: ['invoices'],
  },
  {
    serviceId: 'sdkwork-membership-app-api',
    workspace: 'sdkwork-membership',
    sdkFamily: 'sdkwork-membership-app-sdk',
    apiAuthority: 'sdkwork-membership-app-api',
    segments: ['memberships'],
  },
  {
    serviceId: 'sdkwork-merchandise-app-api',
    workspace: 'sdkwork-merchandise',
    sdkFamily: 'sdkwork-merchandise-app-sdk',
    apiAuthority: 'sdkwork-merchandise-app-api',
    segments: ['catalog/products'],
  },
  {
    serviceId: 'sdkwork-order-app-api',
    workspace: 'sdkwork-order',
    sdkFamily: 'sdkwork-order-app-sdk',
    apiAuthority: 'sdkwork-order-app-api',
    segments: ['cart', 'checkout', 'orders', 'after_sales', 'fulfillments', 'shipments', 'refunds'],
  },
  {
    serviceId: 'sdkwork-payment-app-api',
    workspace: 'sdkwork-payment',
    sdkFamily: 'sdkwork-payment-app-sdk',
    apiAuthority: 'sdkwork-payment-app-api',
    segments: ['payments', 'recharges'],
  },
  {
    serviceId: 'sdkwork-promotion-app-api',
    workspace: 'sdkwork-promotion',
    sdkFamily: 'sdkwork-promotion-app-sdk',
    apiAuthority: 'sdkwork-promotion-app-api',
    segments: ['promotions'],
  },
  {
    serviceId: 'sdkwork-shop-app-api',
    workspace: 'sdkwork-shop',
    sdkFamily: 'sdkwork-shop-app-sdk',
    apiAuthority: 'sdkwork-shop-app-api',
    segments: ['shops'],
  },
]);

export const STANDALONE_EMBEDDED_GATEWAY_APP_API_SURFACES = Object.freeze([
  {
    serviceId: 'sdkwork-drive-app-api',
    workspace: 'sdkwork-drive',
    sdkFamily: 'sdkwork-drive-app-sdk',
    apiAuthority: 'sdkwork-drive-app-api',
    segments: ['drive'],
    requiredBaseUrlKey: 'SDKWORK_DRIVE_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-knowledgebase-app-api',
    workspace: 'sdkwork-knowledgebase',
    sdkFamily: 'sdkwork-knowledgebase-app-sdk',
    apiAuthority: 'sdkwork-knowledgebase-app-api',
    segments: ['knowledge'],
    requiredBaseUrlKey: 'SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-mail-app-api',
    workspace: 'sdkwork-mail',
    sdkFamily: 'sdkwork-mail-app-sdk',
    apiAuthority: 'sdkwork-mail-app-api',
    segments: ['mail'],
    requiredBaseUrlKey: 'SDKWORK_MAIL_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-notary-app-api',
    workspace: 'sdkwork-notary',
    sdkFamily: 'sdkwork-notary-app-sdk',
    apiAuthority: 'sdkwork-notary-app-api',
    segments: ['notary'],
    requiredBaseUrlKey: 'SDKWORK_NOTARY_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-course-app-api',
    workspace: 'sdkwork-course',
    sdkFamily: 'sdkwork-course-app-sdk',
    apiAuthority: 'sdkwork-course-app-api',
    segments: [
      'course_categories',
      'courses',
      'course_offerings',
      'course_enrollments',
      'course_lessons',
      'course_live_sessions',
      'course_comments',
      'course_reactions',
      'course_applications',
    ],
    requiredBaseUrlKey: 'SDKWORK_COURSE_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-community-app-api',
    workspace: 'sdkwork-community',
    sdkFamily: 'sdkwork-community-app-sdk',
    apiAuthority: 'sdkwork-community-app-api',
    segments: ['community'],
    requiredBaseUrlKey: 'SDKWORK_COMMUNITY_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-voice-app-api',
    workspace: 'sdkwork-voice',
    sdkFamily: 'sdkwork-voice-app-sdk',
    apiAuthority: 'sdkwork-voice-app-api',
    segments: ['voice'],
    requiredBaseUrlKey: 'SDKWORK_VOICE_APP_API_BASE_URL',
  },
  {
    serviceId: 'sdkwork-agents-app-api',
    workspace: 'sdkwork-agents',
    sdkFamily: 'sdkwork-agents-app-sdk',
    apiAuthority: 'sdkwork-agents-app-api',
    segments: ['ai'],
    requiredBaseUrlKey: 'SDKWORK_AGENTS_APP_API_BASE_URL',
  },
]);

function renderDependencySurface(surface, segment) {
  const capability = surface.serviceId
    .replace(/^sdkwork-/, '')
    .replace(/-app-api$/, '')
    .replace(/-/g, '_')
    .toUpperCase();
  const requiredBaseUrlKey = surface.requiredBaseUrlKey
    ?? `SDKWORK_${capability}_APP_API_BASE_URL`;
  return [
    '[[dependencySurfaces]]',
    `serviceId = "${surface.serviceId}"`,
    `workspace = "${surface.workspace}"`,
    `sdkFamily = "${surface.sdkFamily}"`,
    `apiAuthority = "${surface.apiAuthority}"`,
    'surface = "app"',
    `apiPrefix = "/app/v3/api/${segment}"`,
    'runtimeMode = "split-or-embedded"',
    'sameOriginAllowed = true',
    `coverage = "${surface.apiAuthority}-${segment.replace(/\//g, '-')}-routes"`,
    `requiredBaseUrlKey = "${requiredBaseUrlKey}"`,
    '',
  ].join('\n');
}

function renderUpstream(serviceId, baseUrl) {
  return [
    '[[upstreams]]',
    `serviceId = "${serviceId}"`,
    `baseUrl = "${baseUrl}"`,
    '',
  ].join('\n');
}

export function renderImApiCloudGatewayCommerceSurfacesToml({
  applicationIngressBaseUrl = 'http://127.0.0.1:18079',
} = {}) {
  const blocks = [];
  const upstreamServiceIds = new Set();

  for (const surface of COMMERCE_T1_GATEWAY_APP_API_SURFACES) {
    for (const segment of surface.segments) {
      blocks.push(renderDependencySurface(surface, segment));
    }
    upstreamServiceIds.add(surface.serviceId);
  }

  for (const surface of STANDALONE_EMBEDDED_GATEWAY_APP_API_SURFACES) {
    for (const segment of surface.segments) {
      blocks.push(renderDependencySurface(surface, segment));
    }
    upstreamServiceIds.add(surface.serviceId);
  }

  for (const serviceId of upstreamServiceIds) {
    blocks.push(renderUpstream(serviceId, applicationIngressBaseUrl));
  }

  return blocks.join('\n');
}
