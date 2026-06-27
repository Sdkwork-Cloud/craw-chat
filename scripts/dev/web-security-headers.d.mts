export const WEB_SECURITY_HEADERS: Record<string, string>;

export function createWebSecurityHeaders(options?: {
  allowInlineScripts?: boolean;
  allowUnsafeEval?: boolean;
}): Record<string, string>;

export function applyWebSecurityHeaders(
  target: {
    setHeader(name: string, value: string): void;
  },
  options?: {
    allowInlineScripts?: boolean;
    allowUnsafeEval?: boolean;
  },
): Record<string, string>;
