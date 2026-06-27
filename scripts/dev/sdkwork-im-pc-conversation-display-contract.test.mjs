import assert from 'node:assert/strict';
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';

const root = process.cwd();
const read = (path) => readFileSync(join(root, path), 'utf8');

const chatServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts');
const groupServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService.ts');
const chatLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');
const appShellFrameSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/AppShellFrame.tsx');
const chatRightPanelSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx');
const scanQrCodeModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ScanQrCodeModal.tsx');
const defaultAvatarServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/DefaultAvatarService.ts');
const enLocale = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/en-US.json');
const zhLocale = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/zh-CN.json');

function listAuthoredSourceFiles(directory) {
  return readdirSync(join(root, directory), { withFileTypes: true }).flatMap((entry) => {
    const entryPath = `${directory}/${entry.name}`;
    if (entry.isDirectory()) {
      return listAuthoredSourceFiles(entryPath);
    }
    if (!entry.isFile() || !/\.(?:ts|tsx|js|jsx|mjs)$/u.test(entry.name)) {
      return [];
    }
    if (statSync(join(root, entryPath)).size === 0) {
      return [];
    }
    return [entryPath];
  });
}

assert.doesNotMatch(
  chatServiceSource,
  /`(?:Group|Agent Handoff) \$\{entry\.conversationId\}`/u,
  'ChatService user-visible fallback titles must not concatenate internal conversation ids',
);

assert.doesNotMatch(
  groupServiceSource,
  /name:\s*`Group \$\{entry\.conversationId\}`/u,
  'GroupService conversation-list fallback must not expose raw group conversation ids',
);

assert.doesNotMatch(
  chatLayoutSource,
  /name:\s*`Group \$\{groupId\}`/u,
  'opening a group invite must not create a visible Group <internal-id> fallback',
);

assert.doesNotMatch(
  chatRightPanelSource,
  /ID:\s*\{activeChat\.id\}/u,
  'conversation right panel must not display raw conversation ids as ordinary profile text',
);

assert.doesNotMatch(
  chatRightPanelSource,
  /memberProfile\?\.name\s*\?\?\s*memberId/u,
  'group member list must not use raw member ids as the visible member name when profile projection is missing',
);

assert.doesNotMatch(
  chatRightPanelSource,
  /memberProfile\?\.email\s*\?\?\s*memberProfile\?\.phone\s*\?\?\s*memberId/u,
  'group member list must not use raw member ids as the visible member subtitle when profile projection is missing',
);

assert.doesNotMatch(
  scanQrCodeModalSource,
  /group\?\.name\s*\?\?\s*resolvedResult\.payload\.groupId/u,
  'QR group result must use a neutral fallback instead of displaying raw group ids',
);

assert.doesNotMatch(
  scanQrCodeModalSource,
  /community\?\.name\s*\?\?\s*resolvedResult\.payload\.communityId/u,
  'QR community result must use a neutral fallback instead of displaying raw community ids',
);

assert.match(
  chatLayoutSource,
  /header=\{renderHeaderContent\(\)\}/u,
  'chat layout must delegate the unified header into AppShellFrame',
);
assert.match(
  appShellFrameSource,
  /<div\s+className="h-\[52px\] w-full flex items-center shrink-0 border-b border-white\/5 bg-\[#1e1e1e\] relative print:hidden">[\s\S]*?\{header\}/u,
  'chat unified header must use a compact 52px height shared by the conversation list header and chat window header',
);

