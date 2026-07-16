/**
 * The generated backend SDK has no platform-broadcast operation. Publishing a
 * broadcast UI without that authority would misrepresent its delivery state.
 */
export const ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE =
  'Platform announcements are unavailable because the required backend API contract has not been published.';

export const adminAnnouncementCapability = {
  available: false as const,
  reason: ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE,
};
