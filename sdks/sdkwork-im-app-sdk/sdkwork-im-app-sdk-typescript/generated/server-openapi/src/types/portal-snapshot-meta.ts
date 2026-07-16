export interface PortalSnapshotMeta {
  section: 'access' | 'automation' | 'conversations' | 'dashboard' | 'governance' | 'home' | 'media' | 'realtime';
  generatedAt: string;
  opsStatus: 'ok' | 'idle' | 'degraded' | 'unavailable' | 'critical' | 'unknown';
}
