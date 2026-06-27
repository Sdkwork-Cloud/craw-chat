import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AgentToolCall, AppendAgentResponseDeltaRequest, AutomationExecution, AutomationExecutionRequestResponse, CompleteAgentResponseRequest, CompleteAgentToolCallRequest, RequestAgentToolCallRequest, RequestAutomationExecution, StartAgentResponseRequest, StreamFrame, StreamSession } from '../types';


export class AutomationExecutionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Request an automation execution */
  async create(body: RequestAutomationExecution): Promise<AutomationExecutionRequestResponse> {
    return this.client.post<AutomationExecutionRequestResponse>(appApiPath(`/automation/executions`), body, undefined, undefined, 'application/json');
  }

/** Get an automation execution */
  async retrieve(executionId: string): Promise<AutomationExecution> {
    return this.client.get<AutomationExecution>(appApiPath(`/automation/executions/${serializePathParameter(executionId, { name: 'executionId', style: 'simple', explode: false })}`));
  }
}

export class AutomationAgentToolCallsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Request an agent tool call */
  async create(body: RequestAgentToolCallRequest): Promise<AgentToolCall> {
    return this.client.post<AgentToolCall>(appApiPath(`/automation/agent_tool_calls`), body, undefined, undefined, 'application/json');
  }

/** Complete an agent tool call */
  async complete(executionId: string, toolCallId: string, body: CompleteAgentToolCallRequest): Promise<AgentToolCall> {
    return this.client.post<AgentToolCall>(appApiPath(`/automation/executions/${serializePathParameter(executionId, { name: 'executionId', style: 'simple', explode: false })}/agent_tool_calls/${serializePathParameter(toolCallId, { name: 'toolCallId', style: 'simple', explode: false })}/complete`), body, undefined, undefined, 'application/json');
  }
}

export class AutomationAgentResponsesFramesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Append a frame to an agent response stream */
  async create(streamId: string, body: AppendAgentResponseDeltaRequest): Promise<StreamFrame> {
    return this.client.post<StreamFrame>(appApiPath(`/automation/agent_responses/${serializePathParameter(streamId, { name: 'streamId', style: 'simple', explode: false })}/frames`), body, undefined, undefined, 'application/json');
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
  async create(body: StartAgentResponseRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(appApiPath(`/automation/agent_responses`), body, undefined, undefined, 'application/json');
  }

/** Complete an agent response stream */
  async complete(streamId: string, body: CompleteAgentResponseRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(appApiPath(`/automation/agent_responses/${serializePathParameter(streamId, { name: 'streamId', style: 'simple', explode: false })}/complete`), body, undefined, undefined, 'application/json');
  }
}

export class AutomationApi {
  private client: HttpClient;
  public readonly agentResponses: AutomationAgentResponsesApi;
  public readonly agentToolCalls: AutomationAgentToolCallsApi;
  public readonly executions: AutomationExecutionsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.agentResponses = new AutomationAgentResponsesApi(client);
    this.agentToolCalls = new AutomationAgentToolCallsApi(client);
    this.executions = new AutomationExecutionsApi(client);
  }

}

export function createAutomationApi(client: HttpClient): AutomationApi {
  return new AutomationApi(client);
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
