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

    /** Read the tenant portal access snapshot */
    public AccessRetrieveResponse accessRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/access"));
        return client.convertValue(raw, new TypeReference<AccessRetrieveResponse>() {});
    }

    /** Read the tenant automation snapshot */
    public AutomationRetrieveResponse automationRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/automation"));
        return client.convertValue(raw, new TypeReference<AutomationRetrieveResponse>() {});
    }

    /** Read the tenant conversations snapshot */
    public ConversationSnapshotRetrieveResponse conversationSnapshotRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/conversations"));
        return client.convertValue(raw, new TypeReference<ConversationSnapshotRetrieveResponse>() {});
    }

    /** Read the tenant dashboard snapshot */
    public DashboardRetrieveResponse dashboardRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/dashboard"));
        return client.convertValue(raw, new TypeReference<DashboardRetrieveResponse>() {});
    }

    /** Read the tenant governance snapshot */
    public GovernanceRetrieveResponse governanceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/governance"));
        return client.convertValue(raw, new TypeReference<GovernanceRetrieveResponse>() {});
    }

    /** Read the tenant portal home snapshot */
    public HomeRetrieveResponse homeRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/home"));
        return client.convertValue(raw, new TypeReference<HomeRetrieveResponse>() {});
    }

    /** Read the tenant media snapshot */
    public MediaRetrieveResponse mediaRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/media"));
        return client.convertValue(raw, new TypeReference<MediaRetrieveResponse>() {});
    }

    /** Read the tenant realtime snapshot */
    public RealtimeRetrieveResponse realtimeRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/realtime"));
        return client.convertValue(raw, new TypeReference<RealtimeRetrieveResponse>() {});
    }

    /** Read the current tenant workspace snapshot */
    public WorkspaceRetrieveResponse workspaceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/portal/workspace"));
        return client.convertValue(raw, new TypeReference<WorkspaceRetrieveResponse>() {});
    }




}
