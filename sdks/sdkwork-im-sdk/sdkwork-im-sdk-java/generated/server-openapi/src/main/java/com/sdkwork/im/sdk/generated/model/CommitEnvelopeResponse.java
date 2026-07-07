package com.sdkwork.im.sdk.generated.model;


public class CommitEnvelopeResponse {
    private String eventId;
    private String tenantId;
    private String eventType;
    private Integer eventVersion;
    private String aggregateType;
    private String aggregateId;
    private String scopeType;
    private String scopeId;
    private String orderingKey;
    private Integer orderingSeq;
    private String causationId;
    private String correlationId;
    private String idempotencyKey;
    private EventActor actor;
    private String occurredAt;
    private String committedAt;
    private String payloadSchema;
    private String payload;
    private String retentionClass;
    private String auditClass;

    public String getEventId() {
        return this.eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getEventType() {
        return this.eventType;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public Integer getEventVersion() {
        return this.eventVersion;
    }

    public void setEventVersion(Integer eventVersion) {
        this.eventVersion = eventVersion;
    }

    public String getAggregateType() {
        return this.aggregateType;
    }

    public void setAggregateType(String aggregateType) {
        this.aggregateType = aggregateType;
    }

    public String getAggregateId() {
        return this.aggregateId;
    }

    public void setAggregateId(String aggregateId) {
        this.aggregateId = aggregateId;
    }

    public String getScopeType() {
        return this.scopeType;
    }

    public void setScopeType(String scopeType) {
        this.scopeType = scopeType;
    }

    public String getScopeId() {
        return this.scopeId;
    }

    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public String getOrderingKey() {
        return this.orderingKey;
    }

    public void setOrderingKey(String orderingKey) {
        this.orderingKey = orderingKey;
    }

    public Integer getOrderingSeq() {
        return this.orderingSeq;
    }

    public void setOrderingSeq(Integer orderingSeq) {
        this.orderingSeq = orderingSeq;
    }

    public String getCausationId() {
        return this.causationId;
    }

    public void setCausationId(String causationId) {
        this.causationId = causationId;
    }

    public String getCorrelationId() {
        return this.correlationId;
    }

    public void setCorrelationId(String correlationId) {
        this.correlationId = correlationId;
    }

    public String getIdempotencyKey() {
        return this.idempotencyKey;
    }

    public void setIdempotencyKey(String idempotencyKey) {
        this.idempotencyKey = idempotencyKey;
    }

    public EventActor getActor() {
        return this.actor;
    }

    public void setActor(EventActor actor) {
        this.actor = actor;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }

    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }

    public String getCommittedAt() {
        return this.committedAt;
    }

    public void setCommittedAt(String committedAt) {
        this.committedAt = committedAt;
    }

    public String getPayloadSchema() {
        return this.payloadSchema;
    }

    public void setPayloadSchema(String payloadSchema) {
        this.payloadSchema = payloadSchema;
    }

    public String getPayload() {
        return this.payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public String getRetentionClass() {
        return this.retentionClass;
    }

    public void setRetentionClass(String retentionClass) {
        this.retentionClass = retentionClass;
    }

    public String getAuditClass() {
        return this.auditClass;
    }

    public void setAuditClass(String auditClass) {
        this.auditClass = auditClass;
    }
}
