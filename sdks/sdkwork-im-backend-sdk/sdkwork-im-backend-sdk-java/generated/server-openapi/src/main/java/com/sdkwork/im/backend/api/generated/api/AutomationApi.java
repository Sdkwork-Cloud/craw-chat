package com.sdkwork.im.backend.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class AutomationApi {
    private final HttpClient client;
    
    public AutomationApi(HttpClient client) {
        this.client = client;
    }

    /** Retrieve automation governance */
    public Map<String, Object> governanceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/automation/governance"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
