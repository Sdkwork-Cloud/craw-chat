export interface PostMessageResult {
  messageId: string;
  messageSeq: number;
  eventId: string;
  requestKey?: string;
  deliveryStatus: 'applied' | 'replayed';
  proofVersion?: string;
}
