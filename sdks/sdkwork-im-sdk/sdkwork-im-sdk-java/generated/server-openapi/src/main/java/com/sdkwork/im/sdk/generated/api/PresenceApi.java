package com.sdkwork.im.sdk.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.model.*;
import java.util.List;
import java.util.Map;

public class PresenceApi {
    private final HttpClient client;

    public PresenceApi(HttpClient client) {
        this.client = client;
    }

    /** Publish current client route presence heartbeat */
    public PresenceView heartbeatCreate(PresenceHeartbeatRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/presence/heartbeat"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PresenceView>() {});
    }

    /** Retrieve current principal presence */
    public PresenceView meRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.imPath("/presence/me"));
        return client.convertValue(raw, new TypeReference<PresenceView>() {});
    }




}
