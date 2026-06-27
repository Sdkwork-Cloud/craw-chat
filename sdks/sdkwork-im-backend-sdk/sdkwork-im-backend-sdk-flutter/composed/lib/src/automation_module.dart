import 'context.dart';
import 'types.dart';

class ImBackendAutomationModule {
  final ImBackendSdkContext context;

  ImBackendAutomationModule(this.context);

  Future<ImBackendJsonObject?> governance() async {
    return imBackendAsJsonObject(
      await context.transportClient.automation.governanceRetrieve(),
    );
  }
}
