export interface RoomView {
  roomId: string;
  roomKind: 'live' | 'chat' | 'game';
  conversationId: string;
  activeMemberCount: number;
  maxMembers: number;
}
