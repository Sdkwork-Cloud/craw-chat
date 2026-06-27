import 'context.dart';
import 'types.dart';

class ImBackendAuditModule {
  final ImBackendSdkContext context;

  ImBackendAuditModule(this.context);

  Future<ImBackendJsonObject?> listRecords() async {
    return imBackendAsJsonObject(
      await context.transportClient.audit.recordsList(),
    );
  }

  Future<ImBackendJsonObject?> recordAnchor() async {
    return imBackendAsJsonObject(
      await context.transportClient.audit.recordsCreate(),
    );
  }

  Future<ImBackendJsonObject?> exportBundle() async {
    return imBackendAsJsonObject(
      await context.transportClient.audit.exportRetrieve(),
    );
  }
}
