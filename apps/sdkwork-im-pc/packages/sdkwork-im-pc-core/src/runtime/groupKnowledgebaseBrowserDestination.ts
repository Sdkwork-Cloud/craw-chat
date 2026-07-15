import { isValidGroupKnowledgebaseLaunchTicket } from '../host/knowledgebaseDesktopHost';

const KNOWLEDGEBASE_APP_URL_ENV_KEY = 'VITE_SDKWORK_KNOWLEDGEBASE_APP_URL';

export interface GroupKnowledgebaseBrowserDestinationOptions {
  allowInsecureLoopback?: boolean;
}

function isLoopbackHost(hostname: string): boolean {
  const normalized = hostname.trim().toLowerCase();
  return normalized === 'localhost'
    || normalized === '127.0.0.1'
    || normalized === '::1'
    || normalized === '[::1]';
}

function resolveConfiguredKnowledgebaseAppBaseUrl(): string | null {
  const configured = import.meta.env[KNOWLEDGEBASE_APP_URL_ENV_KEY];
  if (typeof configured !== 'string' || configured.trim().length === 0) {
    return null;
  }
  return configured.trim();
}

export function resolveGroupKnowledgebaseBrowserBaseUrl(
  configuredBaseUrl: string | null | undefined,
  options: GroupKnowledgebaseBrowserDestinationOptions = {},
): URL | null {
  if (!configuredBaseUrl || configuredBaseUrl.trim().length === 0) {
    return null;
  }

  try {
    const base = new URL(configuredBaseUrl.trim());
    const allowsHttp = options.allowInsecureLoopback === true
      && base.protocol === 'http:'
      && isLoopbackHost(base.hostname);
    if ((base.protocol !== 'https:' && !allowsHttp)
      || base.username
      || base.password
      || base.search
      || base.hash) {
      return null;
    }
    if (!base.pathname.endsWith('/')) {
      base.pathname += '/';
    }
    return base;
  } catch {
    return null;
  }
}

function allowsInsecureLoopbackKnowledgebaseUrl(): boolean {
  return import.meta.env.DEV === true;
}

export function isGroupKnowledgebaseBrowserDestinationConfigured(): boolean {
  return resolveGroupKnowledgebaseBrowserBaseUrl(
    resolveConfiguredKnowledgebaseAppBaseUrl(),
    { allowInsecureLoopback: allowsInsecureLoopbackKnowledgebaseUrl() },
  ) !== null;
}

export function buildGroupKnowledgebaseBrowserUrlForBaseUrl(
  launchTicket: string,
  configuredBaseUrl: string | null | undefined,
  options: GroupKnowledgebaseBrowserDestinationOptions = {},
): string | null {
  if (!isValidGroupKnowledgebaseLaunchTicket(launchTicket)) {
    return null;
  }
  const base = resolveGroupKnowledgebaseBrowserBaseUrl(configuredBaseUrl, options);
  if (!base) {
    return null;
  }

  const destination = new URL('group-launch', base);
  destination.hash = `ticket=${encodeURIComponent(launchTicket)}`;
  return destination.toString();
}

export function buildGroupKnowledgebaseBrowserUrl(launchTicket: string): string | null {
  return buildGroupKnowledgebaseBrowserUrlForBaseUrl(
    launchTicket,
    resolveConfiguredKnowledgebaseAppBaseUrl(),
    { allowInsecureLoopback: allowsInsecureLoopbackKnowledgebaseUrl() },
  );
}
