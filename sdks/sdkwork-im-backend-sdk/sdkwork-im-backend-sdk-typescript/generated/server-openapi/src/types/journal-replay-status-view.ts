export interface JournalReplayStatusView {
  status: 'enabled' | 'not_configured' | 'unavailable';
  mode: 'postgres-journal' | 'unconfigured';
  databaseConfigured: boolean;
  journalReady: boolean;
  totalCommits?: string | null;
  headCommitOffset?: string | null;
  latestOccurredAt?: string | null;
  detail?: string | null;
  generatedAt: string;
}
