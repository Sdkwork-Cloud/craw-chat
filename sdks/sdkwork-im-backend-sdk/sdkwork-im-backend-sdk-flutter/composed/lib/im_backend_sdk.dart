library im_backend_sdk;

export 'package:im_backend_api_generated/im_backend_api_generated.dart';

export 'src/admin_module.dart';
export 'src/audit_module.dart';
export 'src/automation_module.dart';
export 'src/context.dart';
export 'src/control_module.dart';
export 'src/ops_module.dart';
export 'src/types.dart';

import 'package:im_backend_api_generated/im_backend_api_generated.dart';

import 'src/admin_module.dart';
import 'src/audit_module.dart';
import 'src/automation_module.dart';
import 'src/context.dart';
import 'src/control_module.dart';
import 'src/ops_module.dart';
import 'src/types.dart';

class ImBackendSdkClient {
  final ImBackendSdkContext _context;
  final SdkworkBackendClient transportClient;

  late final ImBackendOpsModule ops;
  late final ImBackendAuditModule audit;
  late final ImBackendAutomationModule automation;
  late final ImBackendControlModule control;
  late final ImBackendAdminModule admin;

  ImBackendSdkClient(ImBackendSdkClientOptions options)
    : transportClient = options.transportClient,
      _context = ImBackendSdkContext(
        transportClient: options.transportClient,
        apiBaseUrl: options.apiBaseUrl,
        authToken: options.authToken,
        accessToken: options.accessToken,
      ) {
    ops = ImBackendOpsModule(_context);
    audit = ImBackendAuditModule(_context);
    automation = ImBackendAutomationModule(_context);
    control = ImBackendControlModule(_context);
    admin = ImBackendAdminModule(_context);
  }

  OpsApi get opsApi => transportClient.ops;
  AuditApi get auditApi => transportClient.audit;
  AutomationApi get automationApi => transportClient.automation;
  ControlApi get controlApi => transportClient.control;
  AdminApi get adminApi => transportClient.admin;

  factory ImBackendSdkClient.create({
    SdkworkBackendClient? transportClient,
    String? baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    if (transportClient == null && (baseUrl == null || baseUrl.trim().isEmpty)) {
      throw ArgumentError(
        'Provide transportClient or baseUrl when creating ImBackendSdkClient.',
      );
    }

    final resolvedTransportClient =
        transportClient ??
        SdkworkBackendClient.withBaseUrl(
          baseUrl: baseUrl!.trim(),
          authToken: authToken,
          accessToken: accessToken,
          headers: headers,
          timeout: timeout,
        );

    return ImBackendSdkClient(
      ImBackendSdkClientOptions(
        transportClient: resolvedTransportClient,
        apiBaseUrl: baseUrl,
        authToken: authToken,
        accessToken: accessToken,
      ),
    );
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }

  void clearAuthToken() {
    _context.clearAuthToken();
  }

  void setAccessToken(String token) {
    _context.setAccessToken(token);
  }

  void clearAccessToken() {
    _context.clearAccessToken();
  }
}
