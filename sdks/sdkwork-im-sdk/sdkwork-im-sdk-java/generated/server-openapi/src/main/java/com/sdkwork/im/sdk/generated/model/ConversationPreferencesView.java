package com.sdkwork.im.sdk.generated.model;


public class ConversationPreferencesView {
    private String tenantId;
    private String conversationId;
    private String principalKind;
    private String principalId;
    private Boolean isPinned;
    private Boolean isMuted;
    private Boolean isMarkedUnread;
    private Boolean isHidden;
    private String updatedAt;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getPrincipalKind() {
        return this.principalKind;
    }

    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

    public String getPrincipalId() {
        return this.principalId;
    }

    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public Boolean getIsPinned() {
        return this.isPinned;
    }

    public void setIsPinned(Boolean isPinned) {
        this.isPinned = isPinned;
    }

    public Boolean getIsMuted() {
        return this.isMuted;
    }

    public void setIsMuted(Boolean isMuted) {
        this.isMuted = isMuted;
    }

    public Boolean getIsMarkedUnread() {
        return this.isMarkedUnread;
    }

    public void setIsMarkedUnread(Boolean isMarkedUnread) {
        this.isMarkedUnread = isMarkedUnread;
    }

    public Boolean getIsHidden() {
        return this.isHidden;
    }

    public void setIsHidden(Boolean isHidden) {
        this.isHidden = isHidden;
    }

    public String getUpdatedAt() {
        return this.updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }
}
