import type { ComponentType } from "react";

/**
 * Filled SVG variants for the bottom tab bar, restored from the original
 * sdkwork-im-h5 UI. The outline icons come from lucide-react; each tab's
 * active state swaps to its filled counterpart (scale-110, full opacity).
 */
export interface TabSolidIconProps {
  className?: string;
  strokeWidth?: number;
}

/** Filled message bubble (active state of the chat tab). */
export const TabSolidMessage: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" fill="currentColor" />
  </svg>
);

/** Filled four-square grid (active state of the workspace tab). */
export const TabSolidWorkspace: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <rect x="3" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="14" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="3" y="14" width="7" height="7" rx="1" fill="currentColor" />
  </svg>
);

/** Filled bot head (active state of the agents tab). */
export const TabSolidBot: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <path d="M12 2v6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <path d="M8 8V6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <rect x="3" y="10" width="18" height="12" rx="2" fill="currentColor" stroke="currentColor" strokeWidth="2" />
    <circle cx="8.5" cy="15.5" r="1.5" fill="white" />
    <circle cx="15.5" cy="15.5" r="1.5" fill="white" />
  </svg>
);

/** Filled compass (active state of the discover tab). */
export const TabSolidDiscover: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" fill="transparent" />
    <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" fill="currentColor" />
  </svg>
);

/** Filled user (active state of the me tab). */
export const TabSolidUser: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="7" r="5" fill="currentColor" />
    <path d="M20 21a8 8 0 0 0-16 0" fill="currentColor" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
);

/** Filled receipt (active state of the orders tab), matching lucide ReceiptText. */
export const TabSolidOrders: ComponentType<TabSolidIconProps> = ({ className }: TabSolidIconProps) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <path
      d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"
      fill="currentColor"
    />
    <path d="M14 8H8" stroke="white" strokeWidth="1.6" strokeLinecap="round" />
    <path d="M16 12H8" stroke="white" strokeWidth="1.6" strokeLinecap="round" />
    <path d="M13 16H8" stroke="white" strokeWidth="1.6" strokeLinecap="round" />
  </svg>
);
