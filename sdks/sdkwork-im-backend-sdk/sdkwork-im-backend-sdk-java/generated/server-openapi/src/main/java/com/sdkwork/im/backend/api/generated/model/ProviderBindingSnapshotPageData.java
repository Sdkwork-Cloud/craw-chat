package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ProviderBindingSnapshotPageData {
    private List<ProviderBindingSnapshot> items;
    private PageInfo pageInfo;

    public List<ProviderBindingSnapshot> getItems() {
        return this.items;
    }

    public void setItems(List<ProviderBindingSnapshot> items) {
        this.items = items;
    }

    public PageInfo getPageInfo() {
        return this.pageInfo;
    }

    public void setPageInfo(PageInfo pageInfo) {
        this.pageInfo = pageInfo;
    }
}
