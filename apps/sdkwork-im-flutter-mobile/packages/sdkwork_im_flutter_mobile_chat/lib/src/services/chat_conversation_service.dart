import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

class ChatConversationService {
  ChatConversationService(this._client);

  final SdkworkImClient _client;

  Future<TimelineResponse?> fetchTimeline(String conversationId, {int limit = 50}) {
    return _client.chat.conversationsMessagesList(conversationId, null, limit);
  }

  Future<PostedMessageResponse?> sendText(String conversationId, String text) {
    return _client.chat.conversationsMessagesCreate(
      conversationId,
      PostMessageRequest(text: text.trim()),
    );
  }
}

ChatConversationService createChatConversationService(ImSdkClientBundle bundle) {
  return ChatConversationService(bundle.imSdk);
}
