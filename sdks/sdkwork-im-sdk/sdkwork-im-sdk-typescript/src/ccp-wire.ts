export const IM_CCP_WEBSOCKET_SUBPROTOCOL = 'sdkwork-im.ccp.ws.v1';

const CCP_PROTOCOL = { family: 'ccp', major: 1, minor: 0 } as const;

/** CCP binding 标识常量，对齐服务端 sdkwork-im-ccp-core TransportBinding。 */
export const CCP_WS_BINDING = 'Ws1' as const;
export const CCP_TCP_BINDING = 'Tcp1' as const;
export const CCP_UDP_BINDING = 'Udp1' as const;

/** 所有受支持的 CCP binding 标识。 */
export const CCP_BINDING_IDS = [CCP_WS_BINDING, CCP_TCP_BINDING, CCP_UDP_BINDING] as const;

/** 默认使用 WebSocket binding，保持向后兼容。 */
const DEFAULT_CCP_BINDING = CCP_WS_BINDING;

export interface ImCcpAuthBindContext {
  actorKind: string;
  deviceId?: string;
  principalId: string;
  sessionId?: string;
}

interface ImCcpEnvelope {
  binding: string;
  flags: string[];
  kind: string;
  payload: string;
  protocol: { family: string; major: number; minor: number };
  route: null;
  schema: string;
  scope: null;
  trace_id?: string;
}

function encodeCcpEnvelope(
  schema: string,
  kind: string,
  payload: Record<string, unknown>,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  const envelope: ImCcpEnvelope = {
    protocol: { ...CCP_PROTOCOL },
    binding,
    kind,
    schema,
    scope: null,
    route: null,
    flags: [],
    payload: JSON.stringify(payload),
  };
  return JSON.stringify(envelope);
}

export function encodeCcpControlFrame(
  schema: string,
  controlType: string,
  data: Record<string, unknown>,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  return encodeCcpEnvelope(schema, 'control', { type: controlType, data }, binding);
}

export function encodeCcpBusinessFrame(
  schema: string,
  kind: string,
  payload: Record<string, unknown>,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  return encodeCcpEnvelope(schema, kind, payload, binding);
}

export function decodeCcpEnvelope(raw: string): ImCcpEnvelope | undefined {
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isRecord(parsed) || typeof parsed.payload !== 'string' || typeof parsed.schema !== 'string') {
      return undefined;
    }
    return parsed as unknown as ImCcpEnvelope;
  } catch {
    return undefined;
  }
}

export function parseCcpEnvelopePayload(envelope: ImCcpEnvelope): Record<string, unknown> | undefined {
  try {
    const parsed: unknown = JSON.parse(envelope.payload);
    return isRecord(parsed) ? parsed : undefined;
  } catch {
    return undefined;
  }
}

export function unwrapInboundRealtimeFrame(raw: string): string {
  const envelope = decodeCcpEnvelope(raw);
  if (!envelope) {
    return raw;
  }
  return envelope.payload;
}

export function encodeCcpHelloFrame(binding: string = DEFAULT_CCP_BINDING): string {
  return encodeCcpControlFrame(
    'cc.control.hello.v1',
    'hello',
    {
      protocol: { ...CCP_PROTOCOL },
      binding,
      capabilities: { items: ['payload.json', 'session.resume'] },
    },
    binding,
  );
}

export function encodeCcpAuthBindFrame(
  context: ImCcpAuthBindContext,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  return encodeCcpControlFrame(
    'cc.control.auth_bind.v1',
    'auth_bind',
    {
      principal_id: context.principalId,
      device_id: context.deviceId ?? null,
      session_id: context.sessionId ?? null,
      actor_kind: context.actorKind,
    },
    binding,
  );
}

export function encodeCcpHeartbeatFrame(
  sequence: number,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  return encodeCcpControlFrame(
    'cc.control.heartbeat.v1',
    'heartbeat',
    { sequence },
    binding,
  );
}

