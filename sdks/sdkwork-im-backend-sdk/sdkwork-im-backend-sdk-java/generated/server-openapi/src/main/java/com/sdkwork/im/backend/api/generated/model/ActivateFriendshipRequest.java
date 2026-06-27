package com.sdkwork.im.backend.api.generated.model;


public class ActivateFriendshipRequest {
    private String directChatId;
    private String establishedAt;
    private String eventId;
    private String friendshipId;
    private String initiatorUserId;
    private String peerUserId;

    public String getDirectChatId() {
        return this.directChatId;
    }
    
    public void setDirectChatId(String directChatId) {
        this.directChatId = directChatId;
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

    public String getFriendshipId() {
        return this.friendshipId;
    }
    
    public void setFriendshipId(String friendshipId) {
        this.friendshipId = friendshipId;
    }

    public String getInitiatorUserId() {
        return this.initiatorUserId;
    }
    
    public void setInitiatorUserId(String initiatorUserId) {
        this.initiatorUserId = initiatorUserId;
    }

    public String getPeerUserId() {
        return this.peerUserId;
    }
    
    public void setPeerUserId(String peerUserId) {
        this.peerUserId = peerUserId;
    }
}
