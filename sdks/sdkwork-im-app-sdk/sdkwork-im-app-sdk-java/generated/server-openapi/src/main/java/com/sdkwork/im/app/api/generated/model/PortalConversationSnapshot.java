package com.sdkwork.im.app.api.generated.model;


public class PortalConversationSnapshot {
    private PortalSnapshotMeta meta;
    private PortalDataAvailability availability;
    private PortalConversationOperationalMetrics metrics;

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

    public PortalConversationOperationalMetrics getMetrics() {
        return this.metrics;
    }

    public void setMetrics(PortalConversationOperationalMetrics metrics) {
        this.metrics = metrics;
    }
}
