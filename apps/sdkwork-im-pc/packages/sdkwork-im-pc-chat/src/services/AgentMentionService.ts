import type { ImMentionContentPart, TextContentPart } from '@sdkwork/im-sdk';
import type { ChatAgentAssignment } from '@sdkwork/im-pc-types';

export type AgentMentionPart = ImMentionContentPart;

export type AgentTextPart = Omit<TextContentPart, 'text'> & {
  text: string;
};

export type AgentMentionContentPart = AgentMentionPart | AgentTextPart;

export interface ActiveAgentMentionQuery {
  fromTextOffset: number;
  query: string;
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

/**
 * Returns true only for a parts payload that contains at least one complete
 * agent mention and no malformed mention entries. This is used when the
 * assignment catalog is unavailable: an unresolved plain-text `@` must never
 * be silently converted into an offline agent dispatch.
 */
export function hasStructuredAgentMentionParts(parts: readonly unknown[] | undefined): boolean {
  if (!Array.isArray(parts)) {
    return false;
  }
  let hasMention = false;
  for (const part of parts) {
    if (!part || typeof part !== 'object' || Array.isArray(part)) {
      return false;
    }
    const record = part as Record<string, unknown>;
    if (record.kind === 'text') {
      if (typeof record.text !== 'string') {
        return false;
      }
      continue;
    }
    if (record.kind !== 'mention') {
      return false;
    }
    hasMention = true;
    if (
      record.targetKind !== 'agent'
      || typeof record.targetId !== 'string'
      || !STANDARD_AGENT_ID_PATTERN.test(record.targetId.trim())
      || typeof record.displayText !== 'string'
      || !record.displayText.trim().startsWith('@')
      || !Number.isSafeInteger(record.assignmentGeneration)
      || (record.assignmentGeneration as number) < 1
    ) {
      return false;
    }
  }
  return hasMention;
}

function labelForAgent(agent: ChatAgentAssignment): string {
  return agent.name?.trim() || agent.agentId.trim();
}

function disambiguationSuffix(
  agent: ChatAgentAssignment,
  agents: readonly ChatAgentAssignment[],
  label: string,
): string {
  const normalizedLabel = label.toLocaleLowerCase();
  const duplicateIds = agents
    .filter((candidate) => labelForAgent(candidate).toLocaleLowerCase() === normalizedLabel)
    .map((candidate) => candidate.agentId.trim())
    .filter(Boolean);
  const id = agent.agentId.trim();
  const segments = id.replace(/^agent\./u, '').split('.').filter(Boolean);
  for (let length = 1; length <= segments.length; length += 1) {
    const suffix = segments.slice(-length).join('.');
    const collides = duplicateIds.some((candidateId) => (
      candidateId !== id
      && candidateId.replace(/^agent\./u, '').split('.').filter(Boolean).slice(-length).join('.') === suffix
    ));
    if (!collides) {
      return suffix;
    }
  }
  return id;
}

export function mentionLabelForAgent(
  agent: ChatAgentAssignment,
  agents: readonly ChatAgentAssignment[],
): string {
  const label = labelForAgent(agent);
  const duplicate = agents.filter((candidate) => labelForAgent(candidate).toLocaleLowerCase() === label.toLocaleLowerCase()).length > 1;
  if (!duplicate) {
    return label;
  }
  return `${label} (${disambiguationSuffix(agent, agents, label)})`;
}

function isMentionEndBoundary(value: string | undefined): boolean {
  return value === undefined || !/[\p{L}\p{N}_.-]/u.test(value);
}

function isMentionStartBoundary(value: string | undefined): boolean {
  return value === undefined || /[\s([{]/u.test(value);
}

export function resolveActiveAgentMentionQuery(textBeforeCursor: string): ActiveAgentMentionQuery | undefined {
  const markerIndex = textBeforeCursor.lastIndexOf('@');
  if (markerIndex < 0) {
    return undefined;
  }
  const previousCharacter = markerIndex > 0 ? textBeforeCursor[markerIndex - 1] : undefined;
  if (previousCharacter && !isMentionStartBoundary(previousCharacter)) {
    return undefined;
  }
  const query = textBeforeCursor.slice(markerIndex + 1);
  if (query.includes('\n') || query.includes('@') || query.length > 64) {
    return undefined;
  }
  return {
    fromTextOffset: markerIndex,
    query,
  };
}

export function filterMentionAgents(
  agents: readonly ChatAgentAssignment[],
  query: string,
  limit = 8,
): ChatAgentAssignment[] {
  const normalizedQuery = query.trim().toLocaleLowerCase();
  const seen = new Set<string>();
  return agents
    .filter((agent) => agent.enabled !== false && agent.agentId.trim().length > 0)
    .filter((agent) => {
      const id = agent.agentId.trim();
      if (seen.has(id)) {
        return false;
      }
      seen.add(id);
      if (!normalizedQuery) {
        return true;
      }
      return mentionLabelForAgent(agent, agents).toLocaleLowerCase().includes(normalizedQuery)
        || id.toLocaleLowerCase().includes(normalizedQuery);
    })
    .slice(0, Math.max(1, Math.floor(limit)));
}

export function buildAgentMentionParts(
  content: string,
  agents: readonly ChatAgentAssignment[],
  assignmentGeneration: number | undefined,
): AgentMentionContentPart[] | undefined {
  if (!Number.isSafeInteger(assignmentGeneration) || (assignmentGeneration ?? 0) < 1) {
    return undefined;
  }
  const targets = agents
    .filter((agent) => agent.enabled !== false && agent.agentId.trim().length > 0)
    .map((agent) => ({
      agent,
      marker: `@${mentionLabelForAgent(agent, agents)}`,
    }))
    .sort((left, right) => right.marker.length - left.marker.length);
  if (targets.length === 0) {
    return undefined;
  }

  const parts: AgentMentionContentPart[] = [];
  let cursor = 0;
  let mentionCount = 0;
  while (cursor < content.length) {
    let nextIndex = -1;
    let nextTarget: (typeof targets)[number] | undefined;
    for (const target of targets) {
      let candidate = content.indexOf(target.marker, cursor);
      while (
        candidate >= 0
        && (
          !isMentionStartBoundary(candidate > 0 ? content[candidate - 1] : undefined)
          || !isMentionEndBoundary(content[candidate + target.marker.length])
        )
      ) {
        candidate = content.indexOf(target.marker, candidate + target.marker.length);
      }
      if (candidate >= 0 && (nextIndex < 0 || candidate < nextIndex)) {
        nextIndex = candidate;
        nextTarget = target;
      }
    }
    if (nextIndex < 0 || !nextTarget) {
      break;
    }
    if (nextIndex > cursor) {
      parts.push({ kind: 'text', text: content.slice(cursor, nextIndex) });
    }
    parts.push({
      kind: 'mention',
      targetKind: 'agent',
      targetId: nextTarget.agent.agentId.trim(),
      displayText: nextTarget.marker,
      assignmentGeneration: assignmentGeneration as number,
    });
    mentionCount += 1;
    cursor = nextIndex + nextTarget.marker.length;
  }
  if (mentionCount === 0) {
    return undefined;
  }
  if (cursor < content.length) {
    parts.push({ kind: 'text', text: content.slice(cursor) });
  }
  return parts;
}
