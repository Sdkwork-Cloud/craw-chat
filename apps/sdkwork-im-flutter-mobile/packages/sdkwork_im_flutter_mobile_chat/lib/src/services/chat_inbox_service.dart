import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'chat_sdk_response_utils.dart';

const int inboxPageSize = 20;
const int maxInboxSyncPages = 10;

class ChatInboxResult {
  const ChatInboxResult({
    required this.items,
    required this.pageInfo,
  });

  final List<ConversationInboxEntry> items;
  final PageInfo pageInfo;
}

class ChatInboxService {
  ChatInboxService(this._client);

  final SdkworkImClient _client;

  Future<ChatInboxResult> fetchInbox({int pageSize = inboxPageSize}) {
    return fetchInboxPage(pageSize: pageSize);
  }

  Future<ChatInboxResult> fetchInboxPage({
    int pageSize = inboxPageSize,
    String? cursor,
  }) async {
    final response = await _client.chat.inboxList(pageSize, cursor);
    return readInboxPageFromSdkResponse(response);
  }

  Future<List<ConversationInboxEntry>> fetchInboxEntries({
    int pageSize = inboxPageSize,
    int maxPages = maxInboxSyncPages,
  }) async {
    final items = <ConversationInboxEntry>[];
    String? cursor;
    for (var page = 0; page < maxPages; page += 1) {
      final response = await fetchInboxPage(pageSize: pageSize, cursor: cursor);
      final pageItems = response.items;
      items.addAll(pageItems);
      final pageInfo = response.pageInfo;
      final hasMore = pageInfo.hasMore ?? false;
      final nextCursor = pageInfo.nextCursor;
      if (!hasMore || nextCursor == null || nextCursor.isEmpty) {
        break;
      }
      cursor = nextCursor;
    }
    return items;
  }

  Future<void> markConversationRead(
    String conversationId, {
    int readSeq = 0,
  }) async {
    if (readSeq > 0) {
      await _client.chat.conversationsReadCursorUpdate(
        conversationId,
        UpdateReadCursorRequest(readSeq: readSeq),
      );
    }
    await _client.chat.conversationsPreferencesUpdate(
      conversationId,
      UpdateConversationPreferencesRequest(isMarkedUnread: false),
    );
  }
}

ChatInboxService createChatInboxService(ImSdkClientBundle bundle) {
  return ChatInboxService(bundle.imSdk);
}

ChatInboxResult readInboxPageFromSdkResponse(InboxListResponse? response) {
  return ChatInboxResult(
    items: readItemsFromSdkData(
      response?.data,
      ConversationInboxEntry.fromJson,
    ),
    pageInfo: readPageInfoFromSdkData(response?.data),
  );
}
