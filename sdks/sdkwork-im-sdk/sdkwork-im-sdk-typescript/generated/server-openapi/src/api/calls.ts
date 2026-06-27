import { imApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcSession, RtcSessionMutationResponse, RtcSignalEvent, UpdateRtcSessionRequest } from '../types';


export class CallsSessionsCredentialsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Issue an RTC media participant credential for an IM call */
  async create(rtcSessionId: string, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential> {
    return this.client.post<RtcParticipantCredential>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/credentials`), body, undefined, undefined, 'application/json');
  }
}

export class CallsSessionsSignalsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Post an IM call signaling event */
  async create(rtcSessionId: string, body: PostRtcSignalRequest): Promise<RtcSignalEvent> {
    return this.client.post<RtcSignalEvent>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/signals`), body, undefined, undefined, 'application/json');
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
  async create(body: CreateRtcSessionRequest): Promise<RtcSessionMutationResponse> {
    return this.client.post<RtcSessionMutationResponse>(imApiPath(`/calls/sessions`), body, undefined, undefined, 'application/json');
  }

/** Retrieve IM call signaling session state */
  async retrieve(rtcSessionId: string): Promise<RtcSession> {
    return this.client.get<RtcSession>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}`));
  }

/** Invite participants into an IM call signaling session */
  async invite(rtcSessionId: string, body: InviteRtcSessionRequest): Promise<RtcSessionMutationResponse> {
    return this.client.post<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/invite`), body, undefined, undefined, 'application/json');
  }

/** Accept an IM call signaling session */
  async accept(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse> {
    return this.client.post<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/accept`), body, undefined, undefined, 'application/json');
  }

/** Reject an IM call signaling session */
  async reject(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse> {
    return this.client.post<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/reject`), body, undefined, undefined, 'application/json');
  }

/** End an IM call signaling session */
  async end(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse> {
    return this.client.post<RtcSessionMutationResponse>(imApiPath(`/calls/sessions/${serializePathParameter(rtcSessionId, { name: 'rtcSessionId', style: 'simple', explode: false })}/end`), body, undefined, undefined, 'application/json');
  }
}

export class CallsApi {
  private client: HttpClient;
  public readonly sessions: CallsSessionsApi;

  constructor(client: HttpClient) {
    this.client = client;
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
