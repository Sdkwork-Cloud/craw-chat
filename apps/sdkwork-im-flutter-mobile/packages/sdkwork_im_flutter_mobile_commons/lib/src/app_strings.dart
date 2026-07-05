import 'dart:ui';

class AppStrings {
  AppStrings._();

  static String get localeTag {
    final languageCode = PlatformDispatcher.instance.locale.languageCode;
    return languageCode.startsWith('zh') ? 'zh-CN' : 'en-US';
  }

  static String get appTitle => _pick('SDKWork IM', 'Sdkwork IM');

  static String get signInTitle => _pick('IM App Sign In', 'IM 登录');

  static String get signedInPrefix => _pick('Signed in as', '已登录');

  static String get signOut => _pick('Sign out', '退出');

  static String get sendFailed => _pick('Failed to send message', '发送消息失败');

  static String get inboxTitle => _pick('Inbox', '收件箱');

  static String get errorWidgetMessage =>
      _pick('Something went wrong. Please restart the app.', '出现错误，请重启应用。');

  static String _pick(String en, String zh) => localeTag == 'zh-CN' ? zh : en;
}