export function encodeCcpSessionResumeFrame(
  sessionId: string,
  lastAckedSeq = 0,
  binding: string = DEFAULT_CCP_BINDING,
): string {
  return encodeCcpControlFrame(
    'cc.control.session_resume.v1',
    'session_resume',
    {
      session_id: sessionId,
      last_acked_seq: lastAckedSeq,
    },
    binding,
  );
}

function parseCcpControlPayload(raw: string): Record<string, unknown> | undefined {
  const envelope = decodeCcpEnvelope(raw);
  if (!envelope) {
    return undefined;
  }
  return parseCcpEnvelopePayload(envelope);
}

function ccpControlPayloadData(payload: Record<string, unknown> | undefined): Record<string, unknown> | undefined {
  if (!payload) {
    return undefined;
  }
  const data = payload.data;
  return isRecord(data) ? data : payload;
}

function ccpCapabilityItems(payload: Record<string, unknown> | undefined): string[] {
  const data = ccpControlPayloadData(payload);
  const capabilities = data?.capabilities;
  if (!isRecord(capabilities)) {
    return [];
  }
  const items = capabilities.items;
  if (!Array.isArray(items)) {
    return [];
  }
  return items.filter((item): item is string => typeof item === 'string');
}

export function ccpHelloAckNegotiatesSessionResume(raw: string): boolean {
  const payload = parseCcpControlPayload(raw);
  if (pickString(payload?.type) !== 'hello_ack') {
    return false;
  }
  return ccpCapabilityItems(payload).includes('session.resume');
}

export function isCcpHelloAckEnvelope(raw: string): boolean {
  const envelope = decodeCcpEnvelope(raw);
  return envelope?.schema === 'cc.control.hello_ack.v1';
}

export function isCcpAuthOkEnvelope(raw: string): boolean {
  const envelope = decodeCcpEnvelope(raw);
  return envelope?.schema === 'cc.control.auth_ok.v1';
}

export function isCcpSessionResumedEnvelope(raw: string): boolean {
  const envelope = decodeCcpEnvelope(raw);
  return envelope?.schema === 'cc.control.session_resumed.v1';
}

export function decodeJwtPayload(token: string | undefined): Record<string, unknown> | undefined {
  if (!token) {
    return undefined;
  }
  const segment = token.split('.')[1];
  if (!segment) {
    return undefined;
  }
  try {
    const base64 = segment.replace(/-/g, '+').replace(/_/g, '/');
    const padded = `${base64}${'='.repeat((4 - (base64.length % 4)) % 4)}`;
    const json = decodeBase64Utf8(padded);
    const parsed: unknown = JSON.parse(json);
    return isRecord(parsed) ? parsed : undefined;
  } catch {
    return undefined;
  }
}

export function resolveCcpAuthBindContext(params: {
  accessToken?: string;
  actorKind?: string;
  authOk?: Record<string, unknown>;
  deviceId?: string;
}): ImCcpAuthBindContext | undefined {
  const authOk = params.authOk;
  const jwtClaims = decodeJwtPayload(params.accessToken);
  const principalId = pickString(authOk?.principalId, jwtClaims?.user_id, jwtClaims?.userId);
  if (!principalId) {
    return undefined;
  }
  return {
    principalId,
    deviceId: pickString(authOk?.deviceId, params.deviceId, jwtClaims?.device_id, jwtClaims?.deviceId),
    sessionId: pickString(authOk?.sessionId, jwtClaims?.session_id, jwtClaims?.sessionId),
    actorKind: pickString(authOk?.actorKind, jwtClaims?.subject_type, params.actorKind) ?? 'user',
  };
}

function decodeBase64Utf8(value: string): string {
  type RuntimeBuffer = {
    from: (value: string, encoding: 'base64') => { toString: (encoding: 'utf8') => string };
  };
  const runtimeBuffer = (globalThis as { Buffer?: RuntimeBuffer }).Buffer;
  if (runtimeBuffer) {
    return runtimeBuffer.from(value, 'base64').toString('utf8');
  }
  if (typeof globalThis.atob !== 'function') {
    throw new Error('Base64 decoding is not available in this runtime.');
  }
  const binary = globalThis.atob(value);
  const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
  return new TextDecoder().decode(bytes);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value;
    }
  }
  return undefined;
}
