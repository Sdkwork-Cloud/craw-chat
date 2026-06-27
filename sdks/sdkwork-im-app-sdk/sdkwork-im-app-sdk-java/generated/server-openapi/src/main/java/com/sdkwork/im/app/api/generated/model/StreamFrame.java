package com.sdkwork.im.app.api.generated.model;

import java.util.Map;

public class StreamFrame {
    private String tenantId;
    private String streamId;
    private String streamType;
    private String scopeKind;
    private String scopeId;
    private Integer frameSeq;
    private String frameType;
    private String schemaRef;
    private String encoding;
    private String payload;
    private Sender sender;
    private Map<String, String> attributes;
    private String occurredAt;

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

    public Integer getFrameSeq() {
        return this.frameSeq;
    }
    
    public void setFrameSeq(Integer frameSeq) {
        this.frameSeq = frameSeq;
    }

    public String getFrameType() {
        return this.frameType;
    }
    
    public void setFrameType(String frameType) {
        this.frameType = frameType;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }
    
    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getEncoding() {
        return this.encoding;
    }
    
    public void setEncoding(String encoding) {
        this.encoding = encoding;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }

    public Sender getSender() {
        return this.sender;
    }
    
    public void setSender(Sender sender) {
        this.sender = sender;
    }

    public Map<String, String> getAttributes() {
        return this.attributes;
    }
    
    public void setAttributes(Map<String, String> attributes) {
        this.attributes = attributes;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }
    
    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }
}
