import { createImAppAuthRuntime } from './imAppAuthRuntime';
import { resolveImAuthRuntimeConfig } from './imAuthConfig';

export interface IamRuntimeBootstrapOptions {
  appId?: string;
  deploymentMode?: 'local' | 'private' | 'saas';
  environment?: 'dev' | 'prod' | 'test';
}

export interface IamRuntimeComposition {
  authRuntime: ReturnType<typeof createImAppAuthRuntime>;
  authConfig: ReturnType<typeof resolveImAuthRuntimeConfig>;
}

let iamRuntimeComposition: IamRuntimeComposition | null = null;

export function createIamRuntime(
  options: IamRuntimeBootstrapOptions = {},
): IamRuntimeComposition {
  if (iamRuntimeComposition) {
    return iamRuntimeComposition;
  }

  const authRuntime = createImAppAuthRuntime(options);
  const authConfig = resolveImAuthRuntimeConfig();

  iamRuntimeComposition = {
    authRuntime,
    authConfig,
  };

  return iamRuntimeComposition;
}

export function resetIamRuntime(): void {
  iamRuntimeComposition = null;
}

export function getIamRuntime(): IamRuntimeComposition {
  return iamRuntimeComposition ?? createIamRuntime();
}
