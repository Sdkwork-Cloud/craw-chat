package com.sdkwork.im.app.api.generated.model;


public class PortalGovernanceSnapshot {
    private PortalSnapshotMeta meta;
    private PortalDataAvailability availability;
    private String sampledEventCount;
    private PortalGovernanceRiskSample riskSample;

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

    public String getSampledEventCount() {
        return this.sampledEventCount;
    }

    public void setSampledEventCount(String sampledEventCount) {
        this.sampledEventCount = sampledEventCount;
    }

    public PortalGovernanceRiskSample getRiskSample() {
        return this.riskSample;
    }

    public void setRiskSample(PortalGovernanceRiskSample riskSample) {
        this.riskSample = riskSample;
    }
}
