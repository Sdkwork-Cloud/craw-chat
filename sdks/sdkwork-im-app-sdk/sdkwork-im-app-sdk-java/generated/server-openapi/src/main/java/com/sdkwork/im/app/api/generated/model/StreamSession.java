package com.sdkwork.im.app.api.generated.model;


public class StreamSession {
    private String tenantId;
    private String streamId;
    private String streamType;
    private String scopeKind;
    private String scopeId;
    private String durabilityClass;
    private String orderingScope;
    private String schemaRef;
    private String state;
    private Integer lastFrameSeq;
    private Integer lastCheckpointSeq;
    private String resultMessageId;
    private String openedAt;
    private String closedAt;
    private String expiresAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getStreamId() {
        return this.streamId;
    }
    
    public void setStreamId(String streamId) {
        this.streamId = streamId;
    }

    public String getStreamType() {
        return this.streamType;
    }
    
    public void setStreamType(String streamType) {
        this.streamType = streamType;
    }

    public String getScopeKind() {
        return this.scopeKind;
    }
    
    public void setScopeKind(String scopeKind) {
        this.scopeKind = scopeKind;
    }

    public String getScopeId() {
        return this.scopeId;
    }
    
    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public String getDurabilityClass() {
        return this.durabilityClass;
    }
    
    public void setDurabilityClass(String durabilityClass) {
        this.durabilityClass = durabilityClass;
    }

    public String getOrderingScope() {
        return this.orderingScope;
    }
    
    public void setOrderingScope(String orderingScope) {
        this.orderingScope = orderingScope;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }
    
    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getState() {
        return this.state;
    }
    
    public void setState(String state) {
        this.state = state;
    }

    public Integer getLastFrameSeq() {
        return this.lastFrameSeq;
    }
    
    public void setLastFrameSeq(Integer lastFrameSeq) {
        this.lastFrameSeq = lastFrameSeq;
    }

    public Integer getLastCheckpointSeq() {
        return this.lastCheckpointSeq;
    }
    
    public void setLastCheckpointSeq(Integer lastCheckpointSeq) {
        this.lastCheckpointSeq = lastCheckpointSeq;
    }

    public String getResultMessageId() {
        return this.resultMessageId;
    }
    
    public void setResultMessageId(String resultMessageId) {
        this.resultMessageId = resultMessageId;
    }

    public String getOpenedAt() {
        return this.openedAt;
    }
    
    public void setOpenedAt(String openedAt) {
        this.openedAt = openedAt;
    }

    public String getClosedAt() {
        return this.closedAt;
    }
    
    public void setClosedAt(String closedAt) {
        this.closedAt = closedAt;
    }

    public String getExpiresAt() {
        return this.expiresAt;
    }
    
    public void setExpiresAt(String expiresAt) {
        this.expiresAt = expiresAt;
    }
}
