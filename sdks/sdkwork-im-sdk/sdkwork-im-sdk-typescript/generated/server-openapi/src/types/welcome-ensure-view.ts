export interface WelcomeEnsureView {
  status: 'sent' | 'already_sent' | 'already_engaged';
  conversationId: string;
  messageId: string;
  messageSeq: string;
}
