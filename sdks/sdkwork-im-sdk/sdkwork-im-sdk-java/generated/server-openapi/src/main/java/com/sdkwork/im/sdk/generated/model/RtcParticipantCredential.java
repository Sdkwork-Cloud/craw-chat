package com.sdkwork.im.sdk.generated.model;


public class RtcParticipantCredential {
    private String tenantId;
    private String rtcSessionId;
    private String participantId;
    private String credential;
    private String expiresAt;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getRtcSessionId() {
        return this.rtcSessionId;
    }

    public void setRtcSessionId(String rtcSessionId) {
        this.rtcSessionId = rtcSessionId;
    }

    public String getParticipantId() {
        return this.participantId;
    }

    public void setParticipantId(String participantId) {
        this.participantId = participantId;
    }

    public String getCredential() {
        return this.credential;
    }

    public void setCredential(String credential) {
        this.credential = credential;
    }

    public String getExpiresAt() {
        return this.expiresAt;
    }

    public void setExpiresAt(String expiresAt) {
        this.expiresAt = expiresAt;
    }
}
