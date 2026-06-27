import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const messageListSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx',
);
const packageJson = JSON.parse(read('apps/sdkwork-im-pc/package.json'));

assert.ok(
  packageJson.dependencies?.['@tanstack/react-virtual'],
  '@sdkwork/im-pc must depend on @tanstack/react-virtual',
);

assert.match(
  messageListSource,
  /useVirtualizer/u,
  'MessageList must use tanstack virtualizer',
);
assert.match(
  messageListSource,
  /getVirtualItems/u,
  'MessageList must render only virtual rows',
);
assert.match(
  messageListSource,
  /measureElement/u,
  'MessageList must measure dynamic row heights',
);
assert.match(
  messageListSource,
  /scrollToIndex/u,
  'MessageList must support programmatic scroll via virtualizer',
);
assert.match(
  messageListSource,
  /scrollToMessage/u,
  'MessageList must scroll to reply targets through virtualizer',
);
assert.doesNotMatch(
  messageListSource,
  /AnimatePresence|enableMessageMotion|motion\.div/u,
  'MessageList must not keep non-virtual motion rendering branches',
);
assert.doesNotMatch(
  messageListSource,
  /messagesEndRef/u,
  'MessageList must not rely on sentinel scrollIntoView anchors',
);

console.log('sdkwork im pc message list virtualization contract passed.');
