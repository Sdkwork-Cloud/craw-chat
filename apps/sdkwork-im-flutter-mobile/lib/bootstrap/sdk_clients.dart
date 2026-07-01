import 'package:sdkwork_im_flutter_mobile_chat/sdkwork_im_flutter_mobile_chat.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'app_auth.dart';
import 'environment.dart';

class ImSdkClients {
  const ImSdkClients({
    required this.applicationPublicHttpUrl,
    required this.im,
  });

  final String applicationPublicHttpUrl;
  final ImSdkClientBundle im;
}

ImSdkClients? _activeSdkClients;
ImSdkClientBundle? _sharedBundle;

ImSdkClients createSdkClients({ImAppSession? session}) {
  final env = resolveEnvironment();
  final activeSession = session ?? loadAppSession();
  final bundle = createImSdkClient(
    applicationPublicHttpUrl: env.applicationPublicHttpUrl,
    applicationPublicWebSocketUrl: env.applicationPublicWebSocketUrl,
    accessToken: activeSession?.accessToken,
    authToken: activeSession?.authToken ?? activeSession?.accessToken,
    existingClient: _sharedBundle?.imSdk,
    existingComposedClient: _sharedBundle?.composed,
  );
  _sharedBundle = bundle;

  _activeSdkClients = ImSdkClients(
    applicationPublicHttpUrl: env.applicationPublicHttpUrl,
    im: bundle,
  );
  return _activeSdkClients!;
}

ImSdkClients getSdkClients() {
  return _activeSdkClients ?? createSdkClients();
}

Future<void> resetSdkClients() async {
  final bundle = _sharedBundle;
  _activeSdkClients = null;
  _sharedBundle = null;
  if (bundle != null) {
    await disposeChatRealtimeHub(bundle);
  }
}
