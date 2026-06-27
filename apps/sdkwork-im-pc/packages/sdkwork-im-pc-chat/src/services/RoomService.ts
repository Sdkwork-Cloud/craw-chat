import type {
  ImSdkClient,
  PostedMessageResponse,
  RoomView,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';

export const SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX = 'urn:sdkwork:sdkwork-im:message:custom:game.';

export type SdkworkRoomKind = 'live' | 'chat' | 'game';

export interface SdkworkRoomBinding {
  conversationId: string;
  roomId: string;
  roomKind: SdkworkRoomKind;
}

export interface CreateSdkworkRoomOptions {
  roomKind: SdkworkRoomKind;
  conversationId?: string;
  roomId?: string;
  title?: string;
  memberIds?: string[];
}

export interface PostGameMoveOptions {
  conversationId: string;
  gameKey: string;
  payload: Record<string, unknown> | string;
  summary?: string;
  clientMsgId?: string;
}

export interface RoomService {
  createRoom(options: CreateSdkworkRoomOptions): Promise<SdkworkRoomBinding>;
  getRoom(roomId: string): Promise<RoomView>;
  enterRoom(roomId: string): Promise<void>;
  leaveRoom(roomId: string): Promise<void>;
  postGameMove(options: PostGameMoveOptions): Promise<PostedMessageResponse>;
  postRoomMessage(conversationId: string, text: string, clientMsgId?: string): Promise<PostedMessageResponse>;
}

export function buildGameMoveSchemaRef(gameKey: string): string {
  const normalizedKey = gameKey.trim();
  if (!normalizedKey) {
    throw new Error('game move schema key must not be empty');
  }
  return `${SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX}${normalizedKey}`;
}

function createRoomId(): string {
  const requestId =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-room-${requestId}`;
}

function createConversationId(): string {
  const requestId =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-room-conv-${requestId}`;
}

function uniqueMemberIds(memberIds: string[] | undefined): string[] {
  if (!memberIds?.length) {
    return [];
  }
  return [...new Set(memberIds.map((memberId) => memberId.trim()).filter(Boolean))];
}

function normalizeRoomTitle(title: string | undefined, roomKind: SdkworkRoomKind): string {
  const trimmedTitle = title?.trim();
  if (trimmedTitle) {
    return trimmedTitle;
  }
  switch (roomKind) {
    case 'live':
      return 'Live room';
    case 'game':
      return 'Game room';
    default:
      return 'Chat room';
  }
}

function serializeGameMovePayload(payload: Record<string, unknown> | string): string {
  return typeof payload === 'string' ? payload : JSON.stringify(payload);
}

class SdkworkRoomService implements RoomService {
  constructor(private readonly clientFactory: () => ImSdkClient = getImSdkClientWithSession) {}

  private client(): ImSdkClient {
    return this.clientFactory();
  }

  async createRoom(options: CreateSdkworkRoomOptions): Promise<SdkworkRoomBinding> {
    const roomKind = options.roomKind;
    const roomId = options.roomId?.trim() || createRoomId();
    const conversationId = options.conversationId?.trim() || createConversationId();

    await this.client().conversations.create({
      conversationId,
      conversationType: 'group',
    });
    await this.client().conversations.updateProfile(conversationId, {
      displayName: normalizeRoomTitle(options.title, roomKind),
    });
    await this.client().conversations.updatePreferences(conversationId, { isHidden: false });

    for (const memberId of uniqueMemberIds(options.memberIds)) {
      await this.client().conversations.addMember(conversationId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
    }

    await this.client().rooms.create({
      conversationId,
      roomId,
      roomKind,
    });

    return {
      conversationId,
      roomId,
      roomKind,
    };
  }

  async getRoom(roomId: string): Promise<RoomView> {
    return this.client().rooms.get(roomId);
  }

  async enterRoom(roomId: string): Promise<void> {
    await this.client().rooms.enter(roomId);
  }

  async leaveRoom(roomId: string): Promise<void> {
    await this.client().rooms.leave(roomId);
  }

  async postGameMove(options: PostGameMoveOptions): Promise<PostedMessageResponse> {
    const payload = serializeGameMovePayload(options.payload);
    return this.client().conversations.postMessage(options.conversationId, {
      clientMsgId: options.clientMsgId,
      summary: options.summary ?? 'game move',
      parts: [{
        kind: 'data',
        schemaRef: buildGameMoveSchemaRef(options.gameKey),
        encoding: 'application/json',
        payload,
      }],
    });
  }

  async postRoomMessage(
    conversationId: string,
    text: string,
    clientMsgId?: string,
  ): Promise<PostedMessageResponse> {
    return this.client().conversations.postText(conversationId, text, {
      clientMsgId,
      summary: text,
    });
  }
}

export function createSdkworkRoomService(
  clientFactory: () => ImSdkClient = getImSdkClientWithSession,
): RoomService {
  return new SdkworkRoomService(clientFactory);
}

export const roomService = createSdkworkRoomService();
