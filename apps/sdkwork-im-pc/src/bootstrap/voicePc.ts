import { bootstrapVoicePcForIm } from '@sdkwork/im-pc-core';

export function bootstrapImVoicePcIntegration(): Promise<void> {
  return bootstrapVoicePcForIm();
}
