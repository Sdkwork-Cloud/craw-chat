import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcSession, RtcSessionMutationResponse, RtcSignalEvent, UpdateRtcSessionRequest } from '../types';


export class CallsSessionsCredentialsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Issue an RTC media participant credential for an IM call */
  async create(rtcSessionId: string, body: IssueRtcParticipantCredentialRequest, requestOptions?: ApiRequestOptions): Promise<RtcParticipantCredential> {
    return this.client.request<RtcParticipantCredential>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/credentials`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Refresh an expiring RTC media participant credential */
  async refresh(rtcSessionId: string, body: IssueRtcParticipantCredentialRequest, requestOptions?: ApiRequestOptions): Promise<RtcParticipantCredential> {
    return this.client.request<RtcParticipantCredential>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/credentials/refresh`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface CallsSessionsSignalsListParams {
  afterSignalSeq?: string;
  cursor?: string;
  pageSize?: number;
}

export class CallsSessionsSignalsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List IM call signaling events */
  async list(rtcSessionId: string, params?: CallsSessionsSignalsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: RtcSignalEvent[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'afterSignalSeq', value: params?.afterSignalSeq, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: RtcSignalEvent[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/signals`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Post an IM call signaling event */
  async create(rtcSessionId: string, body: PostRtcSignalRequest, requestOptions?: ApiRequestOptions): Promise<RtcSignalEvent> {
    return this.client.request<RtcSignalEvent>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/signals`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class CallsSessionsApi {
  private client: HttpClient;
  public readonly signals: CallsSessionsSignalsApi;
  public readonly credentials: CallsSessionsCredentialsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.signals = new CallsSessionsSignalsApi(client);
    this.credentials = new CallsSessionsCredentialsApi(client);
  }


/** Create an IM call signaling session */
  async create(body: CreateRtcSessionRequest, requestOptions?: ApiRequestOptions): Promise<RtcSessionMutationResponse> {
    return this.client.request<RtcSessionMutationResponse>(imApiPath(`/calls/sessions`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Retrieve IM call signaling session state */
  async retrieve(rtcSessionId: string, requestOptions?: ApiRequestOptions): Promise<RtcSession> {
    return this.client.request<RtcSession>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Invite participants into an IM call signaling session */
  async invite(rtcSessionId: string, body: InviteRtcSessionRequest, requestOptions?: ApiRequestOptions): Promise<RtcSessionMutationResponse> {
    return this.client.request<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/invite`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Accept an IM call signaling session */
  async accept(rtcSessionId: string, body: UpdateRtcSessionRequest, requestOptions?: ApiRequestOptions): Promise<RtcSessionMutationResponse> {
    return this.client.request<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/accept`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Reject an IM call signaling session */
  async reject(rtcSessionId: string, body: UpdateRtcSessionRequest, requestOptions?: ApiRequestOptions): Promise<RtcSessionMutationResponse> {
    return this.client.request<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/reject`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** End an IM call signaling session */
  async end(rtcSessionId: string, body: UpdateRtcSessionRequest, requestOptions?: ApiRequestOptions): Promise<RtcSessionMutationResponse> {
    return this.client.request<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/end`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class CallsApi {
  public readonly sessions: CallsSessionsApi;

  constructor(client: HttpClient) {
    this.sessions = new CallsSessionsApi(client);
  }

}

export function createCallsApi(client: HttpClient): CallsApi {
  return new CallsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
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
