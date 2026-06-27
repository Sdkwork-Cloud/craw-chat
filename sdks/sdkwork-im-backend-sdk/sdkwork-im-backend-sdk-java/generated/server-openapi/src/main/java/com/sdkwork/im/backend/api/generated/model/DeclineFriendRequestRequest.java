package com.sdkwork.im.backend.api.generated.model;


public class DeclineFriendRequestRequest {
    private String declinedAt;
    private String declinedByUserId;
    private String eventId;

    public String getDeclinedAt() {
        return this.declinedAt;
    }
    
    public void setDeclinedAt(String declinedAt) {
        this.declinedAt = declinedAt;
    }

    public String getDeclinedByUserId() {
        return this.declinedByUserId;
    }
    
    public void setDeclinedByUserId(String declinedByUserId) {
        this.declinedByUserId = declinedByUserId;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }
}
