package com.sdkwork.im.backend.api.generated.model;


public class SubmitFriendRequestRequest {
    private String eventId;
    private String requestId;
    private String requestMessage;
    private String requestedAt;
    private String requesterUserId;
    private String targetUserId;

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getRequestId() {
        return this.requestId;
    }
    
    public void setRequestId(String requestId) {
        this.requestId = requestId;
    }

    public String getRequestMessage() {
        return this.requestMessage;
    }
    
    public void setRequestMessage(String requestMessage) {
        this.requestMessage = requestMessage;
    }

    public String getRequestedAt() {
        return this.requestedAt;
    }
    
    public void setRequestedAt(String requestedAt) {
        this.requestedAt = requestedAt;
    }

    public String getRequesterUserId() {
        return this.requesterUserId;
    }
    
    public void setRequesterUserId(String requesterUserId) {
        this.requesterUserId = requesterUserId;
    }

    public String getTargetUserId() {
        return this.targetUserId;
    }
    
    public void setTargetUserId(String targetUserId) {
        this.targetUserId = targetUserId;
    }
}
