import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { LagPageData, ProviderBindingDriftPageData, ProviderBindingSnapshotPageData } from '../types';


export class OpsDiagnosticsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve diagnostics */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/ops/diagnostics`));
  }
}

export interface OpsProviderBindingsDriftRetrieveParams {
  pageSize?: number;
  cursor?: string;
}

export class OpsProviderBindingsDriftApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve provider binding drift */
  async retrieve(params?: OpsProviderBindingsDriftRetrieveParams): Promise<ProviderBindingDriftPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ProviderBindingDriftPageData>(appendQueryString(backendApiPath(`/ops/provider_bindings/drift`), query));
  }
}

export interface OpsProviderBindingsListParams {
  pageSize?: number;
  cursor?: string;
}

export class OpsProviderBindingsApi {
  private client: HttpClient;
  public readonly drift: OpsProviderBindingsDriftApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.drift = new OpsProviderBindingsDriftApi(client);
  }


/** List provider bindings */
  async list(params?: OpsProviderBindingsListParams): Promise<ProviderBindingSnapshotPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ProviderBindingSnapshotPageData>(appendQueryString(backendApiPath(`/ops/provider_bindings`), query));
  }
}

export class OpsRuntimeDirApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Inspect runtime directory */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/ops/runtime_dir`));
  }
}

export class OpsCommercialReadinessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve commercial readiness */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/ops/commercial_readiness`));
  }
}

export interface OpsLagRetrieveParams {
  pageSize?: number;
  cursor?: string;
}

export class OpsLagApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve operational lag */
  async retrieve(params?: OpsLagRetrieveParams): Promise<LagPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LagPageData>(appendQueryString(backendApiPath(`/ops/lag`), query));
  }
}

export class OpsClusterApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve cluster state */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/ops/cluster`));
  }
}

export class OpsHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve ops health */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/ops/health`));
  }
}

export class OpsApi {

  public readonly health: OpsHealthApi;
  public readonly cluster: OpsClusterApi;
  public readonly lag: OpsLagApi;
  public readonly commercialReadiness: OpsCommercialReadinessApi;
  public readonly runtimeDir: OpsRuntimeDirApi;
  public readonly providerBindings: OpsProviderBindingsApi;
  public readonly diagnostics: OpsDiagnosticsApi;

  constructor(client: HttpClient) {

    this.health = new OpsHealthApi(client);
    this.cluster = new OpsClusterApi(client);
    this.lag = new OpsLagApi(client);
    this.commercialReadiness = new OpsCommercialReadinessApi(client);
    this.runtimeDir = new OpsRuntimeDirApi(client);
    this.providerBindings = new OpsProviderBindingsApi(client);
    this.diagnostics = new OpsDiagnosticsApi(client);
  }

}

export function createOpsApi(client: HttpClient): OpsApi {
  return new OpsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}


interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
