import 'dart:convert';
import 'dart:io';

import 'package:shared_preferences/shared_preferences.dart';

const _storageKey = 'sdkwork-im-flutter-mobile:pending-sends:v2';
const _legacyStorageKey = 'sdkwork-im-flutter-mobile:pending-sends:v1';
const _defaultFlushLimit = 50;
const maxPendingSends = 100;

class PendingTextSendPayload {
  const PendingTextSendPayload({
    required this.conversationId,
    required this.text,
    required this.clientMsgId,
  });

  final String conversationId;
  final String text;
  final String clientMsgId;

  Map<String, dynamic> toJson() => {
        'conversationId': conversationId,
        'text': text,
        'clientMsgId': clientMsgId,
      };

  factory PendingTextSendPayload.fromJson(Map<String, dynamic> json) {
    return PendingTextSendPayload(
      conversationId: json['conversationId']?.toString() ?? '',
      text: json['text']?.toString() ?? '',
      clientMsgId: json['clientMsgId']?.toString() ?? '',
    );
  }
}

class PendingTextSendPayloadWithClaim extends PendingTextSendPayload {
  const PendingTextSendPayloadWithClaim({
    required super.conversationId,
    required super.text,
    required super.clientMsgId,
    required this.claimId,
  });

  final String claimId;
}

class _PendingSendRecord {
  const _PendingSendRecord({
    required this.tenantId,
    required this.clientMsgId,
    required this.conversationId,
    required this.payloadJson,
    required this.createdAt,
    this.attemptCount = 0,
    this.flushClaimId,
  });

  final String tenantId;
  final String clientMsgId;
  final String conversationId;
  final String payloadJson;
  final String createdAt;
  final int attemptCount;
  final String? flushClaimId;

  Map<String, dynamic> toJson() => {
        'tenantId': tenantId,
        'clientMsgId': clientMsgId,
        'conversationId': conversationId,
        'payloadJson': payloadJson,
        'createdAt': createdAt,
        'attemptCount': attemptCount,
        if (flushClaimId != null) 'flushClaimId': flushClaimId,
      };

  factory _PendingSendRecord.fromJson(Map<String, dynamic> json) {
    return _PendingSendRecord(
      tenantId: json['tenantId']?.toString() ?? '',
      clientMsgId: json['clientMsgId']?.toString() ?? '',
      conversationId: json['conversationId']?.toString() ?? '',
      payloadJson: json['payloadJson']?.toString() ?? '',
      createdAt: json['createdAt']?.toString() ?? '',
      attemptCount: int.tryParse(json['attemptCount']?.toString() ?? '') ?? 0,
      flushClaimId: json['flushClaimId']?.toString(),
    );
  }

  _PendingSendRecord copyWith({
    int? attemptCount,
    String? flushClaimId,
    bool clearFlushClaimId = false,
  }) {
    return _PendingSendRecord(
      tenantId: tenantId,
      clientMsgId: clientMsgId,
      conversationId: conversationId,
      payloadJson: payloadJson,
      createdAt: createdAt,
      attemptCount: attemptCount ?? this.attemptCount,
      flushClaimId: clearFlushClaimId ? null : (flushClaimId ?? this.flushClaimId),
    );
  }
}

Future<void>? _pendingSendFlushInFlight;

String _createPendingSendClaimId() {
  return 'flutter-flush-${DateTime.now().millisecondsSinceEpoch}-${DateTime.now().microsecond}';
}

bool isRetryableFlutterSendError(Object error) {
  if (error is SocketException || error is HttpException) {
    return true;
  }
  final message = error.toString().toLowerCase();
  return message.contains('failed host lookup')
      || message.contains('connection refused')
      || message.contains('connection reset')
      || message.contains('network')
      || message.contains('timeout')
      || message.contains('service unavailable')
      || message.contains('503')
      || message.contains('502')
      || message.contains('504');
}

Future<List<_PendingSendRecord>> _readQueue(SharedPreferences prefs) async {
  final raw = prefs.getString(_storageKey);
  if (raw == null || raw.isEmpty) {
    return const [];
  }
  try {
    final decoded = jsonDecode(raw);
    if (decoded is! List) {
      return const [];
    }
    return decoded
        .whereType<Map>()
        .map((entry) => _PendingSendRecord.fromJson(Map<String, dynamic>.from(entry)))
        .toList();
  } catch (_) {
    return const [];
  }
}

Future<void> _writeQueue(SharedPreferences prefs, List<_PendingSendRecord> records) async {
  await prefs.setString(
    _storageKey,
    jsonEncode(records.map((record) => record.toJson()).toList()),
  );
}

Future<void> _migrateLegacyQueue(SharedPreferences prefs) async {
  final legacyRaw = prefs.getString(_legacyStorageKey);
  if (legacyRaw == null || legacyRaw.isEmpty) {
    return;
  }
  try {
    final decoded = jsonDecode(legacyRaw);
    if (decoded is! List) {
      return;
    }
    final migrated = <_PendingSendRecord>[];
    for (final entry in decoded.whereType<Map>()) {
      final record = _PendingSendRecord.fromJson(Map<String, dynamic>.from(entry));
      if (record.tenantId.isEmpty || record.clientMsgId.isEmpty) {
        continue;
      }
      migrated.add(
        record.copyWith(clearFlushClaimId: true),
      );
    }
    if (migrated.isNotEmpty) {
      await _writeQueue(prefs, migrated);
    }
  } catch (_) {
    // Drop corrupt legacy queue.
  } finally {
    await prefs.remove(_legacyStorageKey);
  }
}

Future<SharedPreferences> _openPrefs() async {
  final prefs = await SharedPreferences.getInstance();
  await _migrateLegacyQueue(prefs);
  return prefs;
}

