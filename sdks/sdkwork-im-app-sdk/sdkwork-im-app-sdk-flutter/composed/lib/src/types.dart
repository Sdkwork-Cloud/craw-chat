import 'package:im_app_api_generated/im_app_api_generated.dart';
import 'package:rtc_sdk/rtc_sdk.dart';

typedef ImAppJsonObject = Map<String, dynamic>;
typedef ImAppJsonArray = List<dynamic>;

class ImAppSdkClientOptions {
  final SdkworkAppClient transportClient;
  final RtcDataSource? rtcDataSource;
  final String? apiBaseUrl;
  final String? authToken;
  final String? accessToken;

  const ImAppSdkClientOptions({
    required this.transportClient,
    this.rtcDataSource,
    this.apiBaseUrl,
    this.authToken,
    this.accessToken,
  });
}

ImAppJsonObject? imAppAsJsonObject(dynamic value) {
  if (value is ImAppJsonObject) {
    return value;
  }
  if (value is Map) {
    return value.map(
      (key, item) => MapEntry(key.toString(), item),
    );
  }
  return null;
}

List<ImAppJsonObject>? imAppAsJsonObjectList(dynamic value) {
  if (value is! List) {
    return null;
  }
  final result = <ImAppJsonObject>[];
  for (final item in value) {
    final map = imAppAsJsonObject(item);
    if (map != null) {
      result.add(map);
    }
  }
  return result;
}
