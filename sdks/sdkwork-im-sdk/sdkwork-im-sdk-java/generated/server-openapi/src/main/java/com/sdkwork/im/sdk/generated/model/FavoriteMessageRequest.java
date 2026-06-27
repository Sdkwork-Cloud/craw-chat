package com.sdkwork.im.sdk.generated.model;


public class FavoriteMessageRequest {
    private String conversationId;
    private String favoriteType;
    private String title;
    private String contentPreview;
    private String sourceDisplayName;

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getFavoriteType() {
        return this.favoriteType;
    }

    public void setFavoriteType(String favoriteType) {
        this.favoriteType = favoriteType;
    }

    public String getTitle() {
        return this.title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getContentPreview() {
        return this.contentPreview;
    }

    public void setContentPreview(String contentPreview) {
        this.contentPreview = contentPreview;
    }

    public String getSourceDisplayName() {
        return this.sourceDisplayName;
    }

    public void setSourceDisplayName(String sourceDisplayName) {
        this.sourceDisplayName = sourceDisplayName;
    }
}
