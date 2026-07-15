import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'chat_message_history_utils.dart';

class ChatConversationService {
  ChatConversationService(this._client);

  final SdkworkImClient _client;

  Future<ChatMessageHistoryResult> fetchMessageHistory(
    String conversationId, {
    int pageSize = 50,
    int afterSeq = 0,
  }) async {
    final response = await _client.chat.conversationsMessagesList(
      conversationId,
      afterSeq > 0 ? afterSeq.toString() : null,
      pageSize,
    );
    return readMessageHistoryPageFromSdkResponse(response);
  }

  Future<ChatMessageHistoryResult> fetchMessageHistoryDelta(
    String conversationId,
    int afterSeq, {
    int pageSize = 50,
  }) {
    return fetchMessageHistory(
      conversationId,
      pageSize: pageSize,
      afterSeq: afterSeq,
    );
  }

  Future<PostMessageResult?> sendText(
    String conversationId,
    String text, {
    String? clientMsgId,
  }) async {
    final response = await _client.chat.conversationsMessagesCreate(
      conversationId,
      PostMessageRequest(
        text: text.trim(),
        clientMsgId: clientMsgId,
      ),
    );
    return readPostMessageResultFromSdkResponse(response);
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

  Future<PostMessageResult?> sendImageMessage({
    required String conversationId,
    required String driveUri,
    required String spaceId,
    required String nodeId,
    required String fileName,
    required String mimeType,
    required int sizeBytes,
  }) async {
    final response = await _client.chat.conversationsMessagesCreate(
      conversationId,
      PostMessageRequest(
        clientMsgId: 'flutter-${DateTime.now().millisecondsSinceEpoch}',
        summary: fileName,
        parts: [
          MediaContentPart(
            kind: 'media',
            drive: DriveReference(
              driveUri: driveUri,
              spaceId: spaceId,
              nodeId: nodeId,
            ),
            resource: MediaResource(
              source: 'drive',
              uri: driveUri,
              fileName: fileName,
              mimeType: mimeType,
              sizeBytes: '$sizeBytes',
              kind: 'image',
            ),
            mediaRole: 'attachment',
          ),
        ],
      ),
    );
    return readPostMessageResultFromSdkResponse(response);
  }
}

ChatConversationService createChatConversationService(
    ImSdkClientBundle bundle) {
  return ChatConversationService(bundle.imSdk);
}
