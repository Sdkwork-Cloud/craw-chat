import 'package:im_app_api_generated/im_app_api_generated.dart';

import 'context.dart';
import 'types.dart';

class ImAppPortalModule {
  final ImAppSdkContext context;

  ImAppPortalModule(this.context);

  Future<ImAppJsonObject?> access() async {
    return imAppAsJsonObject(await context.transportClient.portal.accessRetrieve());
  }

  Future<ImAppJsonObject?> automation() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.automationRetrieve(),
    );
  }

  Future<ImAppJsonObject?> conversations() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.conversationSnapshotRetrieve(),
    );
  }

  Future<ImAppJsonObject?> dashboard() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.dashboardRetrieve(),
    );
  }

  Future<ImAppJsonObject?> governance() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.governanceRetrieve(),
    );
  }

  Future<ImAppJsonObject?> home() async {
    return imAppAsJsonObject(await context.transportClient.portal.homeRetrieve());
  }

  Future<ImAppJsonObject?> media() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.mediaRetrieve(),
    );
  }

  Future<ImAppJsonObject?> realtime() async {
    return imAppAsJsonObject(
      await context.transportClient.portal.realtimeRetrieve(),
    );
  }

  Future<PortalWorkspaceView?> workspace() {
    return context.transportClient.portal.workspaceRetrieve();
  }
}
