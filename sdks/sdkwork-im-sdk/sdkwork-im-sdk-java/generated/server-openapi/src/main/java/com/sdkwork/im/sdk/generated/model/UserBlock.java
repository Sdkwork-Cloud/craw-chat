package com.sdkwork.im.sdk.generated.model;


public class UserBlock {
    private String tenantId;
    private String blockId;
    private String blockerUserId;
    private String blockedUserId;
    private String scope;
    private String status;
    private String directChatId;
    private String expiresAt;
    private String createdAt;
    private String updatedAt;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getBlockId() {
        return this.blockId;
    }

    public void setBlockId(String blockId) {
        this.blockId = blockId;
    }

    public String getBlockerUserId() {
        return this.blockerUserId;
    }

    public void setBlockerUserId(String blockerUserId) {
        this.blockerUserId = blockerUserId;
    }

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

    public String getStatus() {
        return this.status;
    }

    public void setStatus(String status) {
        this.status = status;
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

    public String getCreatedAt() {
        return this.createdAt;
    }

    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }

    public String getUpdatedAt() {
        return this.updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }
}
