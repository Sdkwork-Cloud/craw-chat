import 'dart:typed_data';

import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

const _chatDriveAppResourceType = 'im_conversation';
const _chatDriveScene = 'im';
const _chatDriveSource = 'chat_message';

Future<DriveUploadReference> uploadChatMediaBytes({
  required String applicationPublicHttpUrl,
  required String conversationId,
  required Uint8List bytes,
  required String userId,
  String? accessToken,
  String? authToken,
  String type = 'image',
  String? originalFileName,
  String? contentType,
}) async {
  final client = DriveAppSdkClient.create(
    applicationPublicHttpUrl: applicationPublicHttpUrl,
    accessToken: accessToken,
    authToken: authToken,
  );
  final request = DriveUploaderRequest(
    bytes: bytes,
    userId: userId,
    appResourceType: _chatDriveAppResourceType,
    appResourceId: conversationId,
    scene: _chatDriveScene,
    source: _chatDriveSource,
    originalFileName: originalFileName,
    contentType: contentType,
    uploadProfileCode: type == 'image' ? 'image' : 'attachment',
  );
  return type == 'image'
      ? client.uploadImage(request)
      : client.uploadAttachment(request);
}

String resolveDriveApplicationBaseUrl(String configuredBaseUrl) {
  return resolveImApplicationBaseUrl(configuredBaseUrl);
}
