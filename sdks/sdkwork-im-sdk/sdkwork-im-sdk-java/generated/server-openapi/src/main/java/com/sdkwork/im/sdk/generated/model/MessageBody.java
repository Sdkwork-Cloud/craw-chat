package com.sdkwork.im.sdk.generated.model;

import java.util.List;
import java.util.Map;

public class MessageBody {
    private String text;
    private List<ContentPart> parts;
    private MessageReplyReference replyTo;
    private Map<String, Object> renderHints;
    private String summary;
    private Map<String, Object> metadata;

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

    public Map<String, Object> getRenderHints() {
        return this.renderHints;
    }

    public void setRenderHints(Map<String, Object> renderHints) {
        this.renderHints = renderHints;
    }

    public String getSummary() {
        return this.summary;
    }

    public void setSummary(String summary) {
        this.summary = summary;
    }

    public Map<String, Object> getMetadata() {
        return this.metadata;
    }

    public void setMetadata(Map<String, Object> metadata) {
        this.metadata = metadata;
    }
}
