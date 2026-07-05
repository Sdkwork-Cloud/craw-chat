import type { MessageReadReceiptReaderView } from './message-read-receipt-reader-view';

export interface MessageReadReceiptSummaryView {
  activeMemberCount: string;
  readCount: string;
  readers: MessageReadReceiptReaderView[];
}
