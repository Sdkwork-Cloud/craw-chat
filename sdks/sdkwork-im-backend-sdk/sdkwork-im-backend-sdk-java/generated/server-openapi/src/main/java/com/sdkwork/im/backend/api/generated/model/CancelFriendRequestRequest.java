package com.sdkwork.im.backend.api.generated.model;


public class CancelFriendRequestRequest {
    private String canceledAt;
    private String canceledByUserId;
    private String eventId;

    public String getCanceledAt() {
        return this.canceledAt;
    }
    
    public void setCanceledAt(String canceledAt) {
        this.canceledAt = canceledAt;
    }

    public String getCanceledByUserId() {
        return this.canceledByUserId;
    }
    
    public void setCanceledByUserId(String canceledByUserId) {
        this.canceledByUserId = canceledByUserId;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }
}
