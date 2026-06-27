import 'package:im_sdk_composed/im_sdk_composed.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('encodeCcpHelloFrame produces hello_ack compatible envelope', () {
    final frame = encodeCcpHelloFrame('hello-1');
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
}
