#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const pcRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');
const workspaceRoot = path.resolve(repoRoot, '..');

function readPc(...segments) {
  return fs.readFileSync(path.join(pcRoot, ...segments), 'utf8');
}

function readSibling(...segments) {
  return fs.readFileSync(path.join(workspaceRoot, ...segments), 'utf8');
}

function assertImAdapterOnly(relativePath, label) {
  assert.ok(
    !fs.existsSync(path.join(pcRoot, relativePath)),
    `${label} must not keep duplicate IM-local implementation at ${relativePath}`,
  );
}

const helpers = readPc('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'appSdkResponseHelpers.ts');
assert.match(helpers, /export async function forEachCursorPage/u);
assert.match(helpers, /export async function forEachOffsetPage/u);
assert.match(helpers, /export async function collectCursorPages/u);
assert.match(helpers, /export function mapAppSdkCursorPage/u);
assert.match(helpers, /SDKWORK_DEFAULT_PAGE_SIZE = 20/u);
assert.match(helpers, /SDKWORK_MAX_PAGE_SIZE = 200/u);

const chatService = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'services', 'ChatService.ts');
assert.match(chatService, /listChatsPage/u);
assert.match(chatService, /forEachCursorPage/u);
assert.match(chatService, /MAX_INBOX_CONVERSATIONS = SDKWORK_MAX_PAGE_SIZE/u);
assert.doesNotMatch(chatService, /listAllInboxEntries/u);
assert.doesNotMatch(chatService, /collectCursorPages/u);

const chatLayout = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'pages', 'ChatLayout.tsx');
assert.match(chatLayout, /listChatsPage/u);
assert.match(chatLayout, /loadMoreInboxChats/u);
assert.match(chatLayout, /groupService\.getGroupById/u);

const groupsContainer = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'contacts', 'GroupsContainer.tsx');
assert.match(groupsContainer, /groupService\.listGroupsPage/u);

const contactService = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'services', 'ContactService.ts');
assert.match(contactService, /forEachCursorPage/u);
assert.match(contactService, /listContactsPage/u);
assert.match(contactService, /listTagsPage/u);
assert.match(contactService, /syncFriendRequestsFromServer/u);
assert.match(contactService, /removeFromBlacklist/u);
assert.match(contactService, /MAX_CONTACT_TAGS_SYNC/u);
assert.doesNotMatch(contactService, /listAllContacts/u);

const groupService = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'services', 'GroupService.ts');
assert.match(groupService, /forEachCursorPage/u);
assert.match(groupService, /listGroupsPage/u);
assert.match(groupService, /getGroupById/u);
assert.doesNotMatch(groupService, /listAllConversationMembers/u);

const organizationDirectoryService = readPc(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'OrganizationDirectoryService.ts',
);
assert.match(organizationDirectoryService, /forEachCursorPage/u);
assert.match(organizationDirectoryService, /SDKWORK_MAX_PAGE_SIZE/u);
assert.doesNotMatch(organizationDirectoryService, /collectCursorPages/u);

const roleService = readPc('packages', 'sdkwork-im-console-roles', 'src', 'services', 'RoleService.ts');
assert.match(roleService, /forEachCursorPage/u);
assert.doesNotMatch(roleService, /collectCursorPages/u);

assertImAdapterOnly('packages/sdkwork-im-pc-shop/src/services/ShopService.ts', 'Shop');
assertImAdapterOnly('packages/sdkwork-im-pc-orders/src/services/OrdersService.ts', 'Orders');
assertImAdapterOnly('packages/sdkwork-im-pc-devices/src/services/DeviceService.ts', 'Devices');
assertImAdapterOnly('packages/sdkwork-im-pc-mail/src/services/MailService.ts', 'Mail');

const imShopAdapter = readPc('packages', 'sdkwork-im-pc-shop', 'src', 'index.tsx');
assert.match(imShopAdapter, /@sdkwork\/shop-pc-consumer/u, 'IM shop adapter must re-export canonical shop PC consumer surfaces');

const shopService = readSibling(
  'sdkwork-shop',
  'apps',
  'sdkwork-shop-pc',
  'packages',
  'sdkwork-shop-pc-consumer',
  'src',
  'services',
  'ShopService.ts',
);
assert.match(shopService, /listProductsPage/u);
assert.match(shopService, /mapAppSdkCursorPage/u);
assert.match(shopService, /initiatePayment[\s\S]*orders\.pay/u);

const ordersService = readSibling(
  'sdkwork-shop',
  'apps',
  'sdkwork-shop-pc',
  'packages',
  'sdkwork-shop-pc-orders',
  'src',
  'services',
  'OrdersService.ts',
);
assert.match(ordersService, /listOrdersPage/u);
assert.doesNotMatch(ordersService, /collectCursorPages/u);

const deviceService = readSibling(
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-device',
  'src',
  'device-service.ts',
);
assert.match(deviceService, /listDevicePage/u);

const mailAppServices = readSibling(
  'sdkwork-mail',
  'apps',
  'sdkwork-mail-pc',
  'packages',
  'sdkwork-mail-pc-mail',
  'src',
  'services',
  'mailAppServices.ts',
);
assert.match(mailAppServices, /mail\.messages\.list/u);
assert.doesNotMatch(mailAppServices, /collectCursorPages/u);

const moduleRegistry = readPc('packages', 'sdkwork-im-pc-shell', 'src', 'moduleRegistry.ts');
const commercialBlock = moduleRegistry.match(
  /COMMERCIAL_RUNTIME_MODULES = new Set<AppModuleId>\(\[([\s\S]*?)\]\)/u,
)?.[1] ?? '';
assert.match(commercialBlock, /"shop"/u);
assert.match(commercialBlock, /"orders"/u);
assert.doesNotMatch(commercialBlock, /"mail"/u, 'mail must stay contract-pending');
assert.doesNotMatch(commercialBlock, /"devices"/u, 'devices must stay contract-pending');
assert.doesNotMatch(commercialBlock, /"course"/u, 'course must stay out of commercial runtime until verified');
assert.doesNotMatch(commercialBlock, /"enterprise"/u, 'enterprise must stay out of commercial runtime until verified');

assert.match(chatService, /LOCAL_MESSAGES_PER_CONVERSATION_CAP = SDKWORK_MAX_PAGE_SIZE/u);

const projectionBootstrap = fs.readFileSync(
  path.join(repoRoot, 'services', 'projection-service', 'src', 'bootstrap.rs'),
  'utf8',
);
assert.match(projectionBootstrap, /pub use im_app_context::is_production_like_im_environment/u);

const spaceRuntimeEnv = fs.readFileSync(
  path.join(repoRoot, 'services', 'space-service', 'src', 'runtime_env.rs'),
  'utf8',
);
assert.match(spaceRuntimeEnv, /pub\(crate\) use im_app_context::is_production_like_im_environment/u);

console.log('pc client pagination standard check passed');
