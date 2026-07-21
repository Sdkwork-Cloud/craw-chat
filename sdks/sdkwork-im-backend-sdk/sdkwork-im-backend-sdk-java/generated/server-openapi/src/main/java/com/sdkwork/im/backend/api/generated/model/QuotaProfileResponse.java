package com.sdkwork.im.backend.api.generated.model;


public class QuotaProfileResponse {
    private String maxConcurrentSessionsPerTenant;
    private String maxInflightMessages;
    private String maxPayloadBytes;
    private String maxSubscriptionsPerSession;
    private String profileId;

    public String getMaxConcurrentSessionsPerTenant() {
        return this.maxConcurrentSessionsPerTenant;
    }

    public void setMaxConcurrentSessionsPerTenant(String maxConcurrentSessionsPerTenant) {
        this.maxConcurrentSessionsPerTenant = maxConcurrentSessionsPerTenant;
    }

    public String getMaxInflightMessages() {
        return this.maxInflightMessages;
    }

    public void setMaxInflightMessages(String maxInflightMessages) {
        this.maxInflightMessages = maxInflightMessages;
    }

    public String getMaxPayloadBytes() {
        return this.maxPayloadBytes;
    }

    public void setMaxPayloadBytes(String maxPayloadBytes) {
        this.maxPayloadBytes = maxPayloadBytes;
    }

    public String getMaxSubscriptionsPerSession() {
        return this.maxSubscriptionsPerSession;
    }

    public void setMaxSubscriptionsPerSession(String maxSubscriptionsPerSession) {
        this.maxSubscriptionsPerSession = maxSubscriptionsPerSession;
    }

    public String getProfileId() {
        return this.profileId;
    }

    public void setProfileId(String profileId) {
        this.profileId = profileId;
    }
}
