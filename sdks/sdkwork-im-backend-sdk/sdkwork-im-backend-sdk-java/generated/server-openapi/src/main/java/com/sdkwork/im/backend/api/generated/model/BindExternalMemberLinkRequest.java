package com.sdkwork.im.backend.api.generated.model;


public class BindExternalMemberLinkRequest {
    private String connectionId;
    private String eventId;
    private String externalDisplayName;
    private String externalMemberId;
    private String linkId;
    private String linkedAt;
    private String localActorId;
    private String localActorKind;

    public String getConnectionId() {
        return this.connectionId;
    }
    
    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getExternalDisplayName() {
        return this.externalDisplayName;
    }
    
    public void setExternalDisplayName(String externalDisplayName) {
        this.externalDisplayName = externalDisplayName;
    }

    public String getExternalMemberId() {
        return this.externalMemberId;
    }
    
    public void setExternalMemberId(String externalMemberId) {
        this.externalMemberId = externalMemberId;
    }

    public String getLinkId() {
        return this.linkId;
    }
    
    public void setLinkId(String linkId) {
        this.linkId = linkId;
    }

    public String getLinkedAt() {
        return this.linkedAt;
    }
    
    public void setLinkedAt(String linkedAt) {
        this.linkedAt = linkedAt;
    }

    public String getLocalActorId() {
        return this.localActorId;
    }
    
    public void setLocalActorId(String localActorId) {
        this.localActorId = localActorId;
    }

    public String getLocalActorKind() {
        return this.localActorKind;
    }
    
    public void setLocalActorKind(String localActorKind) {
        this.localActorKind = localActorKind;
    }
}
