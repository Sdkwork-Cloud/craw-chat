package com.sdkwork.im.backend.api.generated.model;


public class LagItem {
    private String component;
    private String scopeId;
    private String currentOffset;
    private String committedOffset;
    private String lag;

    public String getComponent() {
        return this.component;
    }

    public void setComponent(String component) {
        this.component = component;
    }

    public String getScopeId() {
        return this.scopeId;
    }

    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public String getCurrentOffset() {
        return this.currentOffset;
    }

    public void setCurrentOffset(String currentOffset) {
        this.currentOffset = currentOffset;
    }

    public String getCommittedOffset() {
        return this.committedOffset;
    }

    public void setCommittedOffset(String committedOffset) {
        this.committedOffset = committedOffset;
    }

    public String getLag() {
        return this.lag;
    }

    public void setLag(String lag) {
        this.lag = lag;
    }
}
