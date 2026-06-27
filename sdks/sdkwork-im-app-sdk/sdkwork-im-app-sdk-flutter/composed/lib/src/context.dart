import 'package:im_app_api_generated/im_app_api_generated.dart';
import 'package:rtc_sdk/rtc_sdk.dart';

class ImAppSdkContext {
  final SdkworkAppClient transportClient;
  final RtcDataSource rtcDataSource;
  final String? apiBaseUrl;
  String? _authToken;
  String? _accessToken;

  ImAppSdkContext({
    required this.transportClient,
    RtcDataSource? rtcDataSource,
    this.apiBaseUrl,
    String? authToken,
    String? accessToken,
  }) : rtcDataSource = rtcDataSource ?? RtcDataSource(),
       _authToken = _normalizeToken(authToken),
       _accessToken = _normalizeToken(accessToken) {
    if (_authToken != null && _authToken!.isNotEmpty) {
      transportClient.setAuthToken(_authToken!);
    }
    if (_accessToken != null && _accessToken!.isNotEmpty) {
      transportClient.setAccessToken(_accessToken!);
    }
  }

  void setAuthToken(String token) {
    _authToken = _normalizeToken(token);
    transportClient.setAuthToken(_authToken ?? '');
  }

  void clearAuthToken() {
    _authToken = null;
    transportClient.setAuthToken('');
  }

  void setAccessToken(String token) {
    _accessToken = _normalizeToken(token);
    transportClient.setAccessToken(_accessToken ?? '');
  }

  void clearAccessToken() {
    _accessToken = null;
    transportClient.setAccessToken('');
  }

  String? get authToken => _authToken;
  String? get accessToken => _accessToken;
}

String? _normalizeToken(String? token) {
  if (token == null) {
    return null;
  }
  final trimmed = token.trim();
  if (trimmed.isEmpty) {
    return null;
  }
  return trimmed.replaceFirst(RegExp(r'^Bearer\s+', caseSensitive: false), '');
}
