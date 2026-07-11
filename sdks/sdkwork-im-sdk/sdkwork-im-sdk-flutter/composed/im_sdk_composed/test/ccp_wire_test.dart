import 'dart:convert';

import 'package:im_sdk_composed/im_sdk_composed.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('encodeCcpHelloFrame produces hello_ack compatible envelope', () {
    final frame = encodeCcpHelloFrame();
    expect(isCcpHelloAckEnvelope(frame), isFalse);
    final envelope = decodeCcpEnvelope(frame);
    expect(envelope?['schema'], 'cc.control.hello.v1');
  });

  test('unwrapInboundRealtimeFrame unwraps CCP payload', () {
    final business = encodeCcpBusinessFrame(
      'cc.realtime.events.push.v1',
      'event',
      <String, dynamic>{
        'type': 'event.window',
        'window': <String, dynamic>{
          'items': <Map<String, dynamic>>[
            <String, dynamic>{
              'eventType': 'message.posted',
              'scopeId': 'conv-1',
              'payload': <String, dynamic>{
                'conversationId': 'conv-1',
                'messageId': 'msg-1',
                'body': <String, dynamic>{'text': 'hello'},
              },
            },
          ],
        },
      },
    );
    final inbound = unwrapInboundRealtimeFrame(business);
    expect(inbound.contains('event.window'), isTrue);
    expect(inbound.contains('conv-1'), isTrue);
  });

  test('unwrapInboundRealtimeFrame accepts server heartbeat route metadata', () {
    final serverHeartbeat = jsonEncode(<String, dynamic>{
      'protocol': <String, dynamic>{'family': 'ccp', 'major': 1, 'minor': 0},
      'binding': 'Ws1',
      'kind': 'control',
      'schema': 'cc.control.heartbeat.v1',
      'scope': null,
      'route': <String, dynamic>{
        'tenant_id': '100001',
        'principal_id': '331115548962201600',
        'device_id': 'c_mqy28dxy_hrv4oul7',
      },
      'flags': <String>[],
      'trace_id': 'c9dafbd7-f968-4b5a-8256-6c63577aa1f7',
      'payload': jsonEncode(<String, dynamic>{
        'type': 'heartbeat',
        'data': <String, dynamic>{'sequence': 2},
      }),
    });

    final inbound = unwrapInboundRealtimeFrame(serverHeartbeat);
    final payload = jsonDecode(inbound) as Map<String, dynamic>;

    expect(payload['type'], 'heartbeat');
    expect(payload['data'], <String, dynamic>{'sequence': 2});
  });
}
