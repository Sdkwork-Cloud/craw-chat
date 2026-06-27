package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class ListMembersResponse {
    private List<ConversationMember> items;
    private String nextCursor;
    private Boolean hasMore;

    public List<ConversationMember> getItems() {
        return this.items;
    }

    public void setItems(List<ConversationMember> items) {
        this.items = items;
    }

    public String getNextCursor() {
        return this.nextCursor;
    }

    public void setNextCursor(String nextCursor) {
        this.nextCursor = nextCursor;
    }

    public Boolean getHasMore() {
        return this.hasMore;
    }

    public void setHasMore(Boolean hasMore) {
        this.hasMore = hasMore;
    }
}
