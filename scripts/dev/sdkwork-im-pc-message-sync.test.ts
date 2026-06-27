import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

class TestBlobLike extends Blob {
  readonly name: string;

  constructor(content: BlobPart[], name: string, options: BlobPropertyBag = {}) {
    super(content, options);
    this.name = name;
  }
}

const calls: Array<{
  body?: Record<string, unknown>;
  conversationId?: string;
  messageId?: string;
  method: string;
  params?: {
    afterSeq?: number;
    limit?: number;
  };
}> = [];

const driveUploadCalls: Array<Record<string, unknown>> = [];

function lastMessageCreateCall(): typeof calls[number] | undefined {
  return calls.filter((call) => call.method === 'chat.conversations.messages.create').at(-1);
}

const timelinePages = [
  {
    items: [
      {
        conversationId: 'chat-1',
        messageId: 'message-1',
        messageSeq: 1,
        summary: 'first page message',
        sender: {
          id: 'u_alice',
          kind: 'user',
          metadata: {},
        },
        body: {
          summary: 'first page message',
          parts: [{ kind: 'text', text: 'first page message' }],
          renderHints: {
            sdkworkChatPcType: 'text',
          },
        },
        messageType: 'standard',
        deliveryMode: 'discrete',
        occurredAt: '2026-06-04T10:00:00.000Z',
      },
    ],
    nextAfterSeq: 1,
    hasMore: true,
  },
  {
    items: [
      {
        conversationId: 'chat-1',
        messageId: 'message-2',
        messageSeq: 2,
        summary: 'second page message',
        sender: {
          id: 'u_bob',
          kind: 'user',
          metadata: {},
        },
        body: {
          summary: 'second page message',
          parts: [{ kind: 'text', text: 'second page body text' }],
          renderHints: {
            sdkworkChatPcType: 'text',
          },
          replyTo: {
            messageId: 'message-1',
            senderDisplayName: 'Alice',
            contentPreview: 'first page message',
          },
        },
        messageType: 'standard',
        deliveryMode: 'discrete',
        occurredAt: '2026-06-04T10:00:05.000Z',
      },
    ],
    hasMore: false,
  },
];

