package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class SocialSharedChannelSyncPendingTargetedTakeoverRequest {
    private Boolean allowLegacyUntracked;
    private List<String> requestKeys;

    public Boolean getAllowLegacyUntracked() {
        return this.allowLegacyUntracked;
    }

    public void setAllowLegacyUntracked(Boolean allowLegacyUntracked) {
        this.allowLegacyUntracked = allowLegacyUntracked;
    }

    public List<String> getRequestKeys() {
        return this.requestKeys;
    }

    public void setRequestKeys(List<String> requestKeys) {
        this.requestKeys = requestKeys;
    }
}
