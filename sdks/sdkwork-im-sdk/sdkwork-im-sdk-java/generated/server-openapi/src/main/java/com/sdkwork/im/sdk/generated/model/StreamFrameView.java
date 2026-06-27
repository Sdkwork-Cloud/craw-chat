package com.sdkwork.im.sdk.generated.model;


public class StreamFrameView {
    private String streamId;
    private Integer frameSeq;
    private String payload;
    private String createdAt;

    public String getStreamId() {
        return this.streamId;
    }

    public void setStreamId(String streamId) {
        this.streamId = streamId;
    }

    public Integer getFrameSeq() {
        return this.frameSeq;
    }

    public void setFrameSeq(Integer frameSeq) {
        this.frameSeq = frameSeq;
    }

    public String getPayload() {
        return this.payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public String getCreatedAt() {
        return this.createdAt;
    }

    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }
}
