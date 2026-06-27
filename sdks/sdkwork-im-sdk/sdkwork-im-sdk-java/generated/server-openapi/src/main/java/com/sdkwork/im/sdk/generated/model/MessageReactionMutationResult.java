package com.sdkwork.im.sdk.generated.model;


public class MessageReactionMutationResult {
    private String tenantId;
    private String conversationId;
    private String messageId;
    private String reactionKey;
    private Integer count;
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

    public String getMessageId() {
        return this.messageId;
    }

    public void setMessageId(String messageId) {
        this.messageId = messageId;
    }

    public String getReactionKey() {
        return this.reactionKey;
    }

    public void setReactionKey(String reactionKey) {
        this.reactionKey = reactionKey;
    }

    public Integer getCount() {
        return this.count;
    }

    public void setCount(Integer count) {
        this.count = count;
    }

    public String getUpdatedAt() {
        return this.updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }
}
