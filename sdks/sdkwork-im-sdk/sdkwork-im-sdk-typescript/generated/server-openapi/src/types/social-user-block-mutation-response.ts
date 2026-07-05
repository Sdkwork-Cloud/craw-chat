import type { SocialCommitEnvelope } from './social-commit-envelope';
import type { SocialWritePersistence } from './social-write-persistence';
import type { UserBlock } from './user-block';

export interface SocialUserBlockMutationResponse {
  userBlock: UserBlock;
  latestCommit: SocialCommitEnvelope;
  persistence: SocialWritePersistence;
}
