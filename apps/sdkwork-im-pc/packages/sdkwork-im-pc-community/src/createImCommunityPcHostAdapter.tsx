import type { ComponentType } from 'react';
import { Avatar } from '@sdkwork/im-pc-commons';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';
import {
  getCommunityAppSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/communityAppSdkClient';
import { getFeedsOpenSdkClient } from '@sdkwork/im-pc-core/sdk/feedsOpenSdkClient';
import { readAppSdkSessionTokens } from '@sdkwork/im-pc-core/sdk/session';
import { createGeneratedCommunityAppSdkPort } from '@sdkwork/community-runtime';
import type {
  CommunityPcAvatarProps,
  CommunityPcHostAdapter,
  CommunityPcToast,
} from '@sdkwork/community-pc-community';
import type { SdkworkCommunityAppSdkPort } from '@sdkwork/community-sdk-ports';

const hostLanguageBridge = createImPcHostLanguageBridge();

function ImCommunityAvatar({
  alt,
  className,
  fallback,
  shape,
  size,
  src,
}: CommunityPcAvatarProps) {
  const resolvedSize =
    size === 'sm' || size === 'md' || size === 'lg' ? size : 'md';
  const resolvedShape =
    shape === 'circle' || shape === 'square' ? shape : 'square';

  return (
    <Avatar
      alt={alt}
      className={className}
      fallback={typeof fallback === 'string' ? fallback : undefined}
      shape={resolvedShape}
      size={resolvedSize}
      src={src}
    />
  );
}

export interface CreateImCommunityPcHostAdapterOptions {
  toast: CommunityPcToast;
}

function createCommunityAppSdkPort(): SdkworkCommunityAppSdkPort {
  return createGeneratedCommunityAppSdkPort(getCommunityAppSdkClientWithSession());
}

export function createImCommunityPcHostAdapter({
  toast,
}: CreateImCommunityPcHostAdapterOptions): CommunityPcHostAdapter {
  return {
    Avatar: ImCommunityAvatar as ComponentType<CommunityPcAvatarProps>,
    toast,
    readSessionTokens() {
      const session = readAppSdkSessionTokens();
      if (!session?.user) {
        return null;
      }
      return {
        user: {
          id: String(session.user.userId ?? session.user.id ?? ''),
          name: session.user.name,
          nickname: session.user.nickname,
          displayName: session.user.displayName,
          avatar: session.user.avatar,
        },
      };
    },
    languageBridge: hostLanguageBridge,
    createAppSdkPort: createCommunityAppSdkPort,
    // Circle post/resource feeds read through the standard feeds stream
    // system (community-{circleId} streams, open surface) instead of the
    // deprecated community feed.list surface.
    createFeedsSdkClient: getFeedsOpenSdkClient,
  };
}
