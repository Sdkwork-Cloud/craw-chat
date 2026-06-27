class ImEnvironment {

  final String applicationPublicHttpUrl;

  final String applicationPublicWebSocketUrl;

  final String appbaseLoginUrl;



  const ImEnvironment({

    required this.applicationPublicHttpUrl,

    required this.applicationPublicWebSocketUrl,

    required this.appbaseLoginUrl,

  });

}



String _normalizeBaseUrl(String value, String fallback) {

  final normalized = value.trim();

  return normalized.isEmpty ? fallback : normalized;

}



ImEnvironment resolveEnvironment() {

  const applicationPublicHttpUrl = String.fromEnvironment(

    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',

    defaultValue: 'http://127.0.0.1:18079',

  );

  const applicationPublicWebSocketUrl = String.fromEnvironment(

    'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL',

    defaultValue: 'ws://127.0.0.1:18079',

  );



  return ImEnvironment(

    applicationPublicHttpUrl: _normalizeBaseUrl(

      const String.fromEnvironment('SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL'),

      applicationPublicHttpUrl,

    ),

    applicationPublicWebSocketUrl: _normalizeBaseUrl(

      const String.fromEnvironment('SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL'),

      applicationPublicWebSocketUrl,

    ),

    appbaseLoginUrl: _normalizeBaseUrl(

      const String.fromEnvironment('SDKWORK_IAM_APP_API_BASE_URL'),

      _normalizeBaseUrl(applicationPublicHttpUrl, 'http://127.0.0.1:18079'),

    ),

  );

}
