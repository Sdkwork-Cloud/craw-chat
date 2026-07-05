import type { SocialDerivedSnapshotStatus } from './social-derived-snapshot-status';

export interface SocialWritePersistence {
  journalAuthority: boolean;
  snapshotStatus: SocialDerivedSnapshotStatus;
}
