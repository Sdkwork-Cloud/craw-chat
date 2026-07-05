#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const pcRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');

function readPc(...segments) {
  return fs.readFileSync(path.join(pcRoot, ...segments), 'utf8');
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
assert.match(chatService, /MAX_INBOX_CONVERSATIONS = 500/u);
assert.doesNotMatch(chatService, /listAllInboxEntries/u);
assert.doesNotMatch(chatService, /collectCursorPages/u);

const chatLayout = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'pages', 'ChatLayout.tsx');
assert.match(chatLayout, /listChatsPage/u);
assert.match(chatLayout, /loadMoreInboxChats/u);
assert.match(chatLayout, /groupService\.getGroupById/u);

const groupsContainer = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'contacts', 'GroupsContainer.tsx');
assert.match(groupsContainer, /groupService\.listGroupsPage/u);

const shopService = readPc('packages', 'sdkwork-im-pc-shop', 'src', 'services', 'ShopService.ts');
assert.match(shopService, /collectCursorPages/u);
assert.match(shopService, /initiatePayment[\s\S]*orders\.pay/u);

const ordersService = readPc('packages', 'sdkwork-im-pc-orders', 'src', 'services', 'OrdersService.ts');
assert.match(ordersService, /listOrdersPage/u);
assert.doesNotMatch(ordersService, /collectCursorPages/u);

const contactService = readPc('packages', 'sdkwork-im-pc-chat', 'src', 'services', 'ContactService.ts');
assert.match(contactService, /forEachCursorPage/u);
assert.match(contactService, /listContactsPage/u);
assert.match(contactService, /syncFriendRequestsFromServer/u);
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
assert.match(organizationDirectoryService, /collectCursorPages/u);
assert.match(organizationDirectoryService, /SDKWORK_MAX_PAGE_SIZE/u);

const roleService = readPc('packages', 'sdkwork-im-console-roles', 'src', 'services', 'RoleService.ts');
assert.match(roleService, /collectCursorPages/u);

const deviceService = fs.readFileSync(
  path.join(pcRoot, 'packages', 'sdkwork-im-pc-devices', 'src', 'services', 'DeviceService.ts'),
  'utf8',
);
assert.match(deviceService, /collectCursorPages/u);
assert.match(deviceService, /MAX_DEVICES_SYNC = 500/u);

const bindAgentModal = fs.readFileSync(
  path.join(pcRoot, 'packages', 'sdkwork-im-pc-devices', 'src', 'components', 'BindAgentModal.tsx'),
  'utf8',
);
assert.match(bindAgentModal, /listAgentsPage/u);
assert.doesNotMatch(bindAgentModal, /getAgents|getMarketAgents/u);

const mailService = fs.readFileSync(
  path.join(pcRoot, 'packages', 'sdkwork-im-pc-mail', 'src', 'services', 'MailService.ts'),
  'utf8',
);
assert.match(mailService, /collectCursorPages/u);
assert.match(mailService, /MAX_MAIL_MESSAGES_SYNC = 500/u);

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

assert.match(chatService, /LOCAL_MESSAGES_PER_CONVERSATION_CAP = 500/u);

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
