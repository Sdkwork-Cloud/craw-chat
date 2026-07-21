package com.sdkwork.im.backend.api.generated.model;


public class EstablishExternalConnectionRequest {
    private String connectionId;
    private String connectionKind;
    private String establishedAt;
    private String eventId;
    private String externalOrgName;
    private String externalTenantId;

    public String getConnectionId() {
        return this.connectionId;
    }

    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }

    public String getConnectionKind() {
        return this.connectionKind;
    }

    public void setConnectionKind(String connectionKind) {
        this.connectionKind = connectionKind;
    }

    public String getEstablishedAt() {
        return this.establishedAt;
    }

    public void setEstablishedAt(String establishedAt) {
        this.establishedAt = establishedAt;
    }

    public String getEventId() {
        return this.eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getExternalOrgName() {
        return this.externalOrgName;
    }

    public void setExternalOrgName(String externalOrgName) {
        this.externalOrgName = externalOrgName;
    }

    public String getExternalTenantId() {
        return this.externalTenantId;
    }

    public void setExternalTenantId(String externalTenantId) {
        this.externalTenantId = externalTenantId;
    }
}
