package com.sdkwork.im.backend.api.generated.model;


public class BlockUserRequest {
    private String blockId;
    private String blockedUserId;
    private String blockerUserId;
    private String directChatId;
    private String effectiveAt;
    private String eventId;
    private String expiresAt;
    private String scope;

    public String getBlockId() {
        return this.blockId;
    }

    public void setBlockId(String blockId) {
        this.blockId = blockId;
    }

    public String getBlockedUserId() {
        return this.blockedUserId;
    }

    public void setBlockedUserId(String blockedUserId) {
        this.blockedUserId = blockedUserId;
    }

    public String getBlockerUserId() {
        return this.blockerUserId;
    }

    public void setBlockerUserId(String blockerUserId) {
        this.blockerUserId = blockerUserId;
    }

    public String getDirectChatId() {
        return this.directChatId;
    }

    public void setDirectChatId(String directChatId) {
        this.directChatId = directChatId;
    }

    public String getEffectiveAt() {
        return this.effectiveAt;
    }

    public void setEffectiveAt(String effectiveAt) {
        this.effectiveAt = effectiveAt;
    }

    public String getEventId() {
        return this.eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getExpiresAt() {
        return this.expiresAt;
    }

    public void setExpiresAt(String expiresAt) {
        this.expiresAt = expiresAt;
    }

    public String getScope() {
        return this.scope;
    }

    public void setScope(String scope) {
        this.scope = scope;
    }
}
