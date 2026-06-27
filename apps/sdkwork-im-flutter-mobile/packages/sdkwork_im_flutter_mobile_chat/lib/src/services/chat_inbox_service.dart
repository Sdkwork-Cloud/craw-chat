import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

class ChatInboxService {
  ChatInboxService(this._client);

  final SdkworkImClient _client;

  Future<InboxResponse?> fetchInbox({int limit = 30}) {
    return _client.chat.inboxRetrieve(limit);
  }
}

ChatInboxService createChatInboxService(ImSdkClientBundle bundle) {
  return ChatInboxService(bundle.imSdk);
}
