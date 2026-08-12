import type { ComponentType } from 'react';
import { Avatar } from '@sdkwork/im-pc-commons';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';
import {
  getCompanyAppSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/companyAppSdkClient';
import { readAppSdkSessionTokens } from '@sdkwork/im-pc-core/sdk/session';
import { createGeneratedCompanyAppSdkPort } from '@sdkwork/company-runtime';
import type { CompanyPcHostAdapter } from '@sdkwork/company-pc-company';
import type { SdkworkCompanyAppSdkPort } from '@sdkwork/company-sdk-ports';

const hostLanguageBridge = createImPcHostLanguageBridge();

function ImCompanyAvatar({
  alt,
  className,
  fallback,
  src,
}: {
  alt?: string;
  className?: string;
  fallback?: string;
  src?: string;
}) {
  return (
    <Avatar
      alt={alt}
      className={className}
      fallback={fallback}
      shape="circle"
      size="md"
      src={src}
    />
  );
}

export interface CreateImCompanyPcHostAdapterOptions {
  toast: (message: string) => void;
}

function createCompanyAppSdkPort(): SdkworkCompanyAppSdkPort {
  return createGeneratedCompanyAppSdkPort(getCompanyAppSdkClientWithSession());
}

export function createImCompanyPcHostAdapter({
  toast,
}: CreateImCompanyPcHostAdapterOptions): CompanyPcHostAdapter {
  return {
    Avatar: ImCompanyAvatar as ComponentType<NonNullable<CompanyPcHostAdapter['Avatar']>>,
    toast,
    readSessionTokens() {
      const session = readAppSdkSessionTokens();
      if (!session?.user) {
        return { user: null };
      }
      return { user: { id: session.user.id, name: session.user.name } };
    },
    languageBridge: {
      getLanguage: () => hostLanguageBridge.getLanguage(),
      subscribe: hostLanguageBridge.subscribe,
    },
    createAppSdkPort: createCompanyAppSdkPort,
  };
}
