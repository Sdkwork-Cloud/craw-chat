import 'package:im_backend_api_generated/im_backend_api_generated.dart';

typedef ImBackendJsonObject = Map<String, dynamic>;
typedef ImBackendJsonArray = List<dynamic>;

class ImBackendSdkClientOptions {
  final SdkworkBackendClient transportClient;
  final String? apiBaseUrl;
  final String? authToken;
  final String? accessToken;

  const ImBackendSdkClientOptions({
    required this.transportClient,
    this.apiBaseUrl,
    this.authToken,
    this.accessToken,
  });
}

ImBackendJsonObject? imBackendAsJsonObject(dynamic value) {
  if (value is ImBackendJsonObject) {
    return value;
  }
  if (value is Map) {
    return value.map(
      (key, item) => MapEntry(key.toString(), item),
    );
  }
  return null;
}

List<ImBackendJsonObject>? imBackendAsJsonObjectList(dynamic value) {
  if (value is! List) {
    return null;
  }
  final result = <ImBackendJsonObject>[];
  for (final item in value) {
    final map = imBackendAsJsonObject(item);
    if (map != null) {
      result.add(map);
    }
  }
  return result;
}
