import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const chatLayoutSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
);
const agentViewSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/AgentView.tsx',
);

assert.match(
  chatLayoutSource,
  /<CreateAgentModal[\s\S]*isOpen=\{isCreateAgentModalOpen\}[\s\S]*onSuccess=\{\(\)\s*=>\s*\{[\s\S]*setActiveTab\(["']create-agent["']\)/u,
  'ChatLayout must keep the existing create-agent modal and navigate to the detail page only after modal success',
);

assert.match(
  chatLayoutSource,
  /<AgentView[\s\S]*onCreateAgent=\{\(\)\s*=>\s*\{[\s\S]*setEditAgentId\(undefined\);[\s\S]*setIsCreateAgentModalOpen\(true\);[\s\S]*\}\}/u,
  'AgentView create action must open the shared create-agent modal instead of jumping directly to CreateAgentView',
);

const agentViewCreateHandlerMatch = chatLayoutSource.match(
  /<AgentView[\s\S]*?onCreateAgent=\{\(\)\s*=>\s*\{(?<body>[\s\S]*?)\}\}[\s\S]*?onEditAgent=/u,
);
assert.ok(agentViewCreateHandlerMatch, 'ChatLayout must pass an auditable onCreateAgent handler to AgentView');
assert.doesNotMatch(
  agentViewCreateHandlerMatch.groups.body,
  /setActiveTab\(["']create-agent["']\)/u,
  'AgentView create action must not navigate to CreateAgentView before the modal succeeds',
);

assert.doesNotMatch(
  agentViewSource,
  /CreateAgentModal/u,
  'AgentView must stay dependency-light and must not import/render CreateAgentModal directly',
);

console.log('sdkwork-im-pc agent create flow contract passed');
