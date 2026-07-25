import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import {
  buildAgentMentionParts,
  filterMentionAgents,
  hasStructuredAgentMentionParts,
  mentionLabelForAgent,
  resolveActiveAgentMentionQuery,
} from '../packages/sdkwork-im-pc-chat/src/services/AgentMentionService.ts';

const agents = [
  { agentId: 'agent.writer', name: 'Writer', enabled: true },
  { agentId: 'agent.writer.pro', name: 'Writer', enabled: true },
  { agentId: 'agent.disabled', name: 'Disabled', enabled: false },
];
const sameTailAgents = [
  { agentId: 'agent.team_a.writer', name: 'Writer', enabled: true },
  { agentId: 'agent.team_b.writer', name: 'Writer', enabled: true },
];

assert.deepEqual(resolveActiveAgentMentionQuery('hello @wri'), {
  fromTextOffset: 6,
  query: 'wri',
});
assert.equal(resolveActiveAgentMentionQuery('email@example.com'), undefined);
assert.equal(filterMentionAgents(agents, '').length, 2);
assert.match(mentionLabelForAgent(agents[0], agents), /Writer \(writer\)/u);
assert.notEqual(
  mentionLabelForAgent(sameTailAgents[0], sameTailAgents),
  mentionLabelForAgent(sameTailAgents[1], sameTailAgents),
  'same display names and same terminal id segment must still be distinguishable',
);

const parts = buildAgentMentionParts(
  'Please ask @Writer (writer) and @Writer (pro) to review.',
  agents,
  7,
);
assert.ok(parts);
assert.equal(parts?.filter((part) => part.kind === 'mention').length, 2);
assert.deepEqual(
  parts?.filter((part) => part.kind === 'mention').map((part) => part.targetId),
  ['agent.writer', 'agent.writer.pro'],
);
assert.equal(buildAgentMentionParts('@Writer', agents, 0), undefined);
const sameTailParts = buildAgentMentionParts(
  `@${mentionLabelForAgent(sameTailAgents[0], sameTailAgents)} @${mentionLabelForAgent(sameTailAgents[1], sameTailAgents)}`,
  sameTailAgents,
  2,
);
assert.deepEqual(
  sameTailParts?.filter((part) => part.kind === 'mention').map((part) => part.targetId),
  ['agent.team_a.writer', 'agent.team_b.writer'],
);
assert.equal(buildAgentMentionParts('mail@Writer (writer)', agents, 7), undefined);
assert.equal(buildAgentMentionParts('mail.@Writer (writer)', agents, 7), undefined);
assert.equal(hasStructuredAgentMentionParts(parts), true);
assert.equal(hasStructuredAgentMentionParts(undefined), false);
assert.equal(hasStructuredAgentMentionParts([{
  kind: 'mention',
  targetKind: 'agent',
  targetId: 'agent.writer',
  displayText: '@Writer',
  assignmentGeneration: 0,
}]), false);
assert.equal(hasStructuredAgentMentionParts([{
  kind: 'mention',
  targetKind: 'user',
  targetId: 'user.writer',
  displayText: '@Writer',
  assignmentGeneration: 7,
}]), false);
assert.equal(hasStructuredAgentMentionParts([
  { kind: 'mention', targetKind: 'agent', targetId: 'user.writer', displayText: '@Writer', assignmentGeneration: 7 },
]), false);
assert.equal(hasStructuredAgentMentionParts([
  ...(parts ?? []),
  { kind: 'unknown' },
]), false);
assert.equal(hasStructuredAgentMentionParts([
  ...(parts ?? []),
  { kind: 'mention', targetKind: 'agent', targetId: '', displayText: '@', assignmentGeneration: 7 },
]), false);

const groupServiceSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/GroupService.ts',
  'utf8',
);
const chatServiceSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
const createGroupSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/CreateGroupModal.tsx',
  'utf8',
);
const groupsContainerSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/contacts/GroupsContainer.tsx',
  'utf8',
);
const profileSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx',
  'utf8',
);
const groupAgentsModalSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/GroupAgentsModal.tsx',
  'utf8',
);
const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);
const chatWindowSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/ChatWindow.tsx',
  'utf8',
);
const agentPickerSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/AgentPickerPanel.tsx',
  'utf8',
);
const modalWrapperSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/ModalWrapper.tsx',
  'utf8',
);
const messageInputSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/MessageInput.tsx',
  'utf8',
);
const messageItemsSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/MessageItems.tsx',
  'utf8',
);

