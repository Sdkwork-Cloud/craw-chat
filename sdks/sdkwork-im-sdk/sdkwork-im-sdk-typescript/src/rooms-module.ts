import type {
  CreateConversationResult,
  EnterRoomResponse,
  RoomView,
} from '../generated/server-openapi/dist/index.js';
import { requireStringIdentifier } from './identifier-boundary.js';
import type { ImCreateRoomRequest, ImTransportClientLike } from './transport-client-like.js';

export class ImRoomsModule {
  constructor(private readonly transportClient: ImTransportClientLike) {}

  create(body: ImCreateRoomRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.rooms.create(body);
  }

  get(roomId: string): Promise<RoomView> {
    return this.transportClient.chat.rooms.retrieve(
      requireStringIdentifier(roomId, 'roomId'),
    );
  }

  enter(roomId: string): Promise<EnterRoomResponse> {
    return this.transportClient.chat.rooms.enter(
      requireStringIdentifier(roomId, 'roomId'),
    );
  }

  leave(roomId: string): Promise<EnterRoomResponse> {
    return this.transportClient.chat.rooms.leave(
      requireStringIdentifier(roomId, 'roomId'),
    );
  }
}
