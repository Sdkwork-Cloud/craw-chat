package com.sdkwork.im.backend.api.generated.model;


public class ApplySharedChannelPolicyRequest {
    private String appliedAt;
    private String channelId;
    private String connectionId;
    private String conversationId;
    private String eventId;
    private String historyVisibility;
    private String policyId;
    private Integer policyVersion;

    public String getAppliedAt() {
        return this.appliedAt;
    }
    
    public void setAppliedAt(String appliedAt) {
        this.appliedAt = appliedAt;
    }

    public String getChannelId() {
        return this.channelId;
    }
    
    public void setChannelId(String channelId) {
        this.channelId = channelId;
    }

    public String getConnectionId() {
        return this.connectionId;
    }
    
    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getHistoryVisibility() {
        return this.historyVisibility;
    }
    
    public void setHistoryVisibility(String historyVisibility) {
        this.historyVisibility = historyVisibility;
    }

    public String getPolicyId() {
        return this.policyId;
    }
    
    public void setPolicyId(String policyId) {
        this.policyId = policyId;
    }

    public Integer getPolicyVersion() {
        return this.policyVersion;
    }
    
    public void setPolicyVersion(Integer policyVersion) {
        this.policyVersion = policyVersion;
    }
}
