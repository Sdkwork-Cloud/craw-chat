import 'dart:math';

/// Generates a collision-resistant client message id.
///
/// Idempotent message writes bind the client message id as the deduplication
/// key, so ids must never collide across messages, conversations, or send
/// attempts. Wall-clock + hashcode schemes can collide for two messages in the
/// same millisecond; a cryptographically strong 128-bit random value avoids
/// that entirely.
String newClientMessageId() {
  final random = Random.secure();
  final bytes = List<int>.generate(16, (_) => random.nextInt(256));
  final hex = bytes
      .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
      .join();
  return 'flutter-$hex';
}