PendingTextSendPayload? _parsePayload(_PendingSendRecord record) {
  try {
    final decoded = jsonDecode(record.payloadJson);
    if (decoded is! Map) {
      return null;
    }
    final payload = PendingTextSendPayload.fromJson(
      Map<String, dynamic>.from(decoded),
    );
    if (payload.conversationId.isEmpty
        || payload.text.isEmpty
        || payload.clientMsgId.isEmpty) {
      return null;
    }
    return payload;
  } catch (_) {
    return null;
  }
}

Future<void> enqueuePendingTextSend({
  required String tenantId,
  required PendingTextSendPayload payload,
}) async {
  if (tenantId.isEmpty) {
    return;
  }
  final prefs = await _openPrefs();
  final queue = (await _readQueue(prefs))
      .where((record) => record.clientMsgId != payload.clientMsgId)
      .toList();
  queue.add(
    _PendingSendRecord(
      tenantId: tenantId,
      clientMsgId: payload.clientMsgId,
      conversationId: payload.conversationId,
      payloadJson: jsonEncode(payload.toJson()),
      createdAt: DateTime.now().toUtc().toIso8601String(),
      attemptCount: 0,
    ),
  );
  final tenantQueue = queue.where((record) => record.tenantId == tenantId).toList();
  if (tenantQueue.length > maxPendingSends) {
    final sorted = [...tenantQueue]
      ..sort((left, right) => left.createdAt.compareTo(right.createdAt));
    final dropCount = tenantQueue.length - maxPendingSends;
    final dropIds = sorted
        .take(dropCount)
        .map((record) => record.clientMsgId)
        .toSet();
    queue.removeWhere(
      (record) => record.tenantId == tenantId && dropIds.contains(record.clientMsgId),
    );
  }
  await _writeQueue(prefs, queue);
}

Future<List<PendingTextSendPayload>> listPendingTextSends({
  required String tenantId,
  int limit = _defaultFlushLimit,
}) async {
  if (tenantId.isEmpty) {
    return const [];
  }
  final prefs = await _openPrefs();
  final payloads = <PendingTextSendPayload>[];
  final records = (await _readQueue(prefs))
      .where((record) => record.tenantId == tenantId && record.flushClaimId == null)
      .toList()
    ..sort((left, right) => left.createdAt.compareTo(right.createdAt));
  for (final record in records.take(limit)) {
    final payload = _parsePayload(record);
    if (payload != null) {
      payloads.add(payload);
    }
  }
  return payloads;
}

Future<List<PendingTextSendPayloadWithClaim>> claimPendingTextSends({
  required String tenantId,
  int limit = _defaultFlushLimit,
}) async {
  if (tenantId.isEmpty) {
    return const [];
  }
  final prefs = await _openPrefs();
  final claimId = _createPendingSendClaimId();
  final queue = await _readQueue(prefs);
  final candidates = queue
      .where((record) => record.tenantId == tenantId && record.flushClaimId == null)
      .toList()
    ..sort((left, right) => left.createdAt.compareTo(right.createdAt));
  final selected = candidates.take(limit).map((record) => record.clientMsgId).toSet();
  if (selected.isEmpty) {
    return const [];
  }
  final updated = queue
      .map((record) {
        if (record.tenantId == tenantId && selected.contains(record.clientMsgId)) {
          return record.copyWith(
            flushClaimId: claimId,
            attemptCount: record.attemptCount + 1,
          );
        }
        return record;
      })
      .toList();
  await _writeQueue(prefs, updated);
  final payloads = <PendingTextSendPayloadWithClaim>[];
  for (final record in updated.where(
    (record) => record.tenantId == tenantId && record.flushClaimId == claimId,
  )) {
    final payload = _parsePayload(record);
    if (payload != null) {
      payloads.add(
        PendingTextSendPayloadWithClaim(
          conversationId: payload.conversationId,
          text: payload.text,
          clientMsgId: payload.clientMsgId,
          claimId: claimId,
        ),
      );
    }
  }
  payloads.sort((left, right) => left.clientMsgId.compareTo(right.clientMsgId));
  return payloads;
}

Future<void> releasePendingTextSendClaim({
  required String tenantId,
  required String clientMsgId,
  required String claimId,
}) async {
  if (tenantId.isEmpty || claimId.isEmpty) {
    return;
  }
  final prefs = await _openPrefs();
  final queue = await _readQueue(prefs);
  final updated = queue
      .map((record) {
        if (record.tenantId == tenantId
            && record.clientMsgId == clientMsgId
            && record.flushClaimId == claimId) {
          return record.copyWith(clearFlushClaimId: true);
        }
        return record;
      })
      .toList();
  await _writeQueue(prefs, updated);
}

Future<void> removePendingTextSend({
  required String tenantId,
  required String clientMsgId,
}) async {
  if (tenantId.isEmpty) {
    return;
  }
  final prefs = await _openPrefs();
  final queue = (await _readQueue(prefs))
      .where(
        (record) => !(record.tenantId == tenantId && record.clientMsgId == clientMsgId),
      )
      .toList();
  await _writeQueue(prefs, queue);
}

Future<void> runPendingTextSendFlushForTenant({
  required String tenantId,
  required Future<void> Function(List<PendingTextSendPayloadWithClaim> pending) flush,
  int limit = _defaultFlushLimit,
}) async {
  if (_pendingSendFlushInFlight != null) {
    await _pendingSendFlushInFlight;
    return;
  }
  _pendingSendFlushInFlight = () async {
    final pending = await claimPendingTextSends(tenantId: tenantId, limit: limit);
    if (pending.isEmpty) {
      return;
    }
    await flush(pending);
  }();
  try {
    await _pendingSendFlushInFlight;
  } finally {
    _pendingSendFlushInFlight = null;
  }
}
