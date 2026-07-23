package com.sdkwork.im.app.api.generated.model;


public class PortalRealtimeSnapshot {
    private PortalSnapshotMeta meta;
    private PortalDataAvailability availability;
    private PortalRealtimeMetrics metrics;

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

    public PortalRealtimeMetrics getMetrics() {
        return this.metrics;
    }

    public void setMetrics(PortalRealtimeMetrics metrics) {
        this.metrics = metrics;
    }
}
