export interface Sender {
  id: string;
  kind: string;
  principalId?: string | null;
  principalKind?: string | null;
  displayName?: string | null;
  avatarUrl?: string | null;
}
