import type { CommitEnvelopeResponse } from './commit-envelope-response';
import type { SocialWritePersistence } from './social-write-persistence';
import type { UserBlock } from './user-block';

export interface OpenApiUserBlockResponse {
  userBlock: UserBlock;
  latestCommit: CommitEnvelopeResponse;
  persistence: SocialWritePersistence;
}
