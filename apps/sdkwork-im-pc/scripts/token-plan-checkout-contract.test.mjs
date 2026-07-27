#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

const sidebarSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'Sidebar.tsx');
const capabilitySurfaceSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'surfaces', 'CapabilityModuleSurface.tsx');
const moduleLayoutSource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'moduleLayout.ts');
const authGateSource = readText('src', 'AuthGate.tsx');
const membershipIntegrationSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'membershipPcIntegration.ts');
const tokenPlanPageSource = readText('packages', 'sdkwork-im-pc-token-plan', 'src', 'ImTokenPlanPage.tsx');
const tokenPlanMemberSummarySource = readText('packages', 'sdkwork-im-pc-token-plan', 'src', 'tokenPlanMemberSummary.ts');
const tokenPlanPackage = readJson('packages', 'sdkwork-im-pc-token-plan', 'package.json');
const tokenPlanComponentSpec = readJson('packages', 'sdkwork-im-pc-token-plan', 'specs', 'component.spec.json');
const imPcCoreComponentSpec = readJson('packages', 'sdkwork-im-pc-core', 'specs', 'component.spec.json');
const indexCssSource = readText('src', 'index.css');
const imCheckoutAdapterPath = path.join(
  appRoot,
  'packages',
  'sdkwork-im-pc-token-plan',
  'src',
  'ImTokenPlanCheckoutModal.tsx',
);
const imCheckoutAdapterSource = fs.readFileSync(imCheckoutAdapterPath, 'utf8');
const imCommerceModalsSource = readText(
  'packages',
  'sdkwork-im-pc-token-plan',
  'src',
  'ImTokenPlanCommerceModals.tsx',
);
const tokenPlanI18nSource = readText(
  'packages',
  'sdkwork-im-pc-token-plan',
  'src',
  'i18n',
  'index.ts',
);
const workspaceSource = fs.readFileSync(path.join(repoRoot, 'pnpm-workspace.yaml'), 'utf8');
const rootComponentSpec = JSON.parse(fs.readFileSync(path.join(repoRoot, 'specs', 'component.spec.json'), 'utf8'));
const topologySpec = JSON.parse(fs.readFileSync(path.join(repoRoot, 'specs', 'topology.spec.json'), 'utf8'));
const standaloneGatewayCargoSource = fs.readFileSync(
  path.join(repoRoot, 'crates', 'sdkwork-api-im-standalone-gateway', 'Cargo.toml'),
  'utf8',
);
const standaloneDependencyRoutesSource = fs.readFileSync(
  path.join(repoRoot, 'crates', 'sdkwork-api-im-standalone-gateway', 'src', 'embedded_dependency_routes.rs'),
  'utf8',
);
const subscriptionCatalogPageSource = fs.readFileSync(
  path.join(
    repoRoot,
    '..',
    'sdkwork-membership',
    'apps',
    'sdkwork-membership-pc',
    'packages',
    'sdkwork-membership-pc-subscription',
    'src',
    'pages',
    'SubscriptionCatalogPage.tsx',
  ),
  'utf8',
);
const subscriptionCatalogHostComponentsSource = fs.readFileSync(
  path.join(
    repoRoot,
    '..',
    'sdkwork-membership',
    'apps',
    'sdkwork-membership-pc',
    'packages',
    'sdkwork-membership-pc-subscription',
    'src',
    'components',
    'subscription-catalog-host-components.tsx',
  ),
  'utf8',
);
const orderCheckoutDialogSource = fs.readFileSync(
  path.join(
    repoRoot,
    '..',
    'sdkwork-order',
    'apps',
    'sdkwork-order-pc',
    'packages',
    'sdkwork-order-pc-checkout',
    'src',
    'components',
    'order-checkout-dialog.tsx',
  ),
  'utf8',
);
const orderCheckoutStyleSource = fs.readFileSync(
  path.join(
    repoRoot,
    '..',
    'sdkwork-order',
    'apps',
    'sdkwork-order-pc',
    'packages',
    'sdkwork-order-pc-checkout',
    'src',
    'components',
    'order-checkout-dialog.css',
  ),
  'utf8',
);

function assertSourceContainsInOrder(source, fragments, message) {
  let cursor = 0;
  for (const fragment of fragments) {
    const index = source.indexOf(fragment, cursor);
    assert.notEqual(index, -1, `${message} Missing fragment: ${fragment}`);
    cursor = index + fragment.length;
  }
}

