import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const addGroupMembersModalSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/AddGroupMembersModal.tsx',
);
const contactMemberPickerPanelSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ContactMemberPickerPanel.tsx',
);
const chatEnUsMessages = readJson(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/en-US.json',
);
const chatZhCnMessages = readJson(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/zh-CN.json',
);

assert.match(
  addGroupMembersModalSource,
  /function\s+isExistingGroupMember[\s\S]*existingMemberIds\.has\(contact\.id\)[\s\S]*existingMemberIds\.has\(contact\.chatId/u,
  'group add-member modal must compare both user id and chat id when identifying existing group members',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /contacts\.filter\(\(contact\)\s*=>\s*!isExistingGroupMember\(existingMemberIds,\s*contact\)\)/u,
  'group add-member contacts tab must keep existing members visible instead of filtering them out',
);
assert.match(
  addGroupMembersModalSource,
  /const\s+disabledContactIds\s*=\s*useMemo[\s\S]*isExistingGroupMember\(existingMemberIds,\s*contact\)[\s\S]*contact\.id/u,
  'group add-member contacts tab must build a disabled contact id set from existing group members',
);
assert.match(
  addGroupMembersModalSource,
  /<ContactMemberPickerPanel[\s\S]*contacts=\{contacts\}[\s\S]*disabledContactIds=\{disabledContactIds\}[\s\S]*disabledReason=\{t\(['"]chat\.modal\.selection\.alreadyInGroup['"]\)\}/u,
  'group add-member contacts tab must pass all contacts plus an existing-member disabled state to the shared picker',
);
assert.match(
  addGroupMembersModalSource,
  /if\s*\(\s*disabledContactIds\.has\(contactId\)\s*\)\s*\{[\s\S]*?return\s+previousSelected;[\s\S]*?\}/u,
  'group add-member contacts tab must prevent toggling contacts who are already in the group',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /setNonContactSearchResults\(results\.filter\(\(user\)\s*=>\s*!isExistingGroupMember\(existingMemberIds,\s*user\)\)\)/u,
  'group add-member stranger search must keep existing members visible instead of filtering them out',
);
assert.match(
  addGroupMembersModalSource,
  /const\s+isAlreadyInGroup\s*=\s*isExistingGroupMember\(existingMemberIds,\s*user\)/u,
  'group add-member stranger search results must derive an existing-member state for each user',
);
assert.match(
  addGroupMembersModalSource,
  /disabled=\{isAlreadyInGroup\}/u,
  'group add-member stranger search result rows must be disabled when the user is already in the group',
);
assert.match(
  addGroupMembersModalSource,
  /t\(['"]chat\.modal\.selection\.alreadyInGroup['"]\)/u,
  'group add-member modal must render localized existing-member copy for contacts and stranger search results',
);
assert.match(
  addGroupMembersModalSource,
  /selectedNonContactUser\s*&&\s*!isExistingGroupMember\(existingMemberIds,\s*selectedNonContactUser\)/u,
  'group add-member stranger invite action must remain disabled for selected users who are already in the group',
);
assert.match(
  addGroupMembersModalSource,
  /disabled=\{selectedInviteIds\.length\s*===\s*0\s*\|\|\s*isSubmitting\}/u,
  'group add-member contacts invite action must be disabled when no selected contact is still inviteable',
);

assert.match(
  contactMemberPickerPanelSource,
  /disabledContactIds\?:\s*Set<string>/u,
  'shared contact picker must accept a disabled contact id set',
);
assert.match(
  contactMemberPickerPanelSource,
  /disabledReason\?:\s*string/u,
  'shared contact picker must accept localized disabled reason copy',
);
assert.match(
  contactMemberPickerPanelSource,
  /const\s+disabled\s*=\s*disabledContactIds\.has\(contact\.id\)/u,
  'shared contact picker must derive disabled row state from the disabled contact id set',
);
assert.match(
  contactMemberPickerPanelSource,
  /disabled=\{disabled\}/u,
  'shared contact picker must disable existing-member row buttons',
);
assert.match(
  contactMemberPickerPanelSource,
  /disabledReason/u,
  'shared contact picker must render the disabled reason badge for existing members',
);

assert.equal(
  chatEnUsMessages.chat?.modal?.selection?.alreadyInGroup,
  'Already in group',
  'English chat modal messages must label existing group members',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.selection?.alreadyInGroup,
  '已在群中',
  'Chinese chat modal messages must label existing group members',
);

console.log('sdkwork-im-pc group add-member existing state contract passed');
