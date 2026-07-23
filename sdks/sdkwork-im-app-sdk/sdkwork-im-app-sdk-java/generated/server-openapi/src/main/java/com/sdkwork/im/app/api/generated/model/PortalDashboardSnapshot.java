package com.sdkwork.im.app.api.generated.model;


public class PortalDashboardSnapshot {
    private PortalSnapshotMeta meta;
    private PortalDataAvailability availability;
    private PortalOperationalMetrics metrics;

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

    public PortalOperationalMetrics getMetrics() {
        return this.metrics;
    }

    public void setMetrics(PortalOperationalMetrics metrics) {
        this.metrics = metrics;
    }
}
