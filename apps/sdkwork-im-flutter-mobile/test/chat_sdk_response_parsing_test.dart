import 'package:flutter_test/flutter_test.dart';
import 'package:sdkwork_im_flutter_mobile_chat/sdkwork_im_flutter_mobile_chat.dart';

void main() {
  test('reads message history data and pagination from SDK response data', () {
    final response = ConversationMessageListResponse(
      code: 0,
      data: <String, dynamic>{
        'items': <Map<String, dynamic>>[
          _messageEntry(41).toJson(),
          _messageEntry(42).toJson(),
        ],
        'pageInfo': PageInfo(
          mode: 'cursor',
          nextCursor: '42',
          hasMore: true,
        ).toJson(),
      },
      traceId: 'trace-message-history',
    );

    final page = readMessageHistoryPageFromSdkResponse(response);

    expect(page.items.map((entry) => entry.messageSeq), <int>[41, 42]);
    expect(page.pagination.hasMore, isTrue);
    expect(page.pagination.nextAfterSeq, 42);
  });

  test('reads inbox data and cursor page info from SDK response data', () {
    final response = InboxListResponse(
      code: 0,
      data: <String, dynamic>{
        'items': <Map<String, dynamic>>[
          _inboxEntry('c_1').toJson(),
        ],
        'pageInfo': PageInfo(
          mode: 'cursor',
          nextCursor: 'cursor-2',
          hasMore: true,
        ).toJson(),
      },
      traceId: 'trace-inbox',
    );

    final page = readInboxPageFromSdkResponse(response);

    expect(page.items.single.conversationId, 'c_1');
    expect(page.pageInfo.hasMore, isTrue);
    expect(page.pageInfo.nextCursor, 'cursor-2');
  });

  test('reads created message item from SDK command response data', () {
    final posted = PostMessageResult(
      messageId: 'm_1',
      messageSeq: 7,
      eventId: 'evt_1',
      deliveryStatus: 'applied',
    );
    final response = ConversationsMessagesCreateResponse201(
      code: 0,
      data: <String, dynamic>{
        'item': posted.toJson(),
      },
      traceId: 'trace-post',
    );

    final item = readPostMessageResultFromSdkResponse(response);

    expect(item?.messageId, 'm_1');
    expect(item?.messageSeq, 7);
  });
}

ConversationMessageEntry _messageEntry(int messageSeq) {
  return ConversationMessageEntry(
    tenantId: 'tenant-1',
    conversationId: 'c_1',
    messageId: 'm_$messageSeq',
    messageSeq: messageSeq,
    sender: Sender(id: 'u_1', kind: 'user', displayName: 'Ada'),
    body: MessageBody(text: 'message $messageSeq', parts: <ContentPart>[]),
    messageType: 'text',
    deliveryMode: 'normal',
    occurredAt: '2026-07-07T00:00:00Z',
  );
}

ConversationInboxEntry _inboxEntry(String conversationId) {
  return ConversationInboxEntry(
    tenantId: 'tenant-1',
    conversationId: conversationId,
    conversationType: 'direct',
    displayName: 'Ada',
    lastActivityAt: '2026-07-07T00:00:00Z',
    messageCount: 1,
    lastMessageSeq: 7,
    unreadCount: 1,
  );
}
