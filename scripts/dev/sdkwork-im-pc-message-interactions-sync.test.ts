import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type TimelineListParams = {
  afterSeq?: number;
  limit?: number;
};

type MessageInteractionSummaryCall = {
  conversationId: string;
  messageId: string;
};

type PostMessageCall = {
  conversationId: string;
  body: {
    clientMsgId?: string;
    summary?: string;
    text?: string;
  };
};

const timelineCalls: Array<{
  conversationId: string;
  params?: TimelineListParams;
}> = [];
const interactionSummaryCalls: MessageInteractionSummaryCall[] = [];
const postMessageCalls: PostMessageCall[] = [];

const fakeClient = {
  conversations: {
    async listMessages(
      conversationId: string,
      params?: TimelineListParams,
    ) {
      timelineCalls.push({ conversationId, params });
      return {
        items: [
          {
            conversationId,
            messageId: 'message-1',
            messageSeq: 1,
            summary: 'message with reactions',
          },
          {
            conversationId,
            messageId: 'message-2',
            messageSeq: 2,
            summary: 'message without reactions',
          },
        ],
        hasMore: false,
      };
    },
    async getMessageInteractionSummary(
      conversationId: string,
      messageId: string,
    ) {
      interactionSummaryCalls.push({ conversationId, messageId });
      if (messageId === 'message-1') {
        return {
          tenantId: '100001',
          conversationId,
          messageId,
          messageSeq: 1,
          totalReactionCount: 3,
          reactionCounts: [
            { reactionKey: 'thumbs_up', count: 2 },
            { reactionKey: 'heart', count: 1 },
          ],
        };
      }
      return {
        tenantId: '100001',
        conversationId,
        messageId,
        messageSeq: 2,
        totalReactionCount: 0,
        reactionCounts: [],
      };
    },
    async postText(
      conversationId: string,
      text: string,
      body: PostMessageCall['body'],
    ) {
      postMessageCalls.push({ conversationId, body: { ...body, text } });
      return {
        conversationId,
        messageId: 'message-1',
        messageSeq: 1,
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);
  await service.sendMessage('chat-1', 'local cached message');
  const messages = await service.getMessages('chat-1');

  assert.equal(postMessageCalls.length, 1);
  assert.deepEqual(
    timelineCalls,
    [{ conversationId: 'chat-1', params: { afterSeq: 0, limit: 50 } }],
    'message history must still use the paginated IM SDK timeline contract',
  );
  assert.deepEqual(
    interactionSummaryCalls,
    [
      { conversationId: 'chat-1', messageId: 'message-1' },
      { conversationId: 'chat-1', messageId: 'message-2' },
    ],
    'message history sync must load backend interaction summaries for each timeline message',
  );
  const reactedMessage = messages.find((message) => message.id === 'message-1');
  const plainMessage = messages.find((message) => message.id === 'message-2');

  assert.deepEqual(reactedMessage?.reactions, [
    { emoji: 'thumbs_up', count: 2, hasReacted: false },
    { emoji: 'heart', count: 1, hasReacted: false },
  ]);
  assert.equal(
    plainMessage?.reactions,
    undefined,
    'messages without projected reaction counts should not render empty reaction chrome',
  );

  console.log('sdkwork-im-pc message interaction sync contract passed');
}

void main();
