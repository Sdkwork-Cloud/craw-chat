import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { AgentToolCall, AppendAgentResponseDeltaRequest, AutomationExecution, AutomationExecutionRequestResponse, CompleteAgentResponseRequest, CompleteAgentToolCallRequest, RequestAgentToolCallRequest, RequestAutomationExecution, StartAgentResponseRequest, StreamFrame, StreamSession } from '../types';


export class AutomationExecutionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Request an automation execution */
  async create(body: RequestAutomationExecution, requestOptions?: ApiRequestOptions): Promise<AutomationExecutionRequestResponse> {
    return this.client.request<AutomationExecutionRequestResponse>(appApiPath(`/automation/executions`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Get an automation execution */
  async retrieve(executionId: string, requestOptions?: ApiRequestOptions): Promise<AutomationExecution> {
    return this.client.request<AutomationExecution>(appApiPath(`/automation/executions/${serializePathParameter(executionId, { name: 'executionId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class AutomationAgentToolCallsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Request an agent tool call */
  async create(body: RequestAgentToolCallRequest, requestOptions?: ApiRequestOptions): Promise<AgentToolCall> {
    return this.client.request<AgentToolCall>(appApiPath(`/automation/agent_tool_calls`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Complete an agent tool call */
  async complete(executionId: string, toolCallId: string, body: CompleteAgentToolCallRequest, requestOptions?: ApiRequestOptions): Promise<AgentToolCall> {
    return this.client.request<AgentToolCall>(appApiPath(`/automation/executions/${serializePathParameter(executionId, { name: 'executionId', style: 'simple', explode: false })}/agent_tool_calls/${serializePathParameter(toolCallId, { name: 'toolCallId', style: 'simple', explode: false })}/complete`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AutomationAgentResponsesFramesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Append a frame to an agent response stream */
  async create(streamId: string, body: AppendAgentResponseDeltaRequest, requestOptions?: ApiRequestOptions): Promise<StreamFrame> {
    return this.client.request<StreamFrame>(appApiPath(`/automation/agent_responses/${serializePathParameter(streamId, { name: 'streamId', style: 'simple', explode: false })}/frames`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AutomationAgentResponsesApi {
  private client: HttpClient;
  public readonly frames: AutomationAgentResponsesFramesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.frames = new AutomationAgentResponsesFramesApi(client);
  }


/** Start an agent response stream */
  async create(body: StartAgentResponseRequest, requestOptions?: ApiRequestOptions): Promise<StreamSession> {
    return this.client.request<StreamSession>(appApiPath(`/automation/agent_responses`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Complete an agent response stream */
  async complete(streamId: string, body: CompleteAgentResponseRequest, requestOptions?: ApiRequestOptions): Promise<StreamSession> {
    return this.client.request<StreamSession>(appApiPath(`/automation/agent_responses/${serializePathParameter(streamId, { name: 'streamId', style: 'simple', explode: false })}/complete`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AutomationApi {
  public readonly agentResponses: AutomationAgentResponsesApi;
  public readonly agentToolCalls: AutomationAgentToolCallsApi;
  public readonly executions: AutomationExecutionsApi;

  constructor(client: HttpClient) {
    this.agentResponses = new AutomationAgentResponsesApi(client);
    this.agentToolCalls = new AutomationAgentToolCallsApi(client);
    this.executions = new AutomationExecutionsApi(client);
  }

}

export function createAutomationApi(client: HttpClient): AutomationApi {
  return new AutomationApi(client);
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
