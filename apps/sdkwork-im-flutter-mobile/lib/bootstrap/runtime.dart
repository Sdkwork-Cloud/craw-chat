import 'package:app_links/app_links.dart';

import 'app_auth.dart';
import 'host_adapters.dart';
import 'iam_runtime.dart';
import 'routes.dart';
import 'sdk_clients.dart';

Future<void> bootstrap() async {
  await initAppAuthStorage();
  final initialUri = await AppLinks().getInitialLink();
  await consumeAppbaseCallbackSession(initialUri);
  createIamRuntime();
  registerHostAdapters();
  createSdkClients(session: loadAppSession());
  createRoutes();
}
