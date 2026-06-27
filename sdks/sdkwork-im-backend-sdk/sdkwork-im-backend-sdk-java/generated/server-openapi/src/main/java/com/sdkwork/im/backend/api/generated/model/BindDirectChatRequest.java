package com.sdkwork.im.backend.api.generated.model;


public class BindDirectChatRequest {
    private String boundAt;
    private String conversationId;
    private String directChatId;
    private String eventId;
    private String leftActorId;
    private String rightActorId;

    public String getBoundAt() {
        return this.boundAt;
    }
    
    public void setBoundAt(String boundAt) {
        this.boundAt = boundAt;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getDirectChatId() {
        return this.directChatId;
    }
    
    public void setDirectChatId(String directChatId) {
        this.directChatId = directChatId;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getLeftActorId() {
        return this.leftActorId;
    }
    
    public void setLeftActorId(String leftActorId) {
        this.leftActorId = leftActorId;
    }

    public String getRightActorId() {
        return this.rightActorId;
    }
    
    public void setRightActorId(String rightActorId) {
        this.rightActorId = rightActorId;
    }
}
