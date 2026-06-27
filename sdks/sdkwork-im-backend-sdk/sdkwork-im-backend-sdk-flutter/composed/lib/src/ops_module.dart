import 'context.dart';
import 'types.dart';

class ImBackendOpsModule {
  final ImBackendSdkContext context;

  ImBackendOpsModule(this.context);

  Future<ImBackendJsonObject?> health() async {
    return imBackendAsJsonObject(await context.transportClient.ops.healthRetrieve());
  }

  Future<ImBackendJsonObject?> cluster() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.clusterRetrieve(),
    );
  }

  Future<ImBackendJsonObject?> lag() async {
    return imBackendAsJsonObject(await context.transportClient.ops.lagRetrieve());
  }

  Future<ImBackendJsonObject?> replayStatus() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.replayStatusRetrieve(),
    );
  }

  Future<ImBackendJsonObject?> commercialReadiness() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.commercialReadinessRetrieve(),
    );
  }

  Future<ImBackendJsonObject?> runtimeDir() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.runtimeDirRetrieve(),
    );
  }

  Future<ImBackendJsonObject?> providerBindings() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.providerBindingsList(),
    );
  }

  Future<ImBackendJsonObject?> providerBindingsDrift() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.providerBindingsDriftRetrieve(),
    );
  }

  Future<ImBackendJsonObject?> diagnostics() async {
    return imBackendAsJsonObject(
      await context.transportClient.ops.diagnosticsRetrieve(),
    );
  }
}
