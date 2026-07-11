package com.sdkwork.im.sdk.generated.model;

import java.util.List;
import java.util.Map;

public class EditMessageRequest {
    private String text;
    private List<ContentPart> parts;
    private MessageReplyReference replyTo;
    private String summary;
    private Map<String, Object> renderHints;
    private String idempotencyKey;

    public String getText() {
        return this.text;
    }

    public void setText(String text) {
        this.text = text;
    }

    public List<ContentPart> getParts() {
        return this.parts;
    }

    public void setParts(List<ContentPart> parts) {
        this.parts = parts;
    }

    public MessageReplyReference getReplyTo() {
        return this.replyTo;
    }

    public void setReplyTo(MessageReplyReference replyTo) {
        this.replyTo = replyTo;
    }

    public String getSummary() {
        return this.summary;
    }

    public void setSummary(String summary) {
        this.summary = summary;
    }

    public Map<String, Object> getRenderHints() {
        return this.renderHints;
    }

    public void setRenderHints(Map<String, Object> renderHints) {
        this.renderHints = renderHints;
    }

    public String getIdempotencyKey() {
        return this.idempotencyKey;
    }

    public void setIdempotencyKey(String idempotencyKey) {
        this.idempotencyKey = idempotencyKey;
    }
}
