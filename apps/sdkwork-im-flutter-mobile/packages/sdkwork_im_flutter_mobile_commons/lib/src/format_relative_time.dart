String formatRelativeTime(String? timestamp) {
  if (timestamp == null || timestamp.trim().isEmpty) {
    return '';
  }
  final parsed = DateTime.tryParse(timestamp);
  if (parsed == null) {
    return '';
  }
  final delta = DateTime.now().difference(parsed);
  if (delta.inMinutes < 1) {
    return 'now';
  }
  if (delta.inHours < 1) {
    return '${delta.inMinutes}m';
  }
  if (delta.inDays < 1) {
    return '${delta.inHours}h';
  }
  return '${parsed.month}/${parsed.day}';
}
