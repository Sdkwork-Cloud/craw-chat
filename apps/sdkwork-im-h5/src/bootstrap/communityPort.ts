/**
 * IM H5 community (圈子) runtime port wiring.
 *
 * Binds the shared `@sdkwork/community-mobile-react-community` package to the
 * IM host:
 *
 * - importing `@sdkwork/im-h5-community` runs its side effect that binds the
 *   IM auth session port (current-user lookup for the payment sheet);
 * - `configureCommunityRuntimePort` switches the package to the generated
 *   Community App SDK port constructed from the IM gateway base URL and the
 *   shared H5 token manager;
 * - `configureCommunityOrderRuntime` routes circle membership order creation
 *   through the IM-composed order App SDK (`memberships.orders.create`) so
 *   the whole purchase flow settles on sdkwork-order. The official cashier
 *   (`configureOrderMobileRuntime`) is already composed by the host.
 */

import '@sdkwork/im-h5-community';
import {
  configureCommunityFeedsPort,
  configureCommunityMediaRuntimePort,
  configureCommunityOrderRuntime,
  configureCommunityRuntimePort,
  type CreateCircleMembershipOrderOptions,
  type CircleMembershipOrder,
} from '@sdkwork/community-mobile-react-community';
import { getSdkClients } from './sdkClients';
import { getDriveAppSdkClientWithSession } from '@sdkwork/im-h5-core/sdk';


function createIdempotencyKey(): string {
  const random = Math.random().toString(36).slice(2) + Date.now().toString(36);
  if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
    return crypto.randomUUID();
  }
  return `community-${random}`;
}

let bootstrapped = false;

export function bootstrapImCommunityH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;

  configureCommunityRuntimePort(getSdkClients().communityAppSdkPort);

  // Circle post/resource feeds read through the standard feeds stream system
  // (`community-{circleId}` / `community-{circleId}-resources` streams, open
  // surface) instead of the deprecated community feed.list surface.
  configureCommunityFeedsPort(getSdkClients().feedsOpenSdkClient);

  // Post images upload through the platform drive uploader (same transport as
  // chat media); the backend stores the returned drive:// URLs on the entry.
  configureCommunityMediaRuntimePort({
    async uploadImages(files: File[]): Promise<string[]> {
      const client = getDriveAppSdkClientWithSession();
      const urls: string[] = [];
      for (const file of files) {
        const uploadResult = await client.uploader.uploadImage({
          file,
          appResourceType: 'community_post',
          appResourceId: 'community',
          scene: 'community',
          source: 'community_post',
          uploadProfileCode: 'image',
          ...(file.name ? { originalFileName: file.name } : {}),
          ...(file.type ? { contentType: file.type } : {}),
        });
        const spaceId = uploadResult.uploadItem.spaceId || uploadResult.uploadSession.spaceId;
        const nodeId = uploadResult.uploadItem.nodeId || uploadResult.uploadSession.nodeId;
        if (!spaceId || !nodeId) {
          throw new Error('drive upload did not return a space or node id');
        }
        urls.push(`drive://spaces/${spaceId}/nodes/${nodeId}`);
      }
      return urls;
    },
  });

  configureCommunityOrderRuntime({
    async createMembershipOrder(
      options: CreateCircleMembershipOrderOptions,
    ): Promise<CircleMembershipOrder> {
      const result = await getSdkClients().orderAppSdkClient.memberships.orders.create(
        {
          action: 'purchase',
          packageId: options.packageId,
          paymentMethod: options.paymentMethod,
          paymentProduct: 'mobile_cashier_h5',
          source: options.source ?? 'community-circle',
        },
        { idempotencyKey: createIdempotencyKey() },
      );
      return {
        orderId: result.orderId,
        orderNo: result.orderNo,
        amount: result.amount,
        cashierUrl: result.cashierUrl,
      };
    },
  });
}

export function isImCommunityH5PortBootstrapped(): boolean {
  return bootstrapped;
}
