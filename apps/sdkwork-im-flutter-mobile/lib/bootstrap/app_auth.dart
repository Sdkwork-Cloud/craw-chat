import 'dart:convert';

import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

export 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart'
    show ImAppSession, defaultAppSession, imFlutterMobileSessionStorageKey;

const FlutterSecureStorage _secureStorage = FlutterSecureStorage(
  aOptions: AndroidOptions(encryptedSharedPreferences: true),
);

ImAppSession? _activeAppSession;

Future<void> initAppAuthStorage() async {
  final raw = await _secureStorage.read(key: imFlutterMobileSessionStorageKey);
  if (raw == null || raw.isEmpty) {
    return;
  }
  _activeAppSession = _parseStoredSession(raw);
}

ImAppSession? _parseStoredSession(String raw) {
  try {
    final decoded = jsonDecode(raw);
    if (decoded is Map<String, dynamic>) {
      final session = ImAppSession.fromJson(decoded);
      if (session.accessToken.isNotEmpty) {
        return session;
      }
    }
  } catch (_) {
    return null;
  }
  return null;
}

ImAppSession? loadAppSession() => _activeAppSession;

Future<void> saveAppSession(ImAppSession session) async {
  _activeAppSession = session;
  await _secureStorage.write(
    key: imFlutterMobileSessionStorageKey,
    value: jsonEncode(session.toJson()),
  );
}

Future<void> clearAppSession() async {
  _activeAppSession = null;
  await _secureStorage.delete(key: imFlutterMobileSessionStorageKey);
}

Future<ImAppSession?> consumeAppbaseCallbackSession(Uri? uri) async {
  final session = parseAppbaseCallbackSession(uri);
  if (session == null) {
    return null;
  }
  await saveAppSession(session);
  return session;
}

ImAppSession? bootstrapAppAuth() => loadAppSession();
