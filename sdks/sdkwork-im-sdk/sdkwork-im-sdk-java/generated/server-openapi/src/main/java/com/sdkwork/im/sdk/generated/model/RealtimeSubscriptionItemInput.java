package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class RealtimeSubscriptionItemInput {
    private String scopeType;
    private String scopeId;
    private List<String> eventTypes;

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

    public List<String> getEventTypes() {
        return this.eventTypes;
    }

    public void setEventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
    }
}
