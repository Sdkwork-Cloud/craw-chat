import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

Map<String, dynamic>? readSdkMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

PageInfo emptyCursorPageInfo() {
  return PageInfo(mode: 'cursor', hasMore: false);
}

PageInfo readPageInfoFromSdkData(dynamic data) {
  final dataMap = readSdkMap(data);
  final pageInfoMap = readSdkMap(dataMap?['pageInfo']);
  if (pageInfoMap == null) {
    return emptyCursorPageInfo();
  }
  return PageInfo.fromJson(pageInfoMap);
}

List<T> readItemsFromSdkData<T>(
  dynamic data,
  T Function(Map<String, dynamic> json) decode,
) {
  final dataMap = readSdkMap(data);
  final rawItems = dataMap?['items'];
  if (rawItems is! Iterable) {
    return <T>[];
  }
  return rawItems
      .map(readSdkMap)
      .whereType<Map<String, dynamic>>()
      .map(decode)
      .toList(growable: false);
}

T? readItemFromSdkData<T>(
  dynamic data,
  T Function(Map<String, dynamic> json) decode,
) {
  final itemMap = readSdkMap(readSdkMap(data)?['item']);
  return itemMap == null ? null : decode(itemMap);
}
