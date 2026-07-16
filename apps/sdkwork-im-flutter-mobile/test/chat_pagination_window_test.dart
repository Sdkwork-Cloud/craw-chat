import 'package:flutter_test/flutter_test.dart';
import 'package:sdkwork_im_flutter_mobile_chat/sdkwork_im_flutter_mobile_chat.dart';

void main() {
  test('preserves opaque message cursors without numeric parsing', () {
    const cursor = 'eyJzZWVrIjoibWVzc2FnZS0wMDAwMDEifQ';
    final pagination = readCursorPageInfo(
      PageInfo(mode: 'cursor', nextCursor: cursor, hasMore: true),
    );

    expect(pagination.hasMore, isTrue);
    expect(pagination.nextCursor, cursor);
  });

  test('retains an older message page before advancing its cursor', () {
    final newestMessages = List<ConversationMessageEntry>.generate(
      maxMessageHistoryEntries,
      (index) => _message(index + 51),
    );
    final olderMessages = List<ConversationMessageEntry>.generate(
      50,
      (index) => _message(index + 1),
    );

    final page = mergeConversationMessagePage(
      newestMessages,
      olderMessages,
      direction: MessageHistoryWindowDirection.older,
    );

    expect(page.incomingPageRetained, isTrue);
    expect(page.items, hasLength(maxMessageHistoryEntries));
    expect(
      olderMessages.every(
        (entry) => page.items.any(
          (retained) => retained.messageId == entry.messageId,
        ),
      ),
      isTrue,
    );
    expect(
      page.items.any((entry) => entry.messageId == 'message-550'),
      isFalse,
    );
  });

  test('rejects a message page that cannot fit in the history window', () {
    final page = mergeConversationMessagePage(
      const <ConversationMessageEntry>[],
      List<ConversationMessageEntry>.generate(
        maxMessageHistoryEntries + 1,
        (index) => _message(index + 1),
      ),
      direction: MessageHistoryWindowDirection.older,
    );

    expect(page.incomingPageRetained, isFalse);
  });

  test('caps inbox windows while retaining the newly loaded older page', () {
    final current = List<ConversationInboxEntry>.generate(
      maxInboxEntries,
      _inboxEntry,
    );
    final olderPage = List<ConversationInboxEntry>.generate(
      20,
      (index) => _inboxEntry(index + maxInboxEntries),
    );

    final page = mergeConversationInboxPage(
      current,
      olderPage,
      direction: InboxWindowDirection.older,
    );

    expect(page.incomingPageRetained, isTrue);
    expect(page.items, hasLength(maxInboxEntries));
    expect(
      olderPage.every(
        (entry) => page.items.any(
          (retained) => retained.conversationId == entry.conversationId,
        ),
      ),
      isTrue,
    );
    expect(
      page.items.any((entry) => entry.conversationId == 'conversation-0'),
      isFalse,
    );
  });

  test('rejects a page that cannot fit in the inbox window', () {
    final page = mergeConversationInboxPage(
      const <ConversationInboxEntry>[],
      List<ConversationInboxEntry>.generate(
        maxInboxEntries + 1,
        _inboxEntry,
      ),
      direction: InboxWindowDirection.older,
    );

    expect(page.incomingPageRetained, isFalse);
  });
}

ConversationMessageEntry _message(int sequence) {
  return ConversationMessageEntry(
    tenantId: 'tenant-1',
    conversationId: 'conversation-1',
    messageId: 'message-$sequence',
    messageSeq: sequence,
    sender: Sender(id: 'user-1', kind: 'user', displayName: 'Ada'),
    body: MessageBody(text: 'message $sequence', parts: <ContentPart>[]),
    messageType: 'text',
    deliveryMode: 'normal',
    occurredAt: '2026-07-16T00:00:00Z',
  );
}

ConversationInboxEntry _inboxEntry(int index) {
  return ConversationInboxEntry(
    tenantId: 'tenant-1',
    conversationId: 'conversation-$index',
    conversationType: 'direct',
    lastActivityAt: '2026-07-16T00:00:00Z',
    messageCount: 1,
    lastMessageSeq: index,
    unreadCount: 0,
  );
}
