package com.sdkwork.im.app.api.generated.model;


public class PortalRealtimeMetrics {
    private String clientRouteWindowCount;
    private String pendingEventCount;
    private String maxClientRouteWindowEventCount;
    private String clientRouteWindowCapacity;
    private Integer maxClientRouteWindowUsagePermille;
    private String capacityTrimmedEventCount;
    private String oldestPendingOccurredAt;

    public String getClientRouteWindowCount() {
        return this.clientRouteWindowCount;
    }

    public void setClientRouteWindowCount(String clientRouteWindowCount) {
        this.clientRouteWindowCount = clientRouteWindowCount;
    }

    public String getPendingEventCount() {
        return this.pendingEventCount;
    }

    public void setPendingEventCount(String pendingEventCount) {
        this.pendingEventCount = pendingEventCount;
    }

    public String getMaxClientRouteWindowEventCount() {
        return this.maxClientRouteWindowEventCount;
    }

    public void setMaxClientRouteWindowEventCount(String maxClientRouteWindowEventCount) {
        this.maxClientRouteWindowEventCount = maxClientRouteWindowEventCount;
    }

    public String getClientRouteWindowCapacity() {
        return this.clientRouteWindowCapacity;
    }

    public void setClientRouteWindowCapacity(String clientRouteWindowCapacity) {
        this.clientRouteWindowCapacity = clientRouteWindowCapacity;
    }

    public Integer getMaxClientRouteWindowUsagePermille() {
        return this.maxClientRouteWindowUsagePermille;
    }

    public void setMaxClientRouteWindowUsagePermille(Integer maxClientRouteWindowUsagePermille) {
        this.maxClientRouteWindowUsagePermille = maxClientRouteWindowUsagePermille;
    }

    public String getCapacityTrimmedEventCount() {
        return this.capacityTrimmedEventCount;
    }

    public void setCapacityTrimmedEventCount(String capacityTrimmedEventCount) {
        this.capacityTrimmedEventCount = capacityTrimmedEventCount;
    }

    public String getOldestPendingOccurredAt() {
        return this.oldestPendingOccurredAt;
    }

    public void setOldestPendingOccurredAt(String oldestPendingOccurredAt) {
        this.oldestPendingOccurredAt = oldestPendingOccurredAt;
    }
}
