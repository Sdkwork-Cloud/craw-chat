package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class MessageInteractionSummaryView {
    private String tenantId;
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private Integer totalReactionCount;
    private List<MessageReactionCountView> reactionCounts;
    private MessagePinView pin;

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

    public Integer getMessageSeq() {
        return this.messageSeq;
    }

    public void setMessageSeq(Integer messageSeq) {
        this.messageSeq = messageSeq;
    }

    public Integer getTotalReactionCount() {
        return this.totalReactionCount;
    }

    public void setTotalReactionCount(Integer totalReactionCount) {
        this.totalReactionCount = totalReactionCount;
    }

    public List<MessageReactionCountView> getReactionCounts() {
        return this.reactionCounts;
    }

    public void setReactionCounts(List<MessageReactionCountView> reactionCounts) {
        this.reactionCounts = reactionCounts;
    }

    public MessagePinView getPin() {
        return this.pin;
    }

    public void setPin(MessagePinView pin) {
        this.pin = pin;
    }
}