assert.doesNotMatch(
  chatLayoutSource,
  /h-\[64px\] w-full flex items-center shrink-0 border-b border-white\/5 bg-\[#1e1e1e\]/u,
  'chat unified header must not use the previous 64px height',
);

assert.doesNotMatch(
  chatLayoutSource,
  /className="w-\[36px\] h-\[36px\] hover:bg-white\/5"/u,
  'chat header action buttons must be compact enough for the 52px header',
);

assert.match(
  chatLayoutSource,
  /className="w-\[32px\] h-\[32px\] hover:bg-white\/5"/u,
  'chat header action buttons must use the compact 32px size',
);

assert.doesNotMatch(
  chatLayoutSource,
  /className="text-\[18px\] text-gray-200 font-medium tracking-wide"/u,
  'chat header title text must not use the previous oversized 18px title in the compact header',
);

assert.match(
  chatLayoutSource,
  /className="text-\[16px\] text-gray-200 font-medium tracking-wide"/u,
  'chat header title text must use the compact 16px title size',
);

for (const [localeName, localeSource] of [['en-US', enLocale], ['zh-CN', zhLocale]]) {
  const locale = JSON.parse(localeSource);
  assert.equal(
    typeof locale.chat?.fallback?.groupName,
    'string',
    `${localeName} must define chat.fallback.groupName for user-safe group fallback titles`,
  );
  assert.equal(
    typeof locale.chat?.fallback?.memberName,
    'string',
    `${localeName} must define chat.fallback.memberName for missing group member profiles`,
  );
  assert.equal(
    typeof locale.scanQr?.group?.unknownName,
    'string',
    `${localeName} must define scanQr.group.unknownName for unresolved QR group results`,
  );
  assert.equal(
    typeof locale.scanQr?.community?.unknownName,
    'string',
    `${localeName} must define scanQr.community.unknownName for unresolved QR community results`,
  );
}

assert.match(
  chatLayoutSource,
  /const\s+needsGroupProjectionMerge\s*=/u,
  'ChatLayout must keep group projection merge guarded so realtime chat-list updates avoid unnecessary group hydration when projection is complete',
);

assert.match(
  chatLayoutSource,
  /if\s*\(\s*!needsGroupProjectionMerge\s*\(\s*sourceChats\s*\)\s*\)\s*\{\s*return\s+sourceChats;\s*\}/u,
  'mergeGroupProjections must return without calling groupService.getGroups when current chat projections are display-complete',
);

assert.match(
  chatServiceSource,
  /const\s+CHAT_LIST_HYDRATION_CONCURRENCY\s*=\s*4/u,
  'ChatService must keep legacy chat-list hydration concurrency bounded for old or incomplete inbox projections',
);

assert.match(
  chatServiceSource,
  /mapWithConcurrencyLimit\(\s*inboxEntries,\s*CHAT_LIST_HYDRATION_CONCURRENCY,/u,
  'ChatService.getChats must not hydrate inbox entries with an unbounded Promise.all',
);

assert.match(
  chatServiceSource,
  /function\s+isAgentDialogConversationId/u,
  'ChatService must recognize SDKWork agent dialog conversation ids separately from ordinary direct chats',
);

assert.match(
  chatServiceSource,
  /isAgentDialogConversationId\(entry\.conversationId\)[\s\S]*?AI assistant chat/u,
  'agent dialog conversations with incomplete projections must keep an agent-specific fallback title',
);

assert.match(
  groupServiceSource,
  /const\s+GROUP_LIST_HYDRATION_CONCURRENCY\s*=\s*4/u,
  'GroupService must keep group profile and member-state hydration concurrency bounded',
);

assert.doesNotMatch(
  groupServiceSource,
  /Promise\.all\(\s*Array\.from\(groupsById\.values\(\)\)\.map/u,
  'GroupService.getGroups must not use unbounded Promise.all for per-group member-state hydration',
);

assert.match(
  defaultAvatarServiceSource,
  /data:image\/svg\+xml;utf8/u,
  'DefaultAvatarService must render local packaged SVG data URI avatars',
);
assert.doesNotMatch(
  defaultAvatarServiceSource,
  /api\.dice(?:bear)\.com|https:\/\//iu,
  'DefaultAvatarService must not depend on network avatar providers',
);

const pcPackageSourceFiles = listAuthoredSourceFiles('apps/sdkwork-im-pc/packages');
const forbiddenGeneratedAvatarProviderPattern = /api\.dice(?:bear)\.com|dice(?:bear)/iu;
for (const sourceFile of pcPackageSourceFiles) {
  assert.doesNotMatch(
    read(sourceFile),
    forbiddenGeneratedAvatarProviderPattern,
    `${sourceFile} must not depend on remote generated avatar services for default avatars; use local packaged default avatars instead`,
  );
}

console.log('sdkwork-im-pc conversation display contract passed');
