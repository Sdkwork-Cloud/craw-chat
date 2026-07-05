import type { SocialEventActor } from './social-event-actor';

export interface SocialCommitEnvelope {
  eventId: string;
  tenantId: string;
  eventType: string;
  eventVersion: number;
  aggregateType: string;
  aggregateId: string;
  scopeType: string;
  scopeId: string;
  orderingKey: string;
  orderingSeq: string;
  causationId?: string | null;
  correlationId?: string | null;
  idempotencyKey?: string | null;
  actor: SocialEventActor;
  occurredAt: string;
  committedAt: string;
  payloadSchema?: string | null;
  payload: string;
  retentionClass: string;
  auditClass: string;
}
