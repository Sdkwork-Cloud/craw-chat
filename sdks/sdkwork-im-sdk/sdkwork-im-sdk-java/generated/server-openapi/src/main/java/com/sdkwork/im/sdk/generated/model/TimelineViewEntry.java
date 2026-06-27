package com.sdkwork.im.sdk.generated.model;


public class TimelineViewEntry {
    private String tenantId;
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private String summary;
    private Sender sender;
    private MessageBody body;
    private String messageType;
    private String deliveryMode;
    private String clientMsgId;
    private String streamSessionId;
    private String rtcSessionId;
    private String occurredAt;
    private String committedAt;

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

    public String getSummary() {
        return this.summary;
    }

    public void setSummary(String summary) {
        this.summary = summary;
    }

    public Sender getSender() {
        return this.sender;
    }

    public void setSender(Sender sender) {
        this.sender = sender;
    }

    public MessageBody getBody() {
        return this.body;
    }

    public void setBody(MessageBody body) {
        this.body = body;
    }

    public String getMessageType() {
        return this.messageType;
    }

    public void setMessageType(String messageType) {
        this.messageType = messageType;
    }

    public String getDeliveryMode() {
        return this.deliveryMode;
    }

    public void setDeliveryMode(String deliveryMode) {
        this.deliveryMode = deliveryMode;
    }

    public String getClientMsgId() {
        return this.clientMsgId;
    }

    public void setClientMsgId(String clientMsgId) {
        this.clientMsgId = clientMsgId;
    }

    public String getStreamSessionId() {
        return this.streamSessionId;
    }

    public void setStreamSessionId(String streamSessionId) {
        this.streamSessionId = streamSessionId;
    }

    public String getRtcSessionId() {
        return this.rtcSessionId;
    }

    public void setRtcSessionId(String rtcSessionId) {
        this.rtcSessionId = rtcSessionId;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }

    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }

    public String getCommittedAt() {
        return this.committedAt;
    }

    public void setCommittedAt(String committedAt) {
        this.committedAt = committedAt;
    }
}
