import { createImAppAuthRuntime } from './imAppAuthRuntime';

let iamRuntime: ReturnType<typeof createImAppAuthRuntime> | null = null;

export function createIamRuntime(): ReturnType<typeof createImAppAuthRuntime> {
  if (!iamRuntime) {
    iamRuntime = createImAppAuthRuntime();
  }
  return iamRuntime;
}

export function getIamRuntime(): ReturnType<typeof createImAppAuthRuntime> {
  return createIamRuntime();
}

export function resetIamRuntime(): void {
  iamRuntime = null;
}
