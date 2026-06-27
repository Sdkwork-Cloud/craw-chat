package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class EditMessageRequest {
    private String text;
    private List<ContentPart> parts;
    private MessageReplyReference replyTo;

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
}
