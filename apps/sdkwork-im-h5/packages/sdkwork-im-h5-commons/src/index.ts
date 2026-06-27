export function formatRelativeTime(timestamp: number | string | undefined): string {
  if (!timestamp) {
    return "";
  }
  const value = typeof timestamp === "string" ? Date.parse(timestamp) : timestamp;
  if (!Number.isFinite(value)) {
    return "";
  }
  const deltaMs = Date.now() - value;
  if (deltaMs < 60_000) {
    return "now";
  }
  if (deltaMs < 3_600_000) {
    return `${Math.floor(deltaMs / 60_000)}m`;
  }
  if (deltaMs < 86_400_000) {
    return `${Math.floor(deltaMs / 3_600_000)}h`;
  }
  return new Date(value).toLocaleDateString();
}
