package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class LagPageData {
    private List<LagItem> items;
    private PageInfo pageInfo;

    public List<LagItem> getItems() {
        return this.items;
    }

    public void setItems(List<LagItem> items) {
        this.items = items;
    }

    public PageInfo getPageInfo() {
        return this.pageInfo;
    }

    public void setPageInfo(PageInfo pageInfo) {
        this.pageInfo = pageInfo;
    }
}
