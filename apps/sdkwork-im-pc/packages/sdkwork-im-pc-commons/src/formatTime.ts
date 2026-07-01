/**
 * Shared time formatting utilities for sdkwork-im-pc.
 *
 * Prefer these over inline formatting to reduce duplication across
 * chat list, message list, favorites, call overlay, workspace, and
 * attendance views.
 */

/**
 * Format a Unix-millisecond timestamp as `HH:mm` (24h).
 */
export function formatMessageTime(timestamp: number): string {
  const date = new Date(timestamp);
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${hours}:${minutes}`;
}

/**
 * Format a Unix-millisecond timestamp as `M/D HH:mm`.
 */
export function formatShortDateTime(timestamp: number): string {
  const date = new Date(timestamp);
  return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
}

/**
 * Format seconds as `mm:ss` (call / music duration).
 */
export function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}
