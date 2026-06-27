library im_app_sdk;

export 'package:im_app_api_generated/im_app_api_generated.dart';

export 'src/automation_module.dart';
export 'src/context.dart';
export 'src/notifications_module.dart';
export 'src/portal_module.dart';
export 'src/provider_module.dart';
export 'src/rtc_module.dart';
export 'src/types.dart';

import 'package:im_app_api_generated/im_app_api_generated.dart';
import 'package:rtc_sdk/rtc_sdk.dart';

import 'src/automation_module.dart';
import 'src/context.dart';
import 'src/notifications_module.dart';
import 'src/portal_module.dart';
import 'src/provider_module.dart';
import 'src/rtc_module.dart';
import 'src/types.dart';

class ImAppSdkClient {
  final ImAppSdkContext _context;
  final SdkworkAppClient transportClient;

  late final ImAppPortalModule portal;
  late final ImAppNotificationsModule notifications;
  late final ImAppAutomationModule automation;
  late final ImAppProviderModule provider;
  late final ImAppRtcModule rtc;

  ImAppSdkClient(ImAppSdkClientOptions options)
    : transportClient = options.transportClient,
      _context = ImAppSdkContext(
        transportClient: options.transportClient,
        rtcDataSource: options.rtcDataSource,
        apiBaseUrl: options.apiBaseUrl,
        authToken: options.authToken,
        accessToken: options.accessToken,
      ) {
    portal = ImAppPortalModule(_context);
    notifications = ImAppNotificationsModule(_context);
    automation = ImAppAutomationModule(_context);
    provider = ImAppProviderModule(_context);
    rtc = ImAppRtcModule(_context);
  }

  PortalApi get portalApi => transportClient.portal;
  NotificationsApi get notificationsApi => transportClient.notifications;
  AutomationApi get automationApi => transportClient.automation;
  ProviderApi get providerApi => transportClient.provider;
  RtcDataSource get rtcDataSource => _context.rtcDataSource;
  RtcDriverManager get rtcDriverManager => _context.rtcDataSource.driverManager;

  factory ImAppSdkClient.create({
    SdkworkAppClient? transportClient,
    RtcDataSource? rtcDataSource,
    String? baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    if (transportClient == null && (baseUrl == null || baseUrl.trim().isEmpty)) {
      throw ArgumentError(
        'Provide transportClient or baseUrl when creating ImAppSdkClient.',
      );
    }

    final resolvedTransportClient =
        transportClient ??
        SdkworkAppClient.withBaseUrl(
          baseUrl: baseUrl!.trim(),
          authToken: authToken,
          accessToken: accessToken,
          headers: headers,
          timeout: timeout,
        );

    return ImAppSdkClient(
      ImAppSdkClientOptions(
        transportClient: resolvedTransportClient,
        rtcDataSource: rtcDataSource,
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
