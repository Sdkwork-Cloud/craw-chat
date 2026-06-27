package com.sdkwork.im.backend.api.generated.model;


public class RemoveFriendshipRequest {
    private String eventId;
    private String removedAt;
    private String removedByUserId;

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getRemovedAt() {
        return this.removedAt;
    }
    
    public void setRemovedAt(String removedAt) {
        this.removedAt = removedAt;
    }

    public String getRemovedByUserId() {
        return this.removedByUserId;
    }
    
    public void setRemovedByUserId(String removedByUserId) {
        this.removedByUserId = removedByUserId;
    }
}
