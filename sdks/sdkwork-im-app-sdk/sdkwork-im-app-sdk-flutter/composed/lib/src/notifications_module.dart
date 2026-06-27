import 'package:im_app_api_generated/im_app_api_generated.dart';

import 'context.dart';

class ImAppNotificationsModule {
  final ImAppSdkContext context;

  ImAppNotificationsModule(this.context);

  Future<NotificationListResponse?> list() {
    return context.transportClient.notifications.list();
  }

  Future<NotificationRequestResponse?> request(RequestNotification body) {
    return context.transportClient.notifications.requestsCreate(body);
  }

  Future<NotificationTask?> get(String notificationId) {
    return context.transportClient.notifications.retrieve(notificationId);
  }
}
