package com.sdkwork.im.sdk.generated.model;


public class BlockUserRequest {
    private String blockedUserId;
    private String scope;
    private String directChatId;
    private String expiresAt;

    public String getBlockedUserId() {
        return this.blockedUserId;
    }

    public void setBlockedUserId(String blockedUserId) {
        this.blockedUserId = blockedUserId;
    }

    public String getScope() {
        return this.scope;
    }

    public void setScope(String scope) {
        this.scope = scope;
    }

    public String getDirectChatId() {
        return this.directChatId;
    }

    public void setDirectChatId(String directChatId) {
        this.directChatId = directChatId;
    }

    public String getExpiresAt() {
        return this.expiresAt;
    }

    public void setExpiresAt(String expiresAt) {
        this.expiresAt = expiresAt;
    }
}
