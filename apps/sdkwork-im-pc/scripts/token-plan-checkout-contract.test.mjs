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
const membershipIntegrationSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'membershipPcIntegration.ts');
const tokenPlanPageSource = readText('packages', 'sdkwork-im-pc-token-plan', 'src', 'ImTokenPlanPage.tsx');
const tokenPlanPackage = readJson('packages', 'sdkwork-im-pc-token-plan', 'package.json');
const tokenPlanComponentSpec = readJson('packages', 'sdkwork-im-pc-token-plan', 'specs', 'component.spec.json');
const indexCssSource = readText('src', 'index.css');
const imCheckoutAdapterPath = path.join(
  appRoot,
  'packages',
  'sdkwork-im-pc-token-plan',
  'src',
  'ImTokenPlanCheckoutModal.tsx',
);
const workspaceSource = fs.readFileSync(path.join(repoRoot, 'pnpm-workspace.yaml'), 'utf8');
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

assert.match(
  sidebarSource,
  /active=\{activeTab === "token-plan"\}[\s\S]*onTabChange\("token-plan"\)[\s\S]*<Crown/u,
  'The IM sidebar must expose a persistent Token Plan action using the Crown icon.',
);
assert.match(
  capabilitySurfaceSource,
  /import\("@sdkwork\/im-pc-token-plan"\)[\s\S]*case "token-plan"[\s\S]*<ImTokenPlanPage/u,
  'The Token Plan surface must be lazy-loaded from its IM adapter package.',
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
assert.doesNotMatch(
  tokenPlanPageSource,
  /ImTokenPlanCheckoutModal|checkoutModal|\bcomponents=\{/u,
  'The IM page must not override the Membership default checkout host.',
);
assert.equal(
  fs.existsSync(imCheckoutAdapterPath),
  false,
  'IM must not retain a product-specific checkout adapter.',
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
assert.match(
  subscriptionCatalogHostComponentsSource,
  /<SdkworkOrderCheckoutDialog/u,
  'The Membership default checkout host must delegate QR payment UI to Order.',
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
  /\.sdkwork-order-checkout-dialog\s*\{[\s\S]*width:\s*min\(92vw,\s*52rem\)\s*!important[\s\S]*min-height:\s*min\(76vh,\s*43rem\)/u,
  'The shared checkout dialog must keep a compact width with room for the structured payment flow.',
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
  /bootstrapSdkworkOrderAppService\(\{[\s\S]*tokenManager/u,
  'IM Membership bootstrap must initialize the Order app service with the shared IAM TokenManager.',
);
assert.match(
  membershipIntegrationSource,
  /resetMembershipPcIntegration[\s\S]*configureSdkworkOrderAppServiceProvider\(null\)/u,
  'IM Membership reset must clear the Order app-service provider.',
);
for (const packageName of [
  '@sdkwork/im-pc-core',
  '@sdkwork/membership-pc-membership',
  '@sdkwork/membership-pc-subscription',
]) {
  assert.equal(
    tokenPlanPackage.dependencies?.[packageName],
    'workspace:*',
    `The IM Token Plan adapter must declare ${packageName} as a workspace dependency.`,
  );
}
assert.equal(
  tokenPlanPackage.dependencies?.['@sdkwork/order-pc-checkout'],
  undefined,
  'The IM Token Plan package must consume the Order checkout only through Membership.',
);
assert.equal(
  tokenPlanPackage.dependencies?.['react-i18next'],
  undefined,
  'The IM Token Plan package must not retain the removed checkout adapter dependency.',
);
assert.equal(
  tokenPlanComponentSpec.contracts.requiredPorts.some(
    (port) => port.name === 'orderCheckoutDialog',
  ),
  false,
  'The IM Token Plan component contract must not declare a direct Order checkout port.',
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
for (const workspacePath of [
  'sdkwork-membership-pc-membership',
  'sdkwork-membership-pc-subscription',
  'sdkwork-order-pc-checkout',
]) {
  assert.match(
    workspaceSource,
    new RegExp(workspacePath, 'u'),
    `The IM workspace must include ${workspacePath}.`,
  );
}

console.log('IM Token Plan checkout contract checks passed');
