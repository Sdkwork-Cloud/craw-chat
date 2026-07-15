package com.sdkwork.im.app.api.generated.model;

import java.util.Map;

public class AppendAgentResponseDeltaRequest {
    private String frameSeq;
    private String frameType;
    private String schemaRef;
    private String encoding;
    private String payload;
    private Map<String, String> attributes;

    public String getFrameSeq() {
        return this.frameSeq;
    }

    public void setFrameSeq(String frameSeq) {
        this.frameSeq = frameSeq;
    }

    public String getFrameType() {
        return this.frameType;
    }

    public void setFrameType(String frameType) {
        this.frameType = frameType;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }

    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getEncoding() {
        return this.encoding;
    }

    public void setEncoding(String encoding) {
        this.encoding = encoding;
    }

    public String getPayload() {
        return this.payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public Map<String, String> getAttributes() {
        return this.attributes;
    }

    public void setAttributes(Map<String, String> attributes) {
        this.attributes = attributes;
    }
}
