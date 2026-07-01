import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

class ChatConversationService {
  ChatConversationService(this._client);

  final SdkworkImClient _client;

  Future<TimelineResponse?> fetchTimeline(
    String conversationId, {
    int limit = 50,
    int afterSeq = 0,
  }) {
    return _client.chat.conversationsMessagesList(
      conversationId,
      afterSeq,
      limit,
    );
  }

  Future<TimelineResponse?> fetchTimelineDelta(
    String conversationId,
    int afterSeq, {
    int limit = 50,
  }) {
    return fetchTimeline(conversationId, limit: limit, afterSeq: afterSeq);
  }

  Future<PostedMessageResponse?> sendText(String conversationId, String text) {
    return _client.chat.conversationsMessagesCreate(
      conversationId,
      PostMessageRequest(text: text.trim()),
    );
  }

  Future<PostedMessageResponse?> sendImageMessage({
    required String conversationId,
    required String driveUri,
    required String spaceId,
    required String nodeId,
    required String fileName,
    required String mimeType,
    required int sizeBytes,
  }) {
    return _client.chat.conversationsMessagesCreate(
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
  }
}

ChatConversationService createChatConversationService(ImSdkClientBundle bundle) {
  return ChatConversationService(bundle.imSdk);
}
