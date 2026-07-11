package com.sdkwork.im.sdk.generated.model;


public class PostMessageResult {
    private String messageId;
    private Integer messageSeq;
    private String eventId;
    private String requestKey;
    private String deliveryStatus;
    private String proofVersion;

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

    public String getEventId() {
        return this.eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getRequestKey() {
        return this.requestKey;
    }

    public void setRequestKey(String requestKey) {
        this.requestKey = requestKey;
    }

    public String getDeliveryStatus() {
        return this.deliveryStatus;
    }

    public void setDeliveryStatus(String deliveryStatus) {
        this.deliveryStatus = deliveryStatus;
    }

    public String getProofVersion() {
        return this.proofVersion;
    }

    public void setProofVersion(String proofVersion) {
        this.proofVersion = proofVersion;
    }
}
