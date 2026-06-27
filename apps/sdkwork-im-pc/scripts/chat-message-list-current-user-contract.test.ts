import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const messageListSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/MessageList.tsx',
  'utf8',
);

assert.match(
  messageListSource,
  /function\s+isCurrentUserMessage\(\s*message:\s*Message,\s*currentUser:\s*User\s*\|\s*null\s*\):\s*boolean/u,
  'MessageList must centralize current-user message detection before rendering message rows.',
);

assert.match(
  messageListSource,
  /message\.senderId\s*===\s*currentUser\.id[\s\S]*message\.senderId\s*===\s*currentUser\.chatId/u,
  'MessageList must treat both the internal user id and public chat id as the current user sender identity.',
);

assert.doesNotMatch(
  messageListSource,
  /senderProfiles\[msg\.senderId\]\s*\|\|\s*usersMap\[msg\.senderId\]\s*\|\|\s*currentUser/u,
  'MessageList must not fall back unknown peer senders to the current user profile.',
);

assert.match(
  messageListSource,
  /const\s+sender\s*=\s*isMe\s*\?\s*currentUser\s*:\s*\(\s*senderProfiles\[msg\.senderId\]\s*\?\?\s*usersMap\[msg\.senderId\]\s*\)/u,
  'MessageList must resolve the current user profile only for messages actually sent by the current user.',
);

assert.match(
  messageListSource,
  /\{!isMe\s*&&\s*sender\?\.name\s*&&\s*\([\s\S]*?<span\s+className="text-\[12px\]\s+text-gray-400\s+font-medium">\{sender\.name\}<\/span>[\s\S]*?\)\}/u,
  'MessageList must render sender names only for non-current-user messages.',
);

console.log('sdkwork im pc message list current-user display contract passed.');
