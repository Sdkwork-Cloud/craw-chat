package com.sdkwork.im.sdk.generated.model;


public class PostRtcSignalRequest {
    private String signalType;
    private String schemaRef;
    private String payload;
    private String signalingStreamId;

    public String getSignalType() {
        return this.signalType;
    }

    public void setSignalType(String signalType) {
        this.signalType = signalType;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }

    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getPayload() {
        return this.payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public String getSignalingStreamId() {
        return this.signalingStreamId;
    }

    public void setSignalingStreamId(String signalingStreamId) {
        this.signalingStreamId = signalingStreamId;
    }
}
