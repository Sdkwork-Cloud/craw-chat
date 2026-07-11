import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const roomService = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RoomService.ts',
);
const authorityOpenapi = read('apis/open-api/im/sdkwork-im-im.openapi.yaml');
const sdkOpenapi = read('sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml');
const roomsModule = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/rooms-module.ts');
const transportClientLike = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/transport-client-like.ts',
);
const httpRuntime = read('services/sdkwork-comms-conversation-service/src/runtime/http.rs');
const roomRuntime = read('services/sdkwork-comms-conversation-service/src/runtime/room.rs');
const runtimeSupport = read('services/sdkwork-comms-conversation-service/src/runtime/support.rs');

assert.doesNotMatch(
  roomService,
  /function createConversationId|pc-room-conv/u,
  'PC RoomService must not generate local conversation ids for room creation.',
);
assert.doesNotMatch(
  roomService,
  /\.conversations\.create\s*\(/u,
  'PC RoomService must let rooms.create create the canonical room conversation.',
);
assert.match(
  roomService,
  /\.rooms\.create\s*\(\s*\{[\s\S]*roomId[\s\S]*roomKind/u,
  'PC RoomService must create room conversations through rooms.create.',
);

for (const [label, source] of [
  ['authority openapi', authorityOpenapi],
  ['sdk openapi mirror', sdkOpenapi],
]) {
  const match = source.match(/CreateRoomRequest:\n([\s\S]*?)(?:\n    [A-Z][A-Za-z0-9]+:|\n  parameters:|\n  responses:)/u);
  assert.ok(match, `${label} must declare CreateRoomRequest`);
  const schema = match[1];
  assert.doesNotMatch(
    schema,
    /required:\s*\n(?:\s*-\s+\w+\s*\n)*\s*-\s+conversationId/u,
    `${label} CreateRoomRequest.conversationId must be optional so the server owns the canonical id.`,
  );
}

assert.match(
  roomsModule,
  /create\(body:\s*ImCreateRoomRequest\):\s*Promise<CreateConversationResult>/u,
  'TypeScript IM rooms composed facade must use the authored room-create request type.',
);
assert.match(
  transportClientLike,
  /conversationId\?:\s*string\s*\|\s*null/u,
  'TypeScript IM rooms composed facade must allow callers to omit conversationId.',
);
assert.match(
  httpRuntime,
  /conversation_id:\s*Option<String>/u,
  'HTTP CreateRoomRequest must accept omitted conversationId.',
);
assert.match(
  roomRuntime,
  /resolve_room_create_conversation_id/u,
  'Room runtime must resolve the canonical room conversation id on the server.',
);
assert.match(
  runtimeSupport,
  /deterministic_conversation_resource_id\("r_",/u,
  'Room conversation ids must use a server-derived canonical room prefix.',
);

console.log('sdkwork im PC room create boundary contract passed.');
