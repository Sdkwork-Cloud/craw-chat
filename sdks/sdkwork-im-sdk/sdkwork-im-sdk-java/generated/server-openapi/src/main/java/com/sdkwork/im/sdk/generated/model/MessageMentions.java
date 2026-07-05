package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class MessageMentions {
    private List<String> userIds;
    private List<String> scopes;

    public List<String> getUserIds() {
        return this.userIds;
    }

    public void setUserIds(List<String> userIds) {
        this.userIds = userIds;
    }

    public List<String> getScopes() {
        return this.scopes;
    }

    public void setScopes(List<String> scopes) {
        this.scopes = scopes;
    }
}
