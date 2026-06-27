import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const roomDomain = read('crates/im-domain-core/src/room.rs');
const roomRuntime = read('services/sdkwork-comms-conversation-service/src/runtime/room.rs');
const http = read('services/sdkwork-comms-conversation-service/src/runtime/http.rs');
const policy = read('services/sdkwork-comms-conversation-service/src/runtime/policy.rs');
const binding = read('services/sdkwork-comms-conversation-service/src/runtime/binding.rs');
const gateway = read('services/sdkwork-im-cloud-gateway/src/lib.rs');
const routerPaths = read('crates/sdkwork-routes-im-chat-open-api/src/paths.rs');
const openapi = read('apis/open-api/im/sdkwork-im-im.openapi.yaml');
const sdkOpenapi = read('sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml');
const architecture = read(
  'docs/架构/69-room-live-chat-game-capability-standard-2026-06-23.md',
);
const roomsModule = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/rooms-module.ts');
const imSdk = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/sdk.ts');
const sdkFamilyConfig = read('sdks/sdkwork-im-sdk/bin/sdk-family-config.mjs');
const generatedChatApi = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/src/api/chat.ts',
);
const flutterGeneratedChat = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi/lib/src/api/chat.dart',
);
const rustGeneratedChat = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/generated/server-openapi/src/api/chat.rs',
);
const assembly = read('sdks/sdkwork-im-sdk/.sdkwork-assembly.json');
const generateSdk = read('sdks/sdkwork-im-sdk/bin/generate-sdk.mjs');

assert.match(roomDomain, /ROOM_BUSINESS_TYPE_LIVE/);
assert.match(roomDomain, /ROOM_BUSINESS_TYPE_CHAT/);
assert.match(roomDomain, /ROOM_BUSINESS_TYPE_GAME/);
assert.match(roomDomain, /SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX/);

assert.match(roomRuntime, /pub fn create_room_from_auth_context/);
assert.match(roomRuntime, /pub fn enter_room_from_auth_context/);
assert.match(roomRuntime, /pub fn leave_room_from_auth_context/);
assert.match(roomRuntime, /pub fn room_view_from_auth_context/);

assert.match(http, /\/im\/v3\/api\/chat\/rooms/);
assert.match(http, /\/im\/v3\/api\/chat\/rooms\/\{room_id\}\/enter/);
assert.match(http, /\/im\/v3\/api\/chat\/rooms\/\{room_id\}\/leave/);

assert.match(policy, /ensure_room_enter_allowed/);
assert.match(policy, /ensure_room_message_post_allowed/);
assert.match(policy, /SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT/);

assert.match(binding, /organization_id_from_auth_context\(auth\)/);
assert.doesNotMatch(binding, /require_active_member_with_kind\([\s\S]*"default"/);

assert.match(gateway, /\/im\/v3\/api\/chat\/rooms/);
assert.match(routerPaths, /pub const ROOMS/);
assert.match(routerPaths, /pub const ROOM_ENTER/);

for (const [label, source] of [
  ['authority openapi', openapi],
  ['sdk openapi mirror', sdkOpenapi],
]) {
  assert.match(source, /\/im\/v3\/api\/chat\/rooms:/, `${label} must declare rooms.create`);
  assert.match(source, /operationId: rooms\.create/, `${label} must declare rooms.create`);
  assert.match(source, /operationId: rooms\.get/, `${label} must declare rooms.get`);
  assert.match(source, /operationId: rooms\.enter/, `${label} must declare rooms.enter`);
  assert.match(source, /operationId: rooms\.leave/, `${label} must declare rooms.leave`);
  assert.match(source, /CreateRoomRequest:/, `${label} must declare CreateRoomRequest schema`);
  assert.match(source, /RoomView:/, `${label} must declare RoomView schema`);
  assert.match(source, /EnterRoomResponse:/, `${label} must declare EnterRoomResponse schema`);
}

assert.match(architecture, /live_room/);
assert.match(architecture, /chat_room/);
assert.match(architecture, /game_room/);
assert.match(architecture, /rooms\.create/);

assert.match(roomsModule, /class ImRoomsModule/);
assert.match(roomsModule, /transportClient\.chat\.rooms\.create/);
assert.match(roomsModule, /transportClient\.chat\.rooms\.enter/);
assert.match(imSdk, /readonly rooms: ImRoomsModule/);
assert.match(sdkFamilyConfig, /\/im\/v3\/api\/chat\/rooms/);
assert.match(generatedChatApi, /public readonly rooms: ChatRoomsApi/);
assert.match(generatedChatApi, /imApiPath\(`\/chat\/rooms`\)/);
assert.match(generatedChatApi, /CreateRoomRequest/);
assert.match(generatedChatApi, /RoomView/);
assert.match(generatedChatApi, /EnterRoomResponse/);
assert.match(flutterGeneratedChat, /roomsCreate/);
assert.match(rustGeneratedChat, /rooms_create/);
assert.match(rustGeneratedChat, /\/chat\/rooms/);
assert.match(assembly, /"authoritySpec": "\.\.\/\.\.\/apis\/open-api\/im\/sdkwork-im-im\.openapi\.yaml"/);
assert.match(generateSdk, /sync-openapi-authority-mirror\.mjs/);
assert.match(read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RoomService.ts'), /roomService = createSdkworkRoomService/);

assert.match(read('docs/sites/api-reference/im/rooms.md'), /rooms\.create/);
assert.match(read('docs/sites/sdk/modules/rooms.md'), /sdk\.rooms\.create/);

console.log('sdkwork-im-room-capability-standard: ok');
