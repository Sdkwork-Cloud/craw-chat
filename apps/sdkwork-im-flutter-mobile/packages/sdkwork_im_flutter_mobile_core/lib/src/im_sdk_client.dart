import 'package:im_sdk_composed/im_sdk_composed.dart';
import 'package:im_sdk_generated/im_client.dart';

export 'package:im_sdk_generated/im_client.dart';
export 'package:im_sdk_composed/im_sdk_composed.dart';

const imApiPrefix = '/im/v3/api';

class ImSdkClientBundle {
  const ImSdkClientBundle({
    required this.imSdk,
    required this.composed,
  });

  final SdkworkImClient imSdk;
  final ImSdkComposedClient composed;
}

String resolveImApplicationBaseUrl(String configuredBaseUrl) {
  final trimmed = configuredBaseUrl.trim().replaceAll(RegExp(r'/+$'), '');
  if (trimmed.endsWith(imApiPrefix)) {
    return trimmed.substring(0, trimmed.length - imApiPrefix.length);
  }
  return trimmed;
}

ImSdkClientBundle createImSdkClient({
  required String applicationPublicHttpUrl,
  String? applicationPublicWebSocketUrl,
  String? accessToken,
  String? authToken,
  SdkworkImClient? existingClient,
  ImSdkComposedClient? existingComposedClient,
}) {
  final baseUrl = resolveImApplicationBaseUrl(applicationPublicHttpUrl);
  final websocketBaseUrl = applicationPublicWebSocketUrl != null
      ? resolveImWebSocketBaseUrl(applicationPublicWebSocketUrl)
      : resolveImWebSocketBaseUrl(applicationPublicHttpUrl);

  const headers = <String, String>{
    'x-sdkwork-platform': 'mobile',
  };

  final SdkworkImClient client;
  if (existingClient != null) {
    client = existingClient;
    if (authToken != null && authToken.isNotEmpty) {
      client.setAuthToken(authToken);
    }
    if (accessToken != null && accessToken.isNotEmpty) {
      client.setAccessToken(accessToken);
    }
    for (final entry in headers.entries) {
      client.setHeader(entry.key, entry.value);
    }
  } else {
    client = SdkworkImClient.withBaseUrl(
      baseUrl: baseUrl,
      authToken: authToken,
      accessToken: accessToken,
      headers: headers,
    );
  }

  final composed = existingComposedClient ??
      ImSdkComposedClient(
        transport: client,
        websocketBaseUrl: websocketBaseUrl,
        accessToken: accessToken,
        authToken: authToken,
        headers: headers,
      );
  if (authToken != null && authToken.isNotEmpty) {
    composed.authToken = authToken;
  }
  if (accessToken != null && accessToken.isNotEmpty) {
    composed.accessToken = accessToken;
  }

  return ImSdkClientBundle(imSdk: client, composed: composed);
}
