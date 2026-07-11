import 'app_session.dart';

const _callbackKeys = <String, List<String>>{
  'accessToken': ['accessToken', 'access_token'],
  'authToken': ['authToken', 'auth_token', 'token'],
  'tenantId': ['tenantId', 'tenant_id'],
  'organizationId': ['organizationId', 'organization_id'],
  'userId': ['userId', 'user_id'],
};

final Uri _expectedCallbackUri = Uri.parse(appbaseCallbackReturnUrl);

String _readParam(Map<String, String> params, List<String> keys) {
  for (final key in keys) {
    final value = params[key]?.trim();
    if (value != null && value.isNotEmpty) {
      return value;
    }
  }
  return '';
}

Uri buildAppbaseLoginUrl({
  required String loginUrl,
  required String returnUrl,
}) {
  final target = Uri.parse(loginUrl);
  return target.replace(
    queryParameters: {
      ...target.queryParameters,
      'returnUrl': returnUrl,
    },
  );
}

ImAppSession? parseAppbaseCallbackSession(Uri? uri) {
  if (uri == null) {
    return null;
  }
  if (!_isExpectedCallbackUri(uri)) {
    return null;
  }

  final params = uri.queryParameters;
  final accessToken = _readParam(params, _callbackKeys['accessToken']!);
  final authToken = _readParam(params, _callbackKeys['authToken']!);
  final tenantId = _readParam(params, _callbackKeys['tenantId']!);
  final organizationId = _readParam(params, _callbackKeys['organizationId']!);
  final userId = _readParam(params, _callbackKeys['userId']!);
  if (accessToken.isEmpty || authToken.isEmpty) {
    return null;
  }
  if (tenantId.isEmpty || organizationId.isEmpty || userId.isEmpty) {
    return null;
  }

  return ImAppSession(
    accessToken: accessToken,
    authToken: authToken,
    tenantId: tenantId,
    organizationId: organizationId,
    userId: userId,
  );
}

bool _isExpectedCallbackUri(Uri uri) =>
    uri.scheme == _expectedCallbackUri.scheme &&
    uri.host == _expectedCallbackUri.host &&
    uri.path == _expectedCallbackUri.path;

String get appbaseCallbackReturnUrl => 'sdkworkim://auth/callback';
