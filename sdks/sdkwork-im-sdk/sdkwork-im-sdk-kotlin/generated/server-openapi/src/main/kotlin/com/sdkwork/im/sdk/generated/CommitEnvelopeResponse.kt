package com.sdkwork.im.sdk.generated

data class CommitEnvelopeResponse(
    val eventId: String? = null,
    val tenantId: String? = null,
    val eventType: String? = null,
    val eventVersion: Int? = null,
    val aggregateType: String? = null,
    val aggregateId: String? = null,
    val scopeType: String? = null,
    val scopeId: String? = null,
    val orderingKey: String? = null,
    val orderingSeq: Int? = null,
    val causationId: String? = null,
    val correlationId: String? = null,
    val idempotencyKey: String? = null,
    val actor: EventActor? = null,
    val occurredAt: String? = null,
    val committedAt: String? = null,
    val payloadSchema: String? = null,
    val payload: String? = null,
    val retentionClass: String? = null,
    val auditClass: String? = null
)
