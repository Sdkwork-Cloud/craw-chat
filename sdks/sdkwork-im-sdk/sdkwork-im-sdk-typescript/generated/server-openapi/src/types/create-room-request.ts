export interface CreateRoomRequest {
  conversationId: string;
  roomId: string;
  roomKind: 'live' | 'chat' | 'game';
}
