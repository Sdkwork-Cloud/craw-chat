import type { SocialApi } from '../generated/server-openapi/dist/index.js';
import type { ImTransportClientLike } from './transport-client-like.js';

/**
 * Composed social surface adapter.
 *
 * The generated transport exposes the pending incoming friend-request count
 * as the path-nested `friendRequests.pending.count.retrieve()` shape (GET
 * `/social/friend_requests/pending/count`). The composed facade flattens it
 * to `friendRequests.pendingCount()` exactly as declared by
 * `ImTransportClientLike['social']`; every other surface passes through
 * unmodified.
 *
 * The facade types describe application-facing views, so the generated
 * surface is adapted at this boundary (same `as unknown as` pattern the
 * transport itself uses in `sdk.ts`).
 */
export function composeSocialSurface(social: SocialApi): ImTransportClientLike['social'] {
  const friendRequests = social.friendRequests;
  return {
    users: social.users,
    contacts: social.contacts,
    friendships: social.friendships,
    userBlocks: social.userBlocks,
    friendRequests: {
      list: (params?: Parameters<typeof friendRequests.list>[0]) => friendRequests.list(params),
      create: (body: Parameters<typeof friendRequests.create>[0]) => friendRequests.create(body),
      accept: (requestId: string) => friendRequests.accept(requestId),
      decline: (requestId: string) => friendRequests.decline(requestId),
      cancel: (requestId: string) => friendRequests.cancel(requestId),
      pendingCount: () => friendRequests.pending.count.retrieve(),
    },
  } as unknown as ImTransportClientLike['social'];
}
