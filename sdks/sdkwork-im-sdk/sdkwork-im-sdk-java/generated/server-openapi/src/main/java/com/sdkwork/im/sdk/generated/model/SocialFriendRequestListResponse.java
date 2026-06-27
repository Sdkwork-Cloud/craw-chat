package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class SocialFriendRequestListResponse {
    private List<FriendRequest> items;
    private String nextCursor;

    public List<FriendRequest> getItems() {
        return this.items;
    }

    public void setItems(List<FriendRequest> items) {
        this.items = items;
    }

    public String getNextCursor() {
        return this.nextCursor;
    }

    public void setNextCursor(String nextCursor) {
        this.nextCursor = nextCursor;
    }
}
