import { createImAppAuthRuntime } from './imAppAuthRuntime';

export type { SdkworkAppbasePcAuthRuntimeComposition } from '@sdkwork/auth-runtime-pc-react';

let iamRuntimeComposition: ReturnType<typeof createImAppAuthRuntime> | null = null;

export function getIamRuntime(): ReturnType<typeof createImAppAuthRuntime> {
  if (!iamRuntimeComposition) {
    iamRuntimeComposition = createImAppAuthRuntime();
  }
  return iamRuntimeComposition;
}

export function resetIamRuntimeComposition(): void {
  iamRuntimeComposition = null;
}
