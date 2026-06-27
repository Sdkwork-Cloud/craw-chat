import DOMPurify from 'dompurify';

// Whitelist of tags commonly used in IM / mail message bodies.
const ALLOWED_TAGS = [
  'p', 'br', 'wbr', 'strong', 'em', 'b', 'i', 'u', 's', 'del', 'ins', 'mark',
  'sub', 'sup', 'small',
  'ul', 'ol', 'li', 'dl', 'dt', 'dd',
  'a', 'code', 'pre', 'blockquote',
  'span', 'div',
  'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'hr',
  'img',
  'table', 'thead', 'tbody', 'tfoot', 'tr', 'td', 'th', 'caption',
  'colgroup', 'col',
  'figure', 'figcaption',
];

const ALLOWED_ATTR = [
  'href', 'class', 'target', 'rel', 'src', 'alt', 'title',
  'width', 'height', 'colspan', 'rowspan', 'align', 'dir', 'style',
];

// Defense-in-depth: explicitly forbid tags that enable script/markup injection
// even though ALLOWED_TAGS already excludes them.
const FORBID_TAGS = [
  'script', 'iframe', 'object', 'embed', 'link', 'meta', 'base',
  'form', 'style', 'svg', 'math', 'foreignObject',
];

const FORBID_ATTR = ['srcdoc'];

// Only http/https/blob/mailto/tel schemes are allowed. Relative and fragment
// URIs (no scheme) are preserved. Blocks javascript:/data:/vbscript: and any
// other non-whitelisted scheme.
const ALLOWED_URI_REGEXP = /^(?:(?:https?|blob|mailto|tel):|[^a-z]|[a-z+.-]+(?:[^a-z+.-:]|$))/i;

const SANITIZE_CONFIG = {
  ALLOWED_TAGS,
  ALLOWED_ATTR,
  FORBID_TAGS,
  FORBID_ATTR,
  ALLOWED_URI_REGEXP,
  ALLOW_DATA_ATTR: false,
};

let hooksConfigured = false;

function ensureHooksConfigured(): void {
  if (hooksConfigured) {
    return;
  }
  // Force-remove any on* event handler attribute regardless of ALLOWED_ATTR.
  DOMPurify.addHook('uponSanitizeAttribute', (_node, data) => {
    const attrName = data.attrName;
    if (attrName && attrName.toLowerCase().startsWith('on')) {
      data.keepAttr = false;
    }
  });
  hooksConfigured = true;
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

export function sanitizeHtmlForDisplay(html: string): string {
  const trimmed = html.trim();
  if (!trimmed) {
    return '';
  }

  if (typeof window === 'undefined' || !window.document) {
    // SSR / DOM-less fallback: DOMPurify requires a DOM. Escape fully so no
    // untrusted markup can execute when a DOM is unavailable.
    return escapeHtml(trimmed);
  }

  ensureHooksConfigured();
  return DOMPurify.sanitize(trimmed, SANITIZE_CONFIG) as string;
}
