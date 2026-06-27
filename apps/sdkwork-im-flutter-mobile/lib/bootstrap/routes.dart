enum AppRoute {
  chatInbox,
}

extension AppRouteMetadata on AppRoute {
  String get label {
    switch (this) {
      case AppRoute.chatInbox:
        return 'Inbox';
    }
  }

  String get path {
    switch (this) {
      case AppRoute.chatInbox:
        return '#/chat/inbox';
    }
  }
}

List<String> createRoutes() {
  return AppRoute.values.map((route) => route.path).toList(growable: false);
}