assert.match(
  sidebarSource,
  /active=\{activeTab === "token-plan"\}[\s\S]*onTabChange\("token-plan"\)[\s\S]*<Crown/u,
  'The IM sidebar must expose a persistent Token Plan action using the Crown icon.',
);
assert.match(
  capabilitySurfaceSource,
  /import\("@sdkwork\/im-pc-token-plan"\)[\s\S]*case "token-plan"[\s\S]*<ImTokenPlanPage onNotify=\{showToast\}/u,
  'The Token Plan surface must be lazy-loaded and connected to the IM notification host.',
);
assert.match(
  moduleLayoutSource,
  /FULLSCREEN_MODULE_TABS[\s\S]*'token-plan'/u,
  'Token Plan must use the full-screen capability layout.',
);
assert.match(
  tokenPlanPageSource,
  /@sdkwork\/membership-pc-subscription\/catalog/u,
  'The IM adapter must render the canonical Membership catalog.',
);
assert.match(
  tokenPlanPageSource,
  /checkoutPort=\{getImHostedMembershipCheckoutService\(\)\}[\s\S]*components=\{imTokenPlanCatalogHostComponents\}/u,
  'The IM composition adapter must inject the order-owned checkout port and UI into Membership.',
);
assertSourceContainsInOrder(
  authGateSource,
  ['navigate(buildAuthLoginPath(redirectTarget)', 'if (isAuthenticated) {', 'return children'],
  'The IM auth gate must preserve the requested route and mount Token Plan only for authenticated sessions.',
);
assert.match(
  tokenPlanMemberSummarySource,
  /useSdkworkWalletControllerState[\s\S]*pointBalance:\s*walletState\.overview\.account\.tokenBankAvailable/u,
  'The IM Token Plan member summary must expose Membership tier and the Account Token Bank balance.',
);
assert.equal(
  fs.existsSync(imCheckoutAdapterPath),
  true,
  'IM must own the cross-capability checkout UI adapter outside Membership.',
);
assert.match(
  subscriptionCatalogPageSource,
  /components \?\? sdkworkSubscriptionCatalogHostComponents/u,
  'Membership must provide the checkout host components by default.',
);
assert.match(
  subscriptionCatalogHostComponentsSource,
  /checkoutModal:\s*SubscriptionCatalogCheckoutModal/u,
  'The Membership default host must register its checkout component.',
);
assert.doesNotMatch(
  subscriptionCatalogHostComponentsSource,
  /<SdkworkOrderCheckoutDialog/u,
  'The Membership default checkout host must not import or render Order UI.',
);
assert.match(
  imCheckoutAdapterSource,
  /@sdkwork\/order-pc-checkout[\s\S]*<SdkworkOrderCheckoutDialog/u,
  'The IM composition adapter must own the Order checkout dialog integration.',
);
for (const integrationPattern of [
  /pointsDetailsModal:\s*ImTokenPlanTokenBankDetailsModal/u,
  /pointsPurchaseModal:\s*ImTokenPlanPointsPurchaseModal/u,
  /redeemModal:\s*ImTokenPlanRedeemModal/u,
]) {
  assert.match(
    imCheckoutAdapterSource,
    integrationPattern,
    'The IM Token Plan host must replace every Membership placeholder with a landed commerce capability.',
  );
}
assert.match(
  imCommerceModalsSource,
  /@sdkwork\/account-pc-wallet[\s\S]*@sdkwork\/order-pc-recharge/u,
  'The IM Token Plan host must compose Account Token Bank state with Order recharge UI.',
);
assert.match(
  imCommerceModalsSource,
  /SdkworkPointsRechargeDialog[\s\S]*service=\{getImHostedPointsRechargeService\(\)\}/u,
  'Compute Credits purchase must use the Order-owned recharge dialog and injected service.',
);
assert.match(
  imCommerceModalsSource,
  /SdkworkCouponRedemptionDialog[\s\S]*service=\{getImHostedCouponRechargeService\(\)\}/u,
  'Membership redemption must use the Order-owned coupon redemption dialog and injected service.',
);
assert.doesNotMatch(
  imCommerceModalsSource,
  /\bfetch\s*\(|axios|Authorization|Access-Token/u,
  'Token Plan commerce modals must not bypass generated SDK and composed service boundaries.',
);
assert.match(
  orderCheckoutDialogSource,
  /import "\.\/order-checkout-dialog\.css"[\s\S]*sdkwork-order-checkout-dialog__body[\s\S]*sdkwork-order-checkout-dialog__payment-panel/u,
  'The shared checkout dialog must own its layout stylesheet and semantic summary/payment regions.',
);
assert.match(
  orderCheckoutStyleSource,
  /\.sdkwork-order-checkout-dialog__body\s*\{[\s\S]*grid-template-columns:\s*minmax\(0,\s*1fr\)\s+20rem/u,
  'The shared checkout stylesheet must keep the plan summary and QR payment panel side by side by default.',
);
assert.match(
  orderCheckoutStyleSource,
  /\.sdkwork-order-checkout-dialog\s*\{[\s\S]*width:\s*min\(92vw,\s*52rem\)\s*!important[\s\S]*max-height:\s*min\(calc\(100dvh - 2rem\),\s*48rem\)/u,
  'The shared checkout dialog must keep a compact width and viewport-bounded height.',
);
assert.match(
  orderCheckoutStyleSource,
  /@media \(max-width: 39\.999rem\)[\s\S]*grid-template-columns:\s*minmax\(0,\s*1fr\)/u,
  'The shared checkout stylesheet must reserve a one-column fallback for phone-width viewports only.',
);
assert.doesNotMatch(
  subscriptionCatalogHostComponentsSource,
  /\bfetch\s*\(|Authorization|Access-Token|payment-app-sdk|order-backend-sdk/u,
  'The Membership default checkout host must not bypass the composed Membership and Order boundaries.',
);
assert.doesNotMatch(
  indexCssSource,
  /sdkwork-order\/apps\/sdkwork-order-pc\/packages\/sdkwork-order-pc-checkout\/src/u,
  'IM must not compile the Order checkout through a host Tailwind @source entry.',
);
assert.match(
  membershipIntegrationSource,
  /createAccountAppTransportClient\(\{[\s\S]*tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)[\s\S]*bootstrapSdkworkOrderAppService\(\{[\s\S]*tokenManager/u,
  'IM composition must initialize Account and Order with the global TokenManager.',
);
assert.match(
  membershipIntegrationSource,
  /configureSdkworkOrderSessionTokenProvider\(\(\) => readAppSdkSessionTokens\(\) \?\? \{\}\)[\s\S]*createSdkworkMembershipCheckoutService[\s\S]*createSdkworkPointsRechargeService[\s\S]*createSdkworkCouponRechargeService/u,
  'IM composition must register the current session before creating checkout, recharge, and redemption services.',
);
assert.match(
  membershipIntegrationSource,
  /createMembershipAppSdkClientConfig[\s\S]*tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'The Membership SDK client must share the same IM global TokenManager as Order checkout.',
);
assert.doesNotMatch(
  membershipIntegrationSource,
  /import\s*\{[^}]*bootstrapSdkworkOrderAppService[^}]*\}\s*from '@sdkwork\/membership-service'/u,
  'IM must never import Order bootstrap symbols from Membership.',
);
assert.match(
  membershipIntegrationSource,
  /resetMembershipPcIntegration[\s\S]*configureSdkworkOrderAppServiceProvider\(null\)[\s\S]*configureSdkworkOrderSessionTokenProvider\(null\)/u,
  'IM Membership reset must clear both Order app-service and session-token providers.',
);
for (const packageName of [
  '@sdkwork/account-pc-wallet',
  '@sdkwork/im-pc-core',
  '@sdkwork/membership-pc-membership',
  '@sdkwork/membership-pc-subscription',
  '@sdkwork/order-pc-recharge',
  '@sdkwork/ui-pc-react',
]) {
  assert.equal(
    tokenPlanPackage.dependencies?.[packageName],
    'workspace:*',
    `The IM Token Plan adapter must declare ${packageName} as a workspace dependency.`,
  );
}
assert.equal(
  tokenPlanPackage.dependencies?.['@sdkwork/order-pc-checkout'],
  'workspace:*',
  'The IM composition adapter must declare its direct Order checkout UI dependency.',
);
assert.equal(
  tokenPlanPackage.dependencies?.['react-i18next'],
  'catalog:',
  'The IM checkout adapter must declare the translation runtime it consumes.',
);
assert.equal(
  tokenPlanComponentSpec.contracts.requiredPorts.some(
    (port) => port.name === 'orderCheckoutDialog',
  ),
  true,
  'The IM composition adapter contract must declare the Order checkout dialog port.',
);
for (const portName of ['accountTokenBank', 'orderRechargeDialog']) {
  assert.equal(
    tokenPlanComponentSpec.contracts.requiredPorts.some((port) => port.name === portName),
    true,
    `The IM Token Plan component contract must declare the ${portName} port.`,
  );
}
assert.equal(
  tokenPlanComponentSpec.contracts.publicExports.includes('./i18n'),
  true,
  'The IM Token Plan package must export its package-owned i18n fragment.',
);
assert.match(
  tokenPlanI18nSource,
  /imTokenPlanDialogsEnUS[\s\S]*imTokenPlanDialogsZhCN/u,
  'The IM Token Plan package must publish both active locale fragments.',
);
assert.equal(
  tokenPlanComponentSpec.contracts.requiredPorts.some(
    (port) => port.name === 'membershipService',
  ),
  false,
  'The IM Token Plan component contract must not declare a Membership service port it does not consume.',
);
assert.deepEqual(
  tokenPlanComponentSpec.contracts.sdkDependencies,
  [],
  'The IM Token Plan component must not declare a direct SDK dependency.',
);
assert.deepEqual(
  tokenPlanComponentSpec.contracts.dependencyApiExports,
  [],
  'The IM Token Plan component must not re-export dependency APIs.',
);
assert.deepEqual(
  tokenPlanComponentSpec.contracts.dependencyApiSurfaces,
  [],
  'The IM Token Plan component must not mount dependency APIs.',
);
for (const workspace of ['sdkwork-account-app-sdk', 'sdkwork-membership-app-sdk', 'sdkwork-order-app-sdk']) {
  assert.equal(
    imPcCoreComponentSpec.contracts.sdkDependencies.some(
      (dependency) => dependency.workspace === workspace
        && dependency.surface === 'app-api'
        && dependency.credentialMode === 'authenticated-app-api',
    ),
    true,
    `IM PC core must declare ${workspace} in the authenticated SDK composition closure.`,
  );
  assert.equal(
    rootComponentSpec.contracts.dependencyApiSurfaces.some(
      (dependency) => dependency.workspace === workspace
        && dependency.apiAuthority === workspace.replace('-sdk', '-api')
        && dependency.targetRuntimeIntegration?.mode === 'profile-resolved',
    ),
    true,
    `The IM application contract must route ${workspace} through the shared gateway.`,
  );
}
for (const serviceId of [
  'sdkwork-account-app-api',
  'sdkwork-membership-app-api',
  'sdkwork-order-app-api',
  'sdkwork-payment-app-api',
]) {
  assert.equal(
    rootComponentSpec.integration.platformApiGateway.standaloneEmbeddedAuthorities.includes(serviceId),
    true,
    `Standalone Token Plan runtime must embed ${serviceId}.`,
  );
}
for (const dependency of [
  ['account', 'sdkwork-account'],
  ['membership', 'sdkwork-membership'],
  ['order', 'sdkwork-order'],
  ['payment', 'sdkwork-payment'],
]) {
  const [capability, crateStem] = dependency;
  assertSourceContainsInOrder(
    standaloneGatewayCargoSource,
    [`sdkwork-api-${capability}-assembly`, `${crateStem}-service-host`],
    `Standalone Token Plan runtime must link the ${capability} API assembly and service host.`,
  );
  assertSourceContainsInOrder(
    standaloneDependencyRoutesSource,
    [
      `merge_embedded_dependency(router, "${capability}"`,
      `sdkwork_api_${capability}_assembly::assemble_api_router`,
    ],
    `Standalone Token Plan runtime must mount the ${capability} application router in-process.`,
  );
}
assert.deepEqual(
  topologySpec.vocabulary.deploymentProfile.allowed,
  ['standalone', 'cloud'],
  'Token Plan must remain available under both canonical IM deployment profiles.',
);
assert.equal(
  topologySpec.envKeys.clientApiGatewayBaseUrl,
  'VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
  'Both deployment profiles must resolve Token Plan SDK traffic through the platform API gateway root.',
);
assert.doesNotMatch(
  [tokenPlanPageSource, imCommerceModalsSource, membershipIntegrationSource, standaloneDependencyRoutesSource].join('\n'),
  /clawrouter|claw-router|claw_router/iu,
  'IM Token Plan integration must not depend on ClawRouter business code or SDKs.',
);
for (const workspacePath of [
  'sdkwork-account-service',
  'sdkwork-account-pc-wallet',
  'sdkwork-membership-pc-membership',
  'sdkwork-membership-pc-subscription',
  'sdkwork-order-pc-checkout',
  'sdkwork-order-pc-recharge',
]) {
  assert.match(
    workspaceSource,
    new RegExp(workspacePath, 'u'),
    `The IM workspace must include ${workspacePath}.`,
  );
}

console.log('IM Token Plan checkout contract checks passed');
