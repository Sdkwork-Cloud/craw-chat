package com.sdkwork.im.backend.api.generated.model;


public class QuotaProfileResponse {
    private Integer maxConcurrentSessionsPerTenant;
    private Integer maxInflightMessages;
    private Integer maxPayloadBytes;
    private Integer maxSubscriptionsPerSession;
    private String profileId;

    public Integer getMaxConcurrentSessionsPerTenant() {
        return this.maxConcurrentSessionsPerTenant;
    }
    
    public void setMaxConcurrentSessionsPerTenant(Integer maxConcurrentSessionsPerTenant) {
        this.maxConcurrentSessionsPerTenant = maxConcurrentSessionsPerTenant;
    }

    public Integer getMaxInflightMessages() {
        return this.maxInflightMessages;
    }
    
    public void setMaxInflightMessages(Integer maxInflightMessages) {
        this.maxInflightMessages = maxInflightMessages;
    }

    public Integer getMaxPayloadBytes() {
        return this.maxPayloadBytes;
    }
    
    public void setMaxPayloadBytes(Integer maxPayloadBytes) {
        this.maxPayloadBytes = maxPayloadBytes;
    }

    public Integer getMaxSubscriptionsPerSession() {
        return this.maxSubscriptionsPerSession;
    }
    
    public void setMaxSubscriptionsPerSession(Integer maxSubscriptionsPerSession) {
        this.maxSubscriptionsPerSession = maxSubscriptionsPerSession;
    }

    public String getProfileId() {
        return this.profileId;
    }
    
    public void setProfileId(String profileId) {
        this.profileId = profileId;
    }
}
