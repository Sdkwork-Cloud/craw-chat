import 'dart:convert';

import 'package:shared_preferences/shared_preferences.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

export 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart'
    show ImAppSession, defaultAppSession, imFlutterMobileSessionStorageKey;

SharedPreferences? _preferences;
ImAppSession? _activeAppSession;

Future<void> initAppAuthStorage() async {
  _preferences ??= await SharedPreferences.getInstance();
  final raw = _preferences!.getString(imFlutterMobileSessionStorageKey);
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
  final prefs = _preferences ?? await SharedPreferences.getInstance();
  await prefs.setString(imFlutterMobileSessionStorageKey, jsonEncode(session.toJson()));
}

Future<void> clearAppSession() async {
  _activeAppSession = null;
  final prefs = _preferences ?? await SharedPreferences.getInstance();
  await prefs.remove(imFlutterMobileSessionStorageKey);
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
