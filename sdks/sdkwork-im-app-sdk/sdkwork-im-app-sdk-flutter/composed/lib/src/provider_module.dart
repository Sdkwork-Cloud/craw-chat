import 'context.dart';
import 'types.dart';

class ImAppProviderModule {
  final ImAppSdkContext context;

  ImAppProviderModule(this.context);

  Future<ImAppJsonObject?> mediaHealth() async {
    return imAppAsJsonObject(
      await context.transportClient.provider.mediaHealthRetrieve(),
    );
  }

  Future<ImAppJsonObject?> principalProfileHealth() async {
    return imAppAsJsonObject(
      await context.transportClient.provider.principalProfileHealthRetrieve(),
    );
  }
}
