package com.sdkwork.im.backend.api.generated.model;


public class AcceptFriendRequestRequest {
    private String acceptedAt;
    private String acceptedByUserId;
    private String eventId;

    public String getAcceptedAt() {
        return this.acceptedAt;
    }

    public void setAcceptedAt(String acceptedAt) {
        this.acceptedAt = acceptedAt;
    }

    public String getAcceptedByUserId() {
        return this.acceptedByUserId;
    }

    public void setAcceptedByUserId(String acceptedByUserId) {
        this.acceptedByUserId = acceptedByUserId;
    }

    public String getEventId() {
        return this.eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }
}
