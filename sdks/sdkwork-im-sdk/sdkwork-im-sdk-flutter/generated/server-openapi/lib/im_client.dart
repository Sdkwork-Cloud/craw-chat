import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/presence.dart';
import 'src/api/realtime.dart';
import 'src/api/calls.dart';
import 'src/api/social.dart';
import 'src/api/chat.dart';
import 'src/api/streams.dart';
import 'src/api/spaces.dart';

class SdkworkImClient {
  final HttpClient _httpClient;

  late final PresenceApi presence;
  late final RealtimeApi realtime;
  late final CallsApi calls;
  late final SocialApi social;
  late final ChatApi chat;
  late final StreamsApi streams;
  late final SpacesApi spaces;

  SdkworkImClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    presence = PresenceApi(_httpClient);
    realtime = RealtimeApi(_httpClient);
    calls = CallsApi(_httpClient);
    social = SocialApi(_httpClient);
    chat = ChatApi(_httpClient);
    streams = StreamsApi(_httpClient);
    spaces = SpacesApi(_httpClient);
  }

  factory SdkworkImClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkImClient(
      config: SdkConfig(
        baseUrl: baseUrl,
        timeout: timeout,
        headers: headers ?? const {},
        authToken: authToken,
        accessToken: accessToken,
      ),
    );
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _httpClient.setAccessToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }
}
