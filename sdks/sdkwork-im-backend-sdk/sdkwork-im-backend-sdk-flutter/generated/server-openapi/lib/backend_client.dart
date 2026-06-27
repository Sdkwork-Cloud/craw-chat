import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/ops.dart';
import 'src/api/audit.dart';
import 'src/api/automation.dart';
import 'src/api/control.dart';
import 'src/api/admin.dart';

class SdkworkImBackendClient {
  final HttpClient _httpClient;

  late final OpsApi ops;
  late final AuditApi audit;
  late final AutomationApi automation;
  late final ControlApi control;
  late final AdminApi admin;

  SdkworkImBackendClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    ops = OpsApi(_httpClient);
    audit = AuditApi(_httpClient);
    automation = AutomationApi(_httpClient);
    control = ControlApi(_httpClient);
    admin = AdminApi(_httpClient);
  }

  factory SdkworkImBackendClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkImBackendClient(
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

typedef SdkworkBackendClient = SdkworkImBackendClient;
