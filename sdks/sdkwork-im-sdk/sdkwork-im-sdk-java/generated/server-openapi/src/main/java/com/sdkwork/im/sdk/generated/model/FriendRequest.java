package com.sdkwork.im.sdk.generated.model;


public class FriendRequest {
    private String tenantId;
    private String friendRequestId;
    private String requesterUserId;
    private String targetUserId;
    private String status;
    private String requestMessage;
    private String expiredAt;
    private String createdAt;
    private String updatedAt;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getFriendRequestId() {
        return this.friendRequestId;
    }

    public void setFriendRequestId(String friendRequestId) {
        this.friendRequestId = friendRequestId;
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

    public String getStatus() {
        return this.status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public String getRequestMessage() {
        return this.requestMessage;
    }

    public void setRequestMessage(String requestMessage) {
        this.requestMessage = requestMessage;
    }

    public String getExpiredAt() {
        return this.expiredAt;
    }

    public void setExpiredAt(String expiredAt) {
        this.expiredAt = expiredAt;
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
