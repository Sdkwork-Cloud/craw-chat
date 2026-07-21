package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ProviderBindingDriftPageData {
    private List<ProviderBindingDriftItem> items;
    private PageInfo pageInfo;

    public List<ProviderBindingDriftItem> getItems() {
        return this.items;
    }

    public void setItems(List<ProviderBindingDriftItem> items) {
        this.items = items;
    }

    public PageInfo getPageInfo() {
        return this.pageInfo;
    }

    public void setPageInfo(PageInfo pageInfo) {
        this.pageInfo = pageInfo;
    }
}
