package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class TimelineResponse {
    private List<TimelineViewEntry> items;
    private Integer nextAfterSeq;
    private Boolean hasMore;

    public List<TimelineViewEntry> getItems() {
        return this.items;
    }

    public void setItems(List<TimelineViewEntry> items) {
        this.items = items;
    }

    public Integer getNextAfterSeq() {
        return this.nextAfterSeq;
    }

    public void setNextAfterSeq(Integer nextAfterSeq) {
        this.nextAfterSeq = nextAfterSeq;
    }

    public Boolean getHasMore() {
        return this.hasMore;
    }

    public void setHasMore(Boolean hasMore) {
        this.hasMore = hasMore;
    }
}