assert.match(groupServiceSource, /replaceAgentAssignments/u);
assert.match(groupServiceSource, /getAgentAssignments/u);
assert.match(groupServiceSource, /getCurrentMember/u);
assert.match(groupServiceSource, /MAX_GROUP_MEMBER_LOOKUP_SCAN\s*=\s*10_000/u);
assert.match(groupServiceSource, /MAX_GROUP_INITIAL_MEMBERS\s*=\s*200/u);
assert.match(groupServiceSource, /listGroupMembersPage\(/u);
assert.match(groupServiceSource, /memberCountIsLowerBound/u);
assert.match(groupServiceSource, /getCurrentUserGroupRole\(/u);
assert.match(groupServiceSource, /between 1 and 10 agents/u);
assert.match(
  groupServiceSource,
  /existingGeneration\s*>\s*snapshot\.generation/u,
  'older assignment responses must not roll back the cached generation',
);
assert.match(groupServiceSource, /agentAssignmentSnapshots/u);
assert.match(groupServiceSource, /SDKWORK_IM_SESSION_CHANGED_EVENT/u);
assert.match(groupServiceSource, /this\.agentAssignmentSnapshots\.clear\(\)/u);
assert.match(groupServiceSource, /this\.assertSessionGeneration\(sessionGeneration\)/u);
const assignmentReadMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async getAgentAssignments('),
  groupServiceSource.indexOf('async replaceAgentAssignments('),
);
assert.match(assignmentReadMethod, /const sessionGeneration = this\.sessionGeneration/u);
assert.match(assignmentReadMethod, /this\.assertSessionGeneration\(sessionGeneration\)/u);
const assignmentReplaceMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async replaceAgentAssignments('),
  groupServiceSource.indexOf('private resolveCurrentUserId('),
);
assert.match(assignmentReplaceMethod, /const sessionGeneration = this\.sessionGeneration/u);
assert.match(assignmentReplaceMethod, /this\.assertSessionGeneration\(sessionGeneration\)/u);
const memberPageMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('private async listConversationMembersPage('),
  groupServiceSource.indexOf('async canManageAgents('),
);
assert.match(memberPageMethod, /expectedSessionGeneration/u);
assert.match(memberPageMethod, /this\.assertSessionGeneration\(expectedSessionGeneration\)/u);
const managePermissionMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async canManageAgents('),
  groupServiceSource.indexOf('private async syncActiveMemberIds('),
);
assert.match(managePermissionMethod, /const sessionGeneration = this\.sessionGeneration/u);
assert.match(managePermissionMethod, /this\.assertSessionGeneration\(sessionGeneration\)/u);
const inboxMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async listGroupsPage('),
  groupServiceSource.indexOf('async getGroupById('),
);
assert.match(inboxMethod, /const sessionGeneration = this\.sessionGeneration/u);
assert.match(inboxMethod, /this\.assertSessionGeneration\(sessionGeneration\)/u);
assert.match(inboxMethod, /hydrateConversationEntryGroup\(entry, sessionGeneration\)/u);
assert.match(inboxMethod, /const q = params\?\.q\?\.trim\(\)/u);
assert.match(inboxMethod, /\.\.\.\(q \? \{ q \} : \{\}\)/u);
assert.match(
  groupsContainerSource,
  /listGroupsPage\(\{ q: searchQuery\.trim\(\) \|\| undefined \}\)/u,
);
assert.match(groupsContainerSource, /window\.setTimeout\([\s\S]*?, 250\)/u);
assert.match(
  groupsContainerSource,
  /listGroupsPage\(\{[\s\S]*?cursor: nextCursor,[\s\S]*?q: searchQuery\.trim\(\) \|\| undefined/u,
);
assert.doesNotMatch(groupsContainerSource, /filteredGroups/u);
const getGroupMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async getGroupById('),
  groupServiceSource.indexOf('async getGroups('),
);
assert.match(getGroupMethod, /withMemberState\(group, sessionGeneration\)/u);
const addMembersMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async addMembers('),
  groupServiceSource.indexOf('async inviteUserToGroup('),
);
assert.match(addMembersMethod, /syncActiveMemberIds\(groupId, sessionGeneration\)/u);
assert.match(addMembersMethod, /syncMemberViewState\(groupId, false, sessionGeneration\)/u);
const deleteGroupMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async deleteGroup('),
);
assert.match(deleteGroupMethod, /await this\.chatClient\.deleteChat\(groupId\)/u);
assert.match(deleteGroupMethod, /this\.assertSessionGeneration\(sessionGeneration\)/u);
assert.match(groupServiceSource, /memberUserIds:\s*invitedMemberIds/u);
assert.match(groupServiceSource, /initializeKnowledgebase\?: boolean/u);
assert.match(
  groupServiceSource,
  /const initializeKnowledgebase = options\.initializeKnowledgebase === true/u,
  'group creation must keep Knowledgebase initialization opt-in',
);
assert.match(
  groupServiceSource,
  /initializeKnowledgebase \? \{ initializeKnowledgebase: true \} : \{\}/u,
  'the lazy default must omit initializeKnowledgebase from the SDK request',
);
assert.doesNotMatch(
  groupServiceSource.match(/async createGroup[\s\S]*?async listGroupsPage/u)?.[0] ?? '',
  /conversations\.addMember/u,
  'initial members must be submitted by the atomic create command',
);
assert.match(chatServiceSource, /postMessage\(chatId, \{/u);
assert.match(chatServiceSource, /explicitParts/u);
assert.match(
  chatServiceSource,
  /failedMessage\.parts as ChatContentPart\[\]/u,
  'manual retry must retain structured agent mention parts',
);
assert.match(
  chatServiceSource,
  /refreshAgentMentionParts[\s\S]*refreshAgentMentionGeneration/u,
  'manual and offline retries must rebase mentions to the current assignment generation',
);
const retryMethod = chatServiceSource.slice(
  chatServiceSource.indexOf('async retryFailedMessage('),
  chatServiceSource.indexOf('setReadFocusContext(', chatServiceSource.indexOf('async retryFailedMessage(')),
);
assert.match(
  retryMethod,
  /clientMsgId:\s*failedMessage\.id/u,
  'manual retry must reuse the failed queue client message id as the server idempotency key',
);
assert.match(
  retryMethod,
  /retryParts\s*\?\s*\{\s*parts:\s*retryParts\s*\}/u,
  'manual retry must pass refreshed structured parts into sendMessage',
);
assert.match(retryMethod, /const generation = this\.authSessionGeneration/u);
assert.match(
  retryMethod,
  /refreshAgentMentionParts\([\s\S]*failedMessage\.parts as ChatContentPart\[\],[\s\S]*generation/u,
  'manual retry mention refresh must remain bound to its originating auth session',
);
assert.ok(
  retryMethod.indexOf('await this.sendMessage(') < retryMethod.indexOf('filter((message) => message.id !== messageId)'),
  'the failed message must remain available until its replacement send succeeds',
);
const sendMessageMethod = chatServiceSource.slice(
  chatServiceSource.indexOf('async sendMessage('),
  chatServiceSource.indexOf('async forwardMessages('),
);
const successfulMessageState = sendMessageMethod.match(
  /const message:\s*Message\s*=\s*\{([\s\S]*?)\n\s*\};/u,
)?.[1] ?? '';
assert.match(
  successfulMessageState,
  /\.\.\.\(parts\s*\?\s*\{\s*parts\s*\}\s*:\s*\{\}\)/u,
  'an accepted send must retain structured parts in local message state',
);
assert.match(
  chatServiceSource,
  /item\.parts \? \{ parts: item\.parts/u,
  'offline queue hydration must retain structured agent mention parts',
);
assert.match(chatWindowSource, /groupService\.getAgentAssignments\(chat\.id\)/u);
assert.match(chatWindowSource, /buildAgentMentionParts/u);
assert.match(
  chatWindowSource,
  /if \(!hasStructuredAgentMentionParts\(extraInfo\?\.parts\)\) \{\s*throw error;/u,
  'assignment lookup failure may continue only with a complete structured agent mention payload',
);
assert.doesNotMatch(
  chatWindowSource,
  /&&\s*!extraInfo\?\.parts/u,
  'an existing mention part must still be rebased against the current server generation before send',
);
assert.match(createGroupSource, /selectedAgentIds/u);
assert.match(createGroupSource, /maxSelectable=\{MAX_GROUP_INITIAL_MEMBERS\}/u);
assert.match(createGroupSource, /t\('chat\.fallback\.groupName'\)/u);
assert.match(createGroupSource, /const \[initializeKnowledgebase, setInitializeKnowledgebase\] = useState\(false\)/u);
assert.match(createGroupSource, /setInitializeKnowledgebase\(false\)/u);
assert.doesNotMatch(
  createGroupSource,
  /isCanonicalGroupKnowledgebaseOrganizationId|resolveAppSdkOrganizationId|knowledgebaseOrganizationScopeRequired/u,
  'group creation must leave organization scope authorization to the IM service',
);
assert.match(createGroupSource, /const shouldInitializeKnowledgebase = initializeKnowledgebase/u);
assert.match(createGroupSource, /type="checkbox"[\s\S]{0,350}checked=\{initializeKnowledgebase\}/u);
assert.match(createGroupSource, /checked=\{initializeKnowledgebase\}[\s\S]{0,180}disabled=\{creating\}/u);
assert.match(createGroupSource, /initializeKnowledgebase: shouldInitializeKnowledgebase,\s*memberIds/u);
assert.match(createGroupSource, /group\.knowledgebaseInitialization\s*===\s*['"]active['"]/u);
assert.match(createGroupSource, /groupCreatedKnowledgebaseProvisioning/u);
assert.match(createGroupSource, /groupCreatedKnowledgebaseFailed/u);
assert.match(createGroupSource, /selectedAgentList/u);
assert.match(createGroupSource, /removeSelectedAgent/u);
assert.match(createGroupSource, /chat\.agentPicker\.selectedTitle/u);
assert.match(createGroupSource, /selectedAgentList\.length > 0/u);
assert.match(createGroupSource, /max-\[1100px\]:hidden/u);
assert.match(
  createGroupSource,
  /selectedAgentIds\.size\s*>\s*0[\s\S]{0,220}assignments\?\.map/u,
  'omitting agentAssignments when no agent is selected must preserve server default assignment',
);
assert.doesNotMatch(
  createGroupSource,
  /const assignments = \[\.\.\.selectedAgentIds\][\s\S]{0,240}\.sort\(/u,
  'initial agent assignment order must follow the user selection order',
);
assert.match(profileSource, /manageAgents/u);
assert.match(groupAgentsModalSource, /modalSessionRef/u);
assert.match(groupAgentsModalSource, /canManageAgents/u);
assert.match(groupAgentsModalSource, /assignmentRequestSequenceRef/u);
assert.match(groupAgentsModalSource, /await groupService\.canManageAgents\(chatId\)/u);
assert.match(groupAgentsModalSource, /permissionDenied/u);
assert.match(groupAgentsModalSource, /disabled=\{!canManageAgents \|\| !assignmentReady \|\| saving\}/u);
assert.match(
  groupAgentsModalSource,
  /requestSequenceRef\.current \+= 1;\s*setLoadingMore\(false\);/u,
  'changing an agent search must release stale load-more state immediately',
);
assert.match(
  groupAgentsModalSource,
  /catch \{\s*if \(modalSessionRef\.current !== sessionId \|\| openedChatIdRef\.current !== chatId\) \{\s*return;/u,
  'a stale conflict refresh must not update a closed or switched modal',
);
assert.match(chatLayoutSource, /preserveGroupAgentSnapshot/u);
assert.match(chatLayoutSource, /!hasAuthoritativeGroupAgentSnapshot\(chat\)/u);
assert.match(chatLayoutSource, /nextGeneration >= previousGeneration/u);
assert.match(
  chatLayoutSource,
  /preserveGroupAgentSnapshot\(previousActiveChat, refreshedActiveChat\)/u,
  'full inbox refresh must preserve a newer authoritative active-chat assignment snapshot',
);
assert.match(chatLayoutSource, /canManageAgents=\{canManageGroupAgents\}/u);
assert.match(chatLayoutSource, /canManageGroupMembers=\{/u);
assert.match(chatLayoutSource, /onLoadMoreGroupMembers=\{/u);
assert.match(profileSource, /canRemoveMember\s*=\s*canManageGroupMembers/u);
assert.match(profileSource, /memberCountAtLeast/u);

const createGroupMethod = groupServiceSource.slice(
  groupServiceSource.indexOf('async createGroup('),
  groupServiceSource.indexOf('async listGroupsPage('),
);
assert.match(createGroupMethod, /memberUserIds:\s*invitedMemberIds/u);
assert.doesNotMatch(createGroupMethod, /\.addMember\(/u);
assert.match(createGroupMethod, /Promise\.allSettled/u);
assert.match(groupAgentsModalSource, /groupService\.getAgentAssignments\(chatId\)/u);
assert.match(groupAgentsModalSource, /loadAssignmentSnapshot\(chat\.id\)/u);
assert.match(groupAgentsModalSource, /applyAssignmentSnapshot\(latest\.agents, latest\.generation\)/u);
assert.match(groupAgentsModalSource, /assignmentReady/u);
assert.match(agentPickerSource, /mentionLabelForAgent/u);
assert.match(agentPickerSource, /errorText/u);
assert.match(agentPickerSource, /onRetry/u);
assert.match(groupAgentsModalSource, /errorText=\{loadError/u);
assert.match(groupAgentsModalSource, /onRetry=\{\(\) => void load\(1, false, searchQuery\)\}/u);
assert.match(createGroupSource, /errorText=\{agentLoadError/u);
assert.match(createGroupSource, /retryAgentCatalog/u);
assert.match(modalWrapperSource, /role="dialog"/u);
assert.match(modalWrapperSource, /aria-modal="true"/u);
assert.match(modalWrapperSource, /event.key === 'Escape'/u);
assert.match(modalWrapperSource, /restoreFocusRef/u);
assert.match(messageInputSource, /role="listbox"/u);
assert.match(messageInputSource, /role="option"/u);
assert.match(messageInputSource, /aria-activedescendant/u);
assert.match(messageItemsSource, /readAgentMentions/u);
assert.match(messageItemsSource, /targetKind !== 'agent'/u);

console.log('sdkwork im pc group-agent chat contract passed.');
