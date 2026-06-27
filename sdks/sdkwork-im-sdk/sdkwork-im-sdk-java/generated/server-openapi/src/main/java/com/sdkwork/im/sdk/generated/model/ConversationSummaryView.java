package com.sdkwork.im.sdk.generated.model;


public class ConversationSummaryView {
    private String tenantId;
    private String conversationId;
    private Integer messageCount;
    private Integer lastMessageSeq;
    private String lastSummary;
    private String lastMessageAt;

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

    public Integer getMessageCount() {
        return this.messageCount;
    }

    public void setMessageCount(Integer messageCount) {
        this.messageCount = messageCount;
    }

    public Integer getLastMessageSeq() {
        return this.lastMessageSeq;
    }

    public void setLastMessageSeq(Integer lastMessageSeq) {
        this.lastMessageSeq = lastMessageSeq;
    }

    public String getLastSummary() {
        return this.lastSummary;
    }

    public void setLastSummary(String lastSummary) {
        this.lastSummary = lastSummary;
    }

    public String getLastMessageAt() {
        return this.lastMessageAt;
    }

    public void setLastMessageAt(String lastMessageAt) {
        this.lastMessageAt = lastMessageAt;
    }
}
