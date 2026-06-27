package com.sdkwork.im.app.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class PortalApi {
    private final HttpClient client;
    
    public PortalApi(HttpClient client) {
        this.client = client;
    }

    /** Read the tenant portal sign-in snapshot */
    public Map<String, Object> accessRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/access"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant automation snapshot */
    public Map<String, Object> automationRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/automation"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant conversations snapshot */
    public Map<String, Object> conversationSnapshotRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/conversations"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant dashboard snapshot */
    public Map<String, Object> dashboardRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/dashboard"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant governance snapshot */
    public Map<String, Object> governanceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/governance"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant portal home snapshot */
    public Map<String, Object> homeRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/home"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant media snapshot */
    public Map<String, Object> mediaRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/media"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant realtime snapshot */
    public Map<String, Object> realtimeRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/realtime"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the current tenant workspace snapshot */
    public PortalWorkspaceView workspaceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/workspace"));
        return client.convertValue(raw, new TypeReference<PortalWorkspaceView>() {});
    }




}
