import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function assertFile(relativePath, message) {
  assert.ok(fs.existsSync(path.join(repoRoot, relativePath)), message ?? `${relativePath} must exist`);
}

function assertNoSdkBypass(source, label) {
  assert.doesNotMatch(source, /\bfetch\s*\(/u, `${label} must not use raw fetch`);
  assert.doesNotMatch(source, /\baxios\b/u, `${label} must not use axios`);
  assert.doesNotMatch(source, /\/(?:im|app|backend)\/v3/u, `${label} must not hand-code SDKWork API paths`);
  assert.doesNotMatch(source, /\b(Authorization|Access-Token|X-API-Key)\b/u, `${label} must not assemble auth headers`);
}

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SystemAssistantService.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatEmptyHome.tsx');

const systemAssistantSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SystemAssistantService.ts');
const chatEmptyHomeSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatEmptyHome.tsx');
const chatRightPanelSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx');
const chatWindowSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatWindow.tsx');
const messageListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx');
const chatLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');
const chatListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx');
const packageIndexSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/index.ts');
const chatEnLocale = JSON.parse(read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/en-US.json'));
const chatZhLocale = JSON.parse(read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/zh-CN.json'));
const oldAssistantLabelPattern = new RegExp(['SDKWork', 'Assistant'].join(' '), 'u');

assert.match(
  systemAssistantSource,
  /SYSTEM_ASSISTANT_AGENT[\s\S]*id:\s*['"]agent\.sdkwork_assistant['"]/u,
  'system assistant must use a standard agent id accepted by IM agent dialogs',
);
assert.match(
  systemAssistantSource,
  /chatService\.startAgentChat|startAgentChat\s*\(/u,
  'system assistant service must create the assistant conversation through ChatService.startAgentChat',
);
assert.doesNotMatch(
  systemAssistantSource,
  /lastMessage\s*:/u,
  'system assistant service must not synthesize a fake local assistant message history',
);
assert.doesNotMatch(
  systemAssistantSource,
  oldAssistantLabelPattern,
  'system assistant service must not use the old branded assistant display name',
);
assertNoSdkBypass(systemAssistantSource, 'SystemAssistantService');
assertNoSdkBypass(chatEmptyHomeSource, 'ChatEmptyHome');

assert.match(
  packageIndexSource,
  /export\s+\{\s*systemAssistantService[\s\S]*\}\s+from\s+['"]\.\/services\/SystemAssistantService['"]/u,
  'chat package must export the system assistant service as a first-class service',
);
assert.match(
  packageIndexSource,
  /export\s+type\s+\{[^}]*SystemAssistantService[^}]*SystemAssistantStartupResult[^}]*\}\s+from\s+['"]\.\/services\/SystemAssistantService['"]/u,
  'chat package must export system assistant service contracts for package consumers',
);

assert.match(
  chatLayoutSource,
  /import\s+\{\s*ChatEmptyHome\s*\}\s+from\s+['"]\.\.\/components\/ChatEmptyHome['"]/u,
  'chat layout must render a professional first-run workspace instead of a blank center pane',
);
assert.match(
  chatLayoutSource,
  /import\s+\{\s*systemAssistantService\s*\}\s+from\s+['"]\.\.\/services\/SystemAssistantService['"]/u,
  'chat layout startup must use the SDK-backed system assistant service',
);
assert.match(chatLayoutSource, /isChatStartupLoading/u, 'chat layout must expose an explicit startup loading state');
assert.match(chatLayoutSource, /chatStartupError/u, 'chat layout must expose a retryable startup error state');
assert.match(
  chatLayoutSource,
  /systemAssistantService\.ensureSystemAssistantChat\s*\(/u,
  'chat layout must ensure the system assistant during startup',
);
assert.match(
  chatLayoutSource,
  /systemAssistantService\.selectInitialChat\s*\(/u,
  'chat layout must use a centralized initial chat selection policy',
);
assert.match(chatLayoutSource, /<ChatEmptyHome/u, 'chat layout must render ChatEmptyHome when no chat is active');
assert.match(chatLayoutSource, /onOpenAssistant=\{handleOpenAssistant\}/u, 'empty home must offer an assistant open action');

assert.match(chatEmptyHomeSource, /useTranslation\s*\(/u, 'empty home user-facing copy must use react-i18next');
assert.match(chatListSource, /useTranslation\s*\(/u, 'chat list empty copy must use react-i18next');
assert.match(chatRightPanelSource, /useTranslation\s*\(/u, 'chat right panel user-facing copy must use react-i18next');
assert.doesNotMatch(chatEmptyHomeSource, /<aside\b/u, 'empty home must use a flat single-column layout without a right assistant card');
assert.doesNotMatch(chatEmptyHomeSource, /\bborder(?:-| |")/u, 'empty home must use a flat borderless style');
assert.doesNotMatch(chatEmptyHomeSource, /chatId|chatIdLabel|copyChatId|currentUser|<Copy\b/u, 'empty home must not show or manage Chat ID');
assert.doesNotMatch(chatEmptyHomeSource, oldAssistantLabelPattern, 'empty home must not repeat the old branded assistant product name');
assert.doesNotMatch(chatEmptyHomeSource, /Welcome to Sdkwork IM/u, 'empty home title must come from i18n instead of hard-coded English');
assert.doesNotMatch(chatListSource, /No conversations yet/u, 'chat list empty copy must come from i18n instead of hard-coded English');
assert.match(chatEmptyHomeSource, /chat\.emptyHome\.actions\.openAssistant\.title/u, 'empty home must keep a single assistant entry through i18n');
assert.match(chatListSource, /chat\.list\.empty\.noConversations/u, 'chat list must render empty state through i18n keys');
assert.match(chatListSource, /chat\.list\.item\.openConversation/u, 'left chat session rows must expose localized conversation labels');
assert.match(chatListSource, /chat\.list\.item\.unreadCount/u, 'left chat session unread badges must expose localized unread labels');
assert.match(chatLayoutSource, /placeholder=\{t\(["']chat\.searchInput\.placeholder["']\)\}/u, 'top search input placeholder must use dedicated i18n key');
assert.match(chatLayoutSource, /aria-label=\{t\(["']chat\.searchInput\.ariaLabel["']\)\}/u, 'top search input aria label must use dedicated i18n key');
assert.match(chatLayoutSource, /title=\{t\(["']chat\.searchInput\.title["']\)\}/u, 'top search input title must use dedicated i18n key');
assert.match(
  chatLayoutSource,
  /<ChatHistoryModal[\s\S]*?isOpen=\{activeModal === "search"/u,
  'chat search must open the SDK-backed history modal instead of a local inline search form',
);
assert.match(
  read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatHistoryModal.tsx'),
  /chat\.historySearch\.title/u,
  'chat history modal must render its title through i18n keys',
);
assert.match(chatRightPanelSource, /chat\.rightPanel\.actions\.searchChat/u, 'chat right panel must render actions through i18n keys');
assert.match(chatWindowSource, /systemAssistantService\.isSystemAssistantChat/u, 'chat window must recognize the default assistant conversation');
assert.match(chatWindowSource, /chat\.systemAssistant\.welcomeMessage/u, 'chat window must render a localized assistant welcome message');
assert.match(chatWindowSource, /chat\.systemAssistant\.inputPlaceholder/u, 'assistant conversation input must use localized placeholder copy');
assert.match(chatLayoutSource, /chat\.systemAssistant\.displayName/u, 'chat layout must localize the assistant conversation name in list and header');
assert.match(messageListSource, /fallbackMessages/u, 'message list must support display-only fallback messages');
assert.match(messageListSource, /senderProfiles/u, 'message list must support assistant sender profile overrides');

for (const [source, label] of [
  [chatLayoutSource, 'ChatLayout'],
  [chatRightPanelSource, 'ChatRightPanel'],
]) {
  assert.doesNotMatch(
    source,
    /查找聊天|添加群成员|搜索消息|输入成员|群聊名称|备注名称|填写群公告|正在搜索|邀请成员|保存名称失败|群公告已更新|暂无公告|Signed out|Local session cleared|RTC call watch failed/u,
    `${label} must not keep hard-coded user-facing chat copy that belongs in i18n`,
  );
}

function getByPath(source, keyPath) {
  return keyPath.split('.').reduce((cursor, key) => cursor?.[key], source);
}

for (const requiredKey of [
  'chat.emptyHome.eyebrow',
  'chat.emptyHome.title',
  'chat.emptyHome.description',
  'chat.emptyHome.chatIdLabel',
  'chat.emptyHome.copyChatId',
  'chat.emptyHome.status.preparing',
  'chat.emptyHome.status.fallbackLoadError',
  'chat.emptyHome.status.retry',
  'chat.emptyHome.actions.addFriend.title',
  'chat.emptyHome.actions.addFriend.description',
  'chat.emptyHome.actions.openAssistant.title',
  'chat.emptyHome.actions.openAssistant.description',
  'chat.emptyHome.actions.openAssistant.unavailableDescription',
  'chat.emptyHome.actions.createGroup.title',
  'chat.emptyHome.actions.createGroup.description',
  'chat.emptyHome.actions.openContacts.title',
  'chat.emptyHome.actions.openContacts.description',
  'chat.emptyHome.actions.createAgent.title',
  'chat.emptyHome.actions.createAgent.description',
  'chat.list.empty.noMatches',
  'chat.list.empty.noConversations',
  'chat.list.empty.searchHint',
  'chat.list.empty.startHint',
  'chat.list.context.pin',
  'chat.list.context.unpin',
  'chat.list.context.markRead',
  'chat.list.context.markUnread',
  'chat.list.context.mute',
  'chat.list.context.unmute',
  'chat.list.context.delete',
  'chat.list.toast.pinned',
  'chat.list.toast.unpinned',
  'chat.list.toast.markedRead',
  'chat.list.toast.markedUnread',
  'chat.list.toast.muted',
  'chat.list.toast.unmuted',
  'chat.list.toast.deleted',
  'chat.list.toast.operationFailed',
  'chat.list.toast.markReadFailed',
  'chat.list.time.yesterday',
  'chat.list.item.openConversation',
  'chat.list.item.muted',
  'chat.list.item.unreadCount',
  'chat.searchInput.placeholder',
  'chat.searchInput.ariaLabel',
  'chat.searchInput.title',
  'chat.systemAssistant.welcomeMessage',
  'chat.systemAssistant.inputPlaceholder',
  'chat.systemAssistant.displayName',
  'chat.window.inputPlaceholder',
  'chat.window.typing',
  'chat.window.toast.sendFailed',
  'chat.messageList.toast.loadFailed',
  'chat.messageList.toast.deleteFailed',
  'chat.messageList.toast.favoriteFailed',
  'chat.messageList.toast.reactionFailed',
  'chat.startup.assistantUnavailable',
  'chat.startup.assistantToastUnavailable',
  'chat.startup.syncWarning',
  'chat.startup.conversationsUnavailable',
  'chat.toast.chatIdCopied',
  'chat.toast.copyChatIdFailed',
  'chat.toast.startAgentFailed',
  'chat.toast.rtcCallWatchFailed',
  'chat.toast.signedOut',
  'chat.toast.localSessionCleared',
  'chat.toast.voiceLoading',
  'chat.toast.voiceCloneSoon',
  'chat.toast.workspaceAppUnavailable',
  'chat.toast.enterpriseChatFailed',
  'chat.toast.enterpriseCalling',
  'chat.toast.directChatFailed',
  'chat.call.incoming',
  'chat.header.search',
  'chat.header.voiceCall',
  'chat.header.videoCall',
  'chat.header.more',
  'chat.menu.moreActions',
  'chat.menu.startGroup',
  'chat.menu.addFriend',
  'chat.menu.createAssistant',
  'chat.rightPanel.actions.searchChat',
  'chat.rightPanel.actions.addMember',
  'chat.rightPanel.fields.groupName',
  'chat.rightPanel.fields.remark',
  'chat.rightPanel.fields.groupNotice',
  'chat.rightPanel.fields.mute',
  'chat.rightPanel.fields.pin',
  'chat.rightPanel.emptyNotice',
  'chat.rightPanel.actions.leaveGroup',
  'chat.rightPanel.actions.deleteChat',
  'chat.rightPanel.toast.muted',
  'chat.rightPanel.toast.unmuted',
  'chat.rightPanel.toast.muteFailed',
  'chat.rightPanel.toast.pinned',
  'chat.rightPanel.toast.unpinned',
  'chat.rightPanel.toast.pinFailed',
  'chat.rightPanel.toast.groupLeft',
  'chat.rightPanel.toast.chatDeleted',
  'chat.rightPanel.toast.deleteFailed',
  'chat.modal.title.searchMessages',
  'chat.modal.title.addMember',
  'chat.modal.title.editGroupName',
  'chat.modal.title.editRemark',
  'chat.modal.title.editNotice',
  'chat.modal.placeholder.searchMessages',
  'chat.modal.placeholder.memberSearch',
  'chat.modal.placeholder.groupName',
  'chat.modal.placeholder.remarkName',
  'chat.modal.placeholder.groupNotice',
  'chat.modal.actions.cancel',
  'chat.modal.actions.search',
  'chat.modal.actions.invite',
  'chat.modal.actions.save',
  'chat.modal.actions.publish',
  'chat.modal.toast.searching',
  'chat.modal.toast.invitedMembers',
  'chat.modal.toast.inviteFailed',
  'chat.modal.toast.inviteMissing',
  'chat.modal.toast.groupNameUpdated',
  'chat.modal.toast.remarkUpdated',
  'chat.modal.toast.saveNameFailed',
  'chat.modal.toast.noticeUpdated',
  'chat.modal.toast.updateNoticeFailed',
]) {
  assert.equal(typeof getByPath(chatEnLocale, requiredKey), 'string', `en-US locale must define ${requiredKey}`);
  assert.equal(typeof getByPath(chatZhLocale, requiredKey), 'string', `zh-CN locale must define ${requiredKey}`);
}

assert.equal(
  getByPath(chatEnLocale, 'chat.systemAssistant.displayName'),
  'System Assistant',
  'en-US assistant display name must be System Assistant',
);
assert.equal(
  getByPath(chatZhLocale, 'chat.systemAssistant.displayName'),
  '系统助手',
  'zh-CN assistant display name must be 系统助手',
);
assert.doesNotMatch(
  JSON.stringify(chatEnLocale.chat.systemAssistant),
  oldAssistantLabelPattern,
  'en-US system assistant locale must not use the old branded assistant label',
);
assert.doesNotMatch(
  JSON.stringify(chatZhLocale.chat.systemAssistant),
  oldAssistantLabelPattern,
  'zh-CN system assistant locale must not use the old branded assistant label',
);

for (const source of [chatEmptyHomeSource, chatListSource]) {
  assert.doesNotMatch(
    source,
    /鍙|娣|鏆|鎼|閫|璇|鐨|涓|宸|鍔|浼|缃|娑|惂|�/u,
    'touched empty/chat-list UI files must not contain mojibake user-facing copy',
  );
}

console.log('sdkwork-im-pc empty inbox assistant UI contract passed');
