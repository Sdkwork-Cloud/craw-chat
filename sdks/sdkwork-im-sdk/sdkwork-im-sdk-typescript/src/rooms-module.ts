import type {
  CreateConversationResult,
  CreateRoomRequest,
  EnterRoomResponse,
  RoomView,
} from '@sdkwork/im-sdk-generated';
import type { ImTransportClientLike } from './transport-client-like';

export class ImRoomsModule {
  constructor(private readonly transportClient: ImTransportClientLike) {}

  create(body: CreateRoomRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.rooms.create(body);
  }

  get(roomId: string | number): Promise<RoomView> {
    return this.transportClient.chat.rooms.get(roomId);
  }

  enter(roomId: string | number): Promise<EnterRoomResponse> {
    return this.transportClient.chat.rooms.enter(roomId);
  }

  leave(roomId: string | number): Promise<EnterRoomResponse> {
    return this.transportClient.chat.rooms.leave(roomId);
  }
}
