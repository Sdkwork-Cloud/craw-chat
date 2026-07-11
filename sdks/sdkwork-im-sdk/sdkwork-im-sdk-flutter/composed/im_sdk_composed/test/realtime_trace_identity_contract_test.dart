import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

void main() {
  test('realtime outbound wire frames do not expose legacy requestId fields', () {
    final realtimeSource = File('lib/src/im_realtime.dart').readAsStringSync();
    final ccpWireSource = File('lib/src/ccp_wire.dart').readAsStringSync();

    expect(
      realtimeSource,
      isNot(contains('requestId')),
      reason: 'Realtime client wire frames must not send legacy requestId fields.',
    );
    expect(
      realtimeSource,
      isNot(contains('sdkwork-im-subscriptions-sync')),
      reason: 'Subscription sync correlation must not use retired client-generated request ids.',
    );
    expect(
      ccpWireSource,
      isNot(contains('String requestId')),
      reason: 'CCP trace parameters must be named traceId, not requestId.',
    );
  });
}
