import { createNotaryAccessService } from '@sdkwork/notary-pc-core';
import { getNotaryAppSdkClient } from '@sdkwork/im-pc-core';

export type { NotaryAccessService, NotaryAccessState } from '@sdkwork/notary-pc-core';

export function createImNotaryAccessService() {
  return createNotaryAccessService(getNotaryAppSdkClient);
}

export const notaryAccessService = createImNotaryAccessService();
