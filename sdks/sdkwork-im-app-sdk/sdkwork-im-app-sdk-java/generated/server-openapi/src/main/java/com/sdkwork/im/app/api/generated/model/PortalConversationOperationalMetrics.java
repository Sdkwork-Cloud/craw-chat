package com.sdkwork.im.app.api.generated.model;


public class PortalConversationOperationalMetrics {
    private String laggingScopeCount;
    private String maxOperationalLag;
    private String pendingOutboxEventCount;
    private String failedOutboxAttemptCount;

    public String getLaggingScopeCount() {
        return this.laggingScopeCount;
    }

    public void setLaggingScopeCount(String laggingScopeCount) {
        this.laggingScopeCount = laggingScopeCount;
    }

    public String getMaxOperationalLag() {
        return this.maxOperationalLag;
    }

    public void setMaxOperationalLag(String maxOperationalLag) {
        this.maxOperationalLag = maxOperationalLag;
    }

    public String getPendingOutboxEventCount() {
        return this.pendingOutboxEventCount;
    }

    public void setPendingOutboxEventCount(String pendingOutboxEventCount) {
        this.pendingOutboxEventCount = pendingOutboxEventCount;
    }

    public String getFailedOutboxAttemptCount() {
        return this.failedOutboxAttemptCount;
    }

    public void setFailedOutboxAttemptCount(String failedOutboxAttemptCount) {
        this.failedOutboxAttemptCount = failedOutboxAttemptCount;
    }
}
