package com.sdkwork.im.app.api.generated.model;

import java.util.List;

public class PortalAccessSnapshot {
    private PortalSnapshotMeta meta;
    private PortalDataAvailability availability;
    private String tenantId;
    private String principalId;
    private List<PortalAuditRecordView> recentItems;
    private Boolean hasMore;

    public PortalSnapshotMeta getMeta() {
        return this.meta;
    }

    public void setMeta(PortalSnapshotMeta meta) {
        this.meta = meta;
    }

    public PortalDataAvailability getAvailability() {
        return this.availability;
    }

    public void setAvailability(PortalDataAvailability availability) {
        this.availability = availability;
    }

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getPrincipalId() {
        return this.principalId;
    }

    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public List<PortalAuditRecordView> getRecentItems() {
        return this.recentItems;
    }

    public void setRecentItems(List<PortalAuditRecordView> recentItems) {
        this.recentItems = recentItems;
    }

    public Boolean getHasMore() {
        return this.hasMore;
    }

    public void setHasMore(Boolean hasMore) {
        this.hasMore = hasMore;
    }
}
