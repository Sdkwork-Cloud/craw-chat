import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type MessageHistoryListParams = {
  afterSeq?: number;
  pageSize?: number;
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

type ReactionMutationCall = {
  emoji: string;
  messageId: string;
};

const messageHistoryCalls: Array<{
  conversationId: string;
  params?: MessageHistoryListParams;
}> = [];
const interactionSummaryCalls: MessageInteractionSummaryCall[] = [];
const postMessageCalls: PostMessageCall[] = [];
const reactionMutationCalls: ReactionMutationCall[] = [];

const fakeClient = {
  conversations: {
    async listMessages(
      conversationId: string,
      params?: MessageHistoryListParams,
    ) {
      messageHistoryCalls.push({ conversationId, params });
      return {
        items: [
          {
            tenantId: '100001',
            conversationId,
            messageId: 'message-1',
            messageSeq: 1,
            summary: 'message with reactions',
          },
          {
            tenantId: '100001',
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
      return {
        tenantId: '100001',
        conversationId,
        messageId,
        messageSeq: 1,
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
  async addReaction(messageId: string, emoji: string) {
    reactionMutationCalls.push({ messageId, emoji });
    return {
      conversationId: 'chat-1',
      messageId,
      reactionKey: emoji,
    };
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);
  await service.sendMessage('chat-1', 'local cached message');
  const messages = await service.getMessages('chat-1');

  assert.equal(postMessageCalls.length, 1);
  assert.deepEqual(
    messageHistoryCalls,
    [{ conversationId: 'chat-1', params: { afterSeq: 0, pageSize: 20 } }],
    'message history must use the paginated IM SDK listMessages contract',
  );
  assert.deepEqual(
    interactionSummaryCalls,
    [],
    'message history sync must NOT issue per-message interaction_summary requests while loading message history',
  );
  const reactedMessage = messages.find((message) => message.id === 'message-1');
  const plainMessage = messages.find((message) => message.id === 'message-2');

  assert.equal(
    reactedMessage?.reactions,
    undefined,
    'message history must not restore reactions from fields that ConversationMessageEntry does not define',
  );
  assert.equal(
    plainMessage?.reactions,
    undefined,
    'messages without explicit local reactions should not render empty reaction chrome',
  );

  await service.addReaction('chat-1', 'message-1', 'thumbs_up');
  const messagesAfterExplicitReaction = await service.getMessages('chat-1');
  const updatedMessage = messagesAfterExplicitReaction.find((message) => message.id === 'message-1');

  assert.deepEqual(
    reactionMutationCalls,
    [{ messageId: 'message-1', emoji: 'thumbs_up' }],
    'explicit reaction mutations must still route through the IM SDK semantic reaction method',
  );
  assert.deepEqual(
    updatedMessage?.reactions,
    [{ emoji: 'thumbs_up', count: 1, hasReacted: true }],
    'explicit local reaction mutations should update the cached message without requiring message history inline interaction fields',
  );
  assert.deepEqual(
    interactionSummaryCalls,
    [],
    'explicit reaction mutation plus history reload must not issue per-message interaction_summary requests',
  );

  console.log('sdkwork-im-pc message interaction sync contract passed');
}

void main();
