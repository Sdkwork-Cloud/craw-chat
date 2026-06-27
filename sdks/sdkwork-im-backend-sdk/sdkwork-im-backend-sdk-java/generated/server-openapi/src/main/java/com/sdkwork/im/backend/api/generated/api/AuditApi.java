package com.sdkwork.im.backend.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class AuditApi {
    private final HttpClient client;
    
    public AuditApi(HttpClient client) {
        this.client = client;
    }

    /** List audit records */
    public Map<String, Object> recordsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/audit/records"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Record audit anchor */
    public Map<String, Object> recordsCreate() throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/audit/records"), null);
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Export audit bundle */
    public Map<String, Object> exportRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/audit/export"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