const fakeClient = {
  chat: {
    inbox: {
      async retrieve() {
        calls.push({ method: 'chat.inbox.retrieve' });
        return {
          hasMore: false,
          items: [
            {
              conversationId: 'chat-1',
              conversationType: 'group',
              lastActivityAt: '2026-06-04T10:00:10.000Z',
              lastMessageSeq: 3,
              messageCount: 3,
              tenantId: '100001',
              unreadCount: 0,
            },
          ],
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      calls.push({ method: 'conversations.getPreferences', conversationId });
      return {
        conversationId,
        isHidden: false,
        isMarkedUnread: false,
        isMuted: false,
        isPinned: false,
        principalId: 'current-user',
        principalKind: 'user',
        tenantId: '100001',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async getProfile(conversationId: string) {
      calls.push({ method: 'conversations.getProfile', conversationId });
      return {
        avatarUrl: `https://cdn.example.test/${conversationId}.png`,
        conversationId,
        displayName: 'Backend Group',
        notice: '',
        tenantId: '100001',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async postText(
      conversationId: string,
      text: string,
      body: Record<string, unknown>,
    ) {
      calls.push({ method: 'chat.conversations.messages.create', conversationId, body });
      return {
        messageId: 'message-3',
        messageSeq: 3,
        eventId: 'event-3',
      };
    },
    async postMessage(
      conversationId: string,
      body: Record<string, unknown>,
    ) {
      calls.push({ method: 'chat.conversations.messages.create', conversationId, body });
      return {
        messageId: 'message-4',
        messageSeq: 4,
        eventId: 'event-4',
      };
    },
    async listMessages(
      conversationId: string,
      params?: {
        afterSeq?: number;
        limit?: number;
      },
    ) {
      calls.push({ method: 'chat.conversations.messages.list', conversationId, params });
      return timelinePages[calls.filter((call) => call.method === 'chat.conversations.messages.list').length - 1];
    },
  },
  messages: {
    async deleteForMe(messageId: string) {
      calls.push({ method: 'chat.messages.visibility.delete', messageId });
      return {
        tenantId: 't-demo',
        conversationId: 'chat-1',
        messageId,
        messageSeq: 1,
        principalKind: 'user',
        principalId: 'u_alice',
        isDeleted: true,
        updatedAt: '2026-06-04T10:00:10.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

const fakeDriveUploader = {
  async uploadImage(request: Record<string, unknown>) {
    driveUploadCalls.push({ method: 'uploadImage', ...request });
    return buildDriveUploadResult('space-im-chat-1', 'node-image-1', request);
  },
  async uploadAudio(request: Record<string, unknown>) {
    driveUploadCalls.push({ method: 'uploadAudio', ...request });
    return buildDriveUploadResult('space-im-chat-1', 'node-voice-1', request);
  },
  async uploadAttachment(request: Record<string, unknown>) {
    driveUploadCalls.push({ method: 'uploadAttachment', ...request });
    return buildDriveUploadResult('space-im-chat-1', 'node-file-1', request);
  },
  async uploadVideo(request: Record<string, unknown>) {
    driveUploadCalls.push({ method: 'uploadVideo', ...request });
    return buildDriveUploadResult('space-im-chat-1', 'node-video-1', request);
  },
};

function buildDriveUploadResult(
  spaceId: string,
  nodeId: string,
  request: Record<string, unknown>,
) {
  return {
    uploadItem: {
      id: `upload-${nodeId}`,
      tenantId: request.tenantId,
      organizationId: request.organizationId,
      userId: request.userId,
      appId: request.appId,
      appResourceType: request.appResourceType,
      appResourceId: request.appResourceId,
      scene: request.scene,
      source: request.source,
      uploadProfileCode: request.uploadProfileCode,
      spaceId,
      nodeId,
      originalFileName: request.originalFileName,
      contentType: request.contentType,
      contentLength: String((request.file as Blob).size),
    },
    uploadSession: {
      id: `session-${nodeId}`,
      tenantId: request.tenantId,
      spaceId,
      nodeId,
      state: 'completed',
    },
    parts: [],
  };
}

function assertLastMediaPost({
  content,
  coverUrl,
  duration,
  fileName,
  fileSize,
  mediaKind,
  messageType,
  sizeBytes,
  driveNodeId,
}: {
  content: string;
  coverUrl?: string;
  duration?: number;
  fileName: string;
  fileSize?: string;
  mediaKind: string;
  messageType: 'file' | 'image' | 'video' | 'voice';
  sizeBytes?: string;
  driveNodeId: string;
}): void {
  const body = calls.at(-1)?.body as Record<string, unknown>;
  const parts = body.parts as Array<Record<string, unknown>>;
  const part = parts[0];
  const drive = part.drive as Record<string, unknown>;
  const resource = part.resource as Record<string, unknown>;
  assert.equal(
    body.text,
    undefined,
    'PC media send must not persist local object URLs as plain text message bodies',
  );
  assert.equal(part.kind, 'media');
  assert.equal(part.mediaRole, 'attachment');
  assert.equal(drive.driveUri, resource.uri);
  assert.equal(drive.spaceId, 'space-im-chat-1');
  assert.equal(drive.nodeId, driveNodeId);
  assert.equal(resource.kind, mediaKind);
  assert.equal(resource.source, 'drive');
  assert.equal(
    resource.publicUrl,
    undefined,
    'PC media send must not persist local preview URLs as Drive delivery URLs',
  );
  assert.equal(
    resource.url,
    undefined,
    'PC media send must not persist local preview URLs as Drive delivery URLs',
  );
  assert.equal(
    resource.uri,
    drive.driveUri,
    'PC media send must persist the stable Drive URI as the media resource identity',
  );
  assert.equal(resource.fileName, fileName);
  if (sizeBytes !== undefined) {
    assert.equal(resource.sizeBytes, sizeBytes);
  }
  if (duration !== undefined) {
    assert.equal(resource.durationSeconds, duration);
  }
  assert.deepEqual(
    body.renderHints,
    {
      ...(duration ? { duration: String(duration) } : {}),
      ...(fileName ? { fileName } : {}),
      ...(fileSize ? { fileSize } : {}),
      sdkworkChatPcType: messageType,
    },
    'PC media send must preserve UI metadata through render hints without changing visual components',
  );
  if (coverUrl) {
    assert.equal(
      body.renderHints.coverUrl,
      undefined,
      'PC media send must not persist local preview cover URLs as message render hints',
    );
  }
}

function assertLastDriveUpload({
  contentType,
  fileName,
  method,
}: {
  contentType: string;
  fileName: string;
  method: string;
}): void {
  const upload = driveUploadCalls.at(-1);
  assert.equal(upload?.method, method);
  assert.equal(upload?.tenantId, 't_session');
  assert.equal(upload?.organizationId, 'org_session');
  assert.equal(upload?.userId, 'u_session');
  assert.equal(upload?.appId, 'chat');
  assert.equal(upload?.appResourceType, 'im_conversation');
  assert.equal(upload?.appResourceId, 'chat-1');
  assert.equal(upload?.scene, 'im');
  assert.equal(upload?.source, 'chat_message');
  assert.equal(upload?.uploadProfileCode, method === 'uploadAttachment' ? 'attachment' : undefined);
  assert.equal(upload?.originalFileName, fileName);
  assert.equal(upload?.contentType, contentType);
}

async function main(): Promise<void> {
  const service = createSdkworkChatService({
    getClient: () => fakeClient,
    getDriveUploader: () => fakeDriveUploader,
    getSession: () => ({
      accessToken: 'header.eyJ0ZW5hbnRJZCI6InRfc2Vzc2lvbiIsIm9yZ2FuaXphdGlvbklkIjoib3JnX3Nlc3Npb24iLCJ1c2VySWQiOiJ1X3Nlc3Npb24ifQ.signature',
      authToken: 'header.eyJ0ZW5hbnRJZCI6InRfc2Vzc2lvbiIsIm9yZ2FuaXphdGlvbklkIjoib3JnX3Nlc3Npb24iLCJ1c2VySWQiOiJ1X3Nlc3Npb24ifQ.signature',
    }),
  });
  const messages = await service.getMessages('chat-1');

  assert.equal(calls.length, 2, 'chat message history must continue until the IM SDK timeline window is exhausted');
  assert.deepEqual(
    calls,
    [
      { method: 'chat.conversations.messages.list', conversationId: 'chat-1', params: { afterSeq: 0, limit: 50 } },
      { method: 'chat.conversations.messages.list', conversationId: 'chat-1', params: { afterSeq: 1, limit: 50 } },
    ],
  );
  assert.deepEqual(
    messages.map((message) => [
      message.id,
      message.senderId,
      message.content,
      message.type,
      message.timestamp,
      message.replyTo,
    ]),
    [
      ['message-1', 'u_alice', 'first page message', 'text', Date.parse('2026-06-04T10:00:00.000Z'), undefined],
      [
        'message-2',
        'u_bob',
        'second page body text',
        'text',
        Date.parse('2026-06-04T10:00:05.000Z'),
        { id: 'message-1', senderName: 'Alice', content: 'first page message' },
      ],
    ],
  );

  const replyTo = { id: 'message-2', senderName: 'Bob', content: 'second page body text' };
  const locallyNotifiedMessages: string[] = [];
  const unsubscribeMessages = service.subscribeMessages('chat-1', (message) => {
    locallyNotifiedMessages.push(`${message.id}:${message.content}`);
  });
  const chatListSnapshots: string[][] = [];
  const unsubscribeChats = service.subscribeChats((chats) => {
    chatListSnapshots.push(chats.map((chat) => `${chat.id}:${chat.lastMessage?.content ?? ''}`));
  });
  const postedReply = await service.sendMessage('chat-1', 'reply text', 'text', replyTo);
  await Promise.resolve();
  const postedReplyCreateCall = lastMessageCreateCall();
  assert.deepEqual(
    postedReplyCreateCall,
    {
      method: 'chat.conversations.messages.create',
      conversationId: 'chat-1',
      body: {
        clientMsgId: (postedReplyCreateCall as { body?: { clientMsgId?: string } } | undefined)?.body?.clientMsgId,
        summary: 'reply text',
        replyTo: {
          messageId: 'message-2',
          senderDisplayName: 'Bob',
          contentPreview: 'second page body text',
        },
      },
    },
    'PC sendMessage must persist reply references through the standard IM message body',
  );
  assert.deepEqual(
    postedReply.replyTo,
    replyTo,
    'PC sendMessage must keep reply preview in the local optimistic message',
  );
  assert.deepEqual(
    locallyNotifiedMessages,
    ['message-3:reply text'],
    'PC sendMessage must notify local opened-conversation subscribers immediately after the SDK accepts the message',
  );
  assert.deepEqual(
    chatListSnapshots.at(-1),
    ['chat-1:reply text'],
    'PC sendMessage must refresh chat-list subscribers with the accepted local message before waiting for realtime echo',
  );
  unsubscribeMessages();
  unsubscribeChats();

  await assert.rejects(
    () => service.sendMessage('chat-1', 'data:image/png;base64,local-preview', 'image', undefined, {
      fileName: 'missing-upload-file.png',
      fileSize: '4 KB',
    }),
    /require a File or Blob/u,
    'PC media send must fail closed when no real file/blob is available for Drive upload',
  );
  assert.equal(
    (lastMessageCreateCall()?.body as Record<string, unknown> | undefined)?.summary,
    'reply text',
    'PC media send without a Drive upload source must not post a fake IM media message',
  );

  const postedImage = await service.sendMessage('chat-1', 'blob://local-image-1', 'image', undefined, {
    fileName: 'local-image.png',
    fileSize: '4.0 KB',
    coverUrl: 'blob://local-image-cover',
    file: new TestBlobLike(['image'], 'local-image.png', { type: 'image/png' }),
  });
  assertLastDriveUpload({
    contentType: 'image/png',
    fileName: 'local-image.png',
    method: 'uploadImage',
  });
  assertLastMediaPost({
    content: 'blob://local-image-1',
    coverUrl: 'blob://local-image-cover',
    driveNodeId: 'node-image-1',
    fileName: 'local-image.png',
    fileSize: '4.0 KB',
    mediaKind: 'image',
    messageType: 'image',
    sizeBytes: '5',
  });
  assert.equal(postedImage.id, 'message-4');
  assert.equal(postedImage.type, 'image');

  const postedVoice = await service.sendMessage('chat-1', 'blob://local-voice-1', 'voice', undefined, {
    duration: 12,
    file: new TestBlobLike(['voice-data'], 'voice-message.ogg', { type: 'audio/ogg' }),
    fileName: 'voice-message.ogg',
    fileSize: '8192',
  });
  assertLastDriveUpload({
    contentType: 'audio/ogg',
    fileName: 'voice-message.ogg',
    method: 'uploadAudio',
  });
  assertLastMediaPost({
    content: 'blob://local-voice-1',
    driveNodeId: 'node-voice-1',
    duration: 12,
    fileName: 'voice-message.ogg',
    fileSize: '8192',
    mediaKind: 'voice',
    messageType: 'voice',
    sizeBytes: '10',
  });
  assert.equal(postedVoice.type, 'voice');

  const postedFile = await service.sendMessage('chat-1', 'blob://local-file-1', 'file', undefined, {
    file: new TestBlobLike(['report-data'], 'quarterly-report.pdf', { type: 'application/pdf' }),
    fileName: 'quarterly-report.pdf',
    fileSize: '1.5 MB',
  });
  assertLastDriveUpload({
    contentType: 'application/pdf',
    fileName: 'quarterly-report.pdf',
    method: 'uploadAttachment',
  });
  assertLastMediaPost({
    content: 'blob://local-file-1',
    driveNodeId: 'node-file-1',
    fileName: 'quarterly-report.pdf',
    fileSize: '1.5 MB',
    mediaKind: 'file',
    messageType: 'file',
    sizeBytes: '11',
  });
  assert.equal(postedFile.type, 'file');

  const postedVideo = await service.sendMessage('chat-1', 'blob://local-video-1', 'video', undefined, {
    coverUrl: 'blob://local-video-cover',
    duration: 42,
    file: new TestBlobLike(['video-data'], 'demo-video.mp4', { type: 'video/mp4' }),
    fileName: 'demo-video.mp4',
    fileSize: '3 MB',
  });
  assertLastDriveUpload({
    contentType: 'video/mp4',
    fileName: 'demo-video.mp4',
    method: 'uploadVideo',
  });
  assertLastMediaPost({
    content: 'blob://local-video-1',
    coverUrl: 'blob://local-video-cover',
    driveNodeId: 'node-video-1',
    duration: 42,
    fileName: 'demo-video.mp4',
    fileSize: '3 MB',
    mediaKind: 'video',
    messageType: 'video',
    sizeBytes: '10',
  });
  assert.equal(postedVideo.type, 'video');

  await assert.rejects(
    () => service.forwardMessages(['chat-2'], [postedImage]),
    /requires a reusable Drive reference/u,
    'PC media forwarding must fail closed until the original Drive reference can be reused or copied through a real backend capability',
  );
  assert.equal(
    driveUploadCalls.length,
    4,
    'PC media forwarding without a file must not start a fake Drive upload',
  );

  await service.sendMessage('chat-1', 'https://example.com/promo', 'link', undefined, {
    coverUrl: 'https://example.com/promo.png',
    desc: 'Promotion',
    fileName: 'Promo',
  });
  const linkBody = lastMessageCreateCall()?.body as Record<string, unknown>;
  assert.equal(
    linkBody.text,
    undefined,
    'structured link messages should be sent as typed data parts rather than plain text bodies',
  );
  assert.deepEqual(
    linkBody.parts,
    [
      {
        kind: 'data',
        schemaRef: 'urn:sdkwork:sdkwork-im:message:link',
        encoding: 'application/json',
        payload: JSON.stringify({
          title: 'Promo',
          url: 'https://example.com/promo',
          description: 'Promotion',
          coverUrl: 'https://example.com/promo.png',
        }),
      },
    ],
    'structured link messages must not be wrapped as Drive media attachments',
  );
  assert.deepEqual(linkBody.renderHints, {
    coverUrl: 'https://example.com/promo.png',
    desc: 'Promotion',
    fileName: 'Promo',
    sdkworkChatPcType: 'link',
  });

  await service.deleteMessage('chat-1', 'message-1');
  assert.deepEqual(
    calls.at(-1),
    { method: 'chat.messages.visibility.delete', messageId: 'message-1' },
    'PC deleteMessage must persist current-user message deletion through the standard IM SDK visibility API',
  );

  console.log('sdkwork-im-pc message sync contract passed');
}

void main();
