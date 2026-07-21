package com.sdkwork.im.backend.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class ControlApi {
    private final HttpClient client;

    public ControlApi(HttpClient client) {
        this.client = client;
    }

    /** Activate a realtime node and clear drain state. */
    public NodesActivateResponse nodesActivate(String nodeId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/nodes/" + serializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false)) + "/activate"), null);
        return client.convertValue(raw, new TypeReference<NodesActivateResponse>() {});
    }

    /** Mark a realtime node as draining. */
    public NodesDrainResponse nodesDrain(String nodeId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/nodes/" + serializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false)) + "/drain"), null);
        return client.convertValue(raw, new TypeReference<NodesDrainResponse>() {});
    }

    /** Migrate owned routes from the source node to the target node. */
    public NodesRoutesMigrateResponse nodesRoutesMigrate(String nodeId, MigrateRoutesRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/nodes/" + serializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false)) + "/routes/migrate"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<NodesRoutesMigrateResponse>() {});
    }

    /** Read the control-plane protocol governance snapshot. */
    public ProtocolGovernanceRetrieveResponse protocolGovernanceRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/protocol_governance"));
        return client.convertValue(raw, new TypeReference<ProtocolGovernanceRetrieveResponse>() {});
    }

    /** Read the control-plane protocol registry snapshot. */
    public ProtocolRegistryRetrieveResponse protocolRegistryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/protocol_registry"));
        return client.convertValue(raw, new TypeReference<ProtocolRegistryRetrieveResponse>() {});
    }

    /** Read provider policy history. */
    public SdkWorkListResponse providerPoliciesList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Read provider policy diff between two versions. */
    public SdkWorkListResponse providerPoliciesDiffList(String fromVersion, String toVersion, Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("fromVersion", fromVersion, "form", true, false, null),
            new QueryParameterSpec("toVersion", toVersion, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies/diff"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Preview the effective provider policy result before commit. */
    public ProviderPoliciesPreviewResponse providerPoliciesPreview(UpsertProviderBindingPolicyRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/provider_policies/preview"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ProviderPoliciesPreviewResponse>() {});
    }

    /** Rollback provider policy history to a target version. */
    public ProviderPoliciesRollbackResponse providerPoliciesRollback(ProviderPolicyRollbackRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/provider_policies/rollback"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ProviderPoliciesRollbackResponse>() {});
    }

    /** Read the provider registry snapshot. */
    public ProviderRegistryRetrieveResponse providerRegistryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/provider_registry"));
        return client.convertValue(raw, new TypeReference<ProviderRegistryRetrieveResponse>() {});
    }

    /** Read effective provider bindings. */
    public SdkWorkListResponse providerBindingsList(String tenantId, Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("tenantId", tenantId, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_bindings"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Upsert a provider binding policy. */
    public ControlProviderBindingsCreateResponse201 providerBindingsCreate(UpsertProviderBindingPolicyRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/provider_bindings"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ControlProviderBindingsCreateResponse201>() {});
    }

    /** Bind a direct chat to a conversation. */
    public SocialDirectChatsBindingsCreateResponse201 socialDirectChatsBindingsCreate(BindDirectChatRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/direct_chats/bindings"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialDirectChatsBindingsCreateResponse201>() {});
    }

    /** Read a direct chat snapshot. */
    public SocialDirectChatsRetrieveResponse socialDirectChatsRetrieve(String directChatId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/direct_chats/" + serializePathParameter(directChatId, new PathParameterSpec("directChatId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialDirectChatsRetrieveResponse>() {});
    }

    /** Establish an external collaboration connection. */
    public SocialExternalConnectionsCreateResponse201 socialExternalConnectionsCreate(EstablishExternalConnectionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/external_connections"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialExternalConnectionsCreateResponse201>() {});
    }

    /** Read an external connection snapshot. */
    public SocialExternalConnectionsRetrieveResponse socialExternalConnectionsRetrieve(String connectionId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/external_connections/" + serializePathParameter(connectionId, new PathParameterSpec("connectionId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialExternalConnectionsRetrieveResponse>() {});
    }

    /** Bind an external member link. */
    public SocialExternalMemberLinksCreateResponse201 socialExternalMemberLinksCreate(BindExternalMemberLinkRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/external_member_links"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialExternalMemberLinksCreateResponse201>() {});
    }

    /** Read an external member link snapshot. */
    public SocialExternalMemberLinksRetrieveResponse socialExternalMemberLinksRetrieve(String linkId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/external_member_links/" + serializePathParameter(linkId, new PathParameterSpec("linkId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialExternalMemberLinksRetrieveResponse>() {});
    }

    /** Submit a friend request event. */
    public SocialFriendRequestsCreateResponse201 socialFriendRequestsCreate(SubmitFriendRequestRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friend_requests"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendRequestsCreateResponse201>() {});
    }

    /** Read a friend request snapshot. */
    public SocialFriendRequestsRetrieveResponse socialFriendRequestsRetrieve(String requestId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialFriendRequestsRetrieveResponse>() {});
    }

    /** Accept a friend request. */
    public SocialFriendRequestsAcceptResponse socialFriendRequestsAccept(String requestId, AcceptFriendRequestRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/accept"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendRequestsAcceptResponse>() {});
    }

    /** Decline a friend request. */
    public SocialFriendRequestsDeclineResponse socialFriendRequestsDecline(String requestId, DeclineFriendRequestRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/decline"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendRequestsDeclineResponse>() {});
    }

    /** Cancel a friend request. */
    public SocialFriendRequestsCancelResponse socialFriendRequestsCancel(String requestId, CancelFriendRequestRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/cancel"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendRequestsCancelResponse>() {});
    }

    /** Activate a friendship event. */
    public SocialFriendshipsCreateResponse201 socialFriendshipsCreate(ActivateFriendshipRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friendships"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendshipsCreateResponse201>() {});
    }

    /** Read a friendship snapshot. */
    public SocialFriendshipsRetrieveResponse socialFriendshipsRetrieve(String friendshipId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/friendships/" + serializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialFriendshipsRetrieveResponse>() {});
    }

    /** Remove a friendship. */
    public SocialFriendshipsRemoveResponse socialFriendshipsRemove(String friendshipId, RemoveFriendshipRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/friendships/" + serializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false)) + "/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendshipsRemoveResponse>() {});
    }

    /** Claim selected pending shared-channel sync entries. */
    public SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201 socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedClaimRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201>() {});
    }

    /** Read the dead-letter shared-channel sync queue. */
    public SdkWorkListResponse socialRuntimeDeadLetterSharedChannelSyncList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/dead_letter_shared_channel_sync"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Read the delivered shared-channel sync ledger. */
    public SdkWorkListResponse socialRuntimeDeliveredSharedChannelSyncList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/delivered_shared_channel_sync"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Read merged shared-channel sync delivery state. */
    public SdkWorkListResponse socialRuntimeDeliveryStateSharedChannelSyncList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/delivery_state_shared_channel_sync"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Read the pending shared-channel sync queue. */
    public SdkWorkListResponse socialRuntimePendingSharedChannelSyncList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/pending_shared_channel_sync"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** Reclaim stale shared-channel sync pending ownership. */
    public SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201 socialRuntimeReclaimStalePendingSharedChannelSyncCreate() throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), null);
        return client.convertValue(raw, new TypeReference<SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201>() {});
    }

    /** Release selected pending shared-channel sync entries. */
    public SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201 socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedReleaseRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201>() {});
    }

    /** Repair the persisted social runtime derived snapshot. */
    public SocialRuntimeRepairDerivedSnapshotCreateResponse201 socialRuntimeRepairDerivedSnapshotCreate() throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/repair_derived_snapshot"), null);
        return client.convertValue(raw, new TypeReference<SocialRuntimeRepairDerivedSnapshotCreateResponse201>() {});
    }

    /** Repair shared-channel sync backlog state. */
    public SocialRuntimeRepairSharedChannelSyncCreateResponse201 socialRuntimeRepairSharedChannelSyncCreate() throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/repair_shared_channel_sync"), null);
        return client.convertValue(raw, new TypeReference<SocialRuntimeRepairSharedChannelSyncCreateResponse201>() {});
    }

    /** Republish selected pending shared-channel sync entries. */
    public SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201 socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncTargetedRepublishRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201>() {});
    }

    /** Requeue all dead-letter shared-channel sync entries. */
    public SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201 socialRuntimeRequeueDeadLetterSharedChannelSyncCreate() throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), null);
        return client.convertValue(raw, new TypeReference<SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201>() {});
    }

    /** Requeue selected dead-letter shared-channel sync entries. */
    public SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201 socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(SocialSharedChannelSyncDeadLetterTargetedRequeueRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201>() {});
    }

    /** Take over selected pending shared-channel sync entries. */
    public SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201 socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedTakeoverRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201>() {});
    }

    /** Apply a shared-channel policy. */
    public SocialSharedChannelPoliciesCreateResponse201 socialSharedChannelPoliciesCreate(ApplySharedChannelPolicyRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/shared_channel_policies"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialSharedChannelPoliciesCreateResponse201>() {});
    }

    /** Read a shared-channel policy snapshot. */
    public SocialSharedChannelPoliciesRetrieveResponse socialSharedChannelPoliciesRetrieve(String policyId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/shared_channel_policies/" + serializePathParameter(policyId, new PathParameterSpec("policyId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialSharedChannelPoliciesRetrieveResponse>() {});
    }

    /** Block a user in the social graph. */
    public SocialUserBlocksCreateResponse201 socialUserBlocksCreate(BlockUserRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/control/social/user_blocks"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialUserBlocksCreateResponse201>() {});
    }

    /** Read a user block snapshot. */
    public SocialUserBlocksRetrieveResponse socialUserBlocksRetrieve(String blockId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/control/social/user_blocks/" + serializePathParameter(blockId, new PathParameterSpec("blockId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SocialUserBlocksRetrieveResponse>() {});
    }

    private record PathParameterSpec(String name, String style, boolean explode) {}

    private static String serializePathParameter(Object value, PathParameterSpec spec) {
        if (value == null) {
            return "";
        }
        String style = spec.style() == null || spec.style().isBlank() ? "simple" : spec.style();
        if (value instanceof Iterable<?> iterable) {
            return serializePathArray(spec.name(), iterable, style, spec.explode());
        }
        if (value instanceof Map<?, ?> map) {
            return serializePathObject(spec.name(), map, style, spec.explode());
        }
        return pathPrimitivePrefix(spec.name(), style) + pathEncode(String.valueOf(value));
    }

    private static String serializePathArray(String name, Iterable<?> values, String style, boolean explode) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(pathEncode(String.valueOf(item)));
            }
        }
        if (serialized.isEmpty()) {
            return pathPrefix(name, style);
        }
        if ("matrix".equals(style)) {
            if (explode) {
                List<String> parts = new java.util.ArrayList<>();
                for (String item : serialized) {
                    parts.add(";" + name + "=" + item);
                }
                return String.join("", parts);
            }
            return ";" + name + "=" + String.join(",", serialized);
        }
        String separator = explode ? "." : ",";
        return pathPrefix(name, style) + String.join(separator, serialized);
    }

    private static String serializePathObject(String name, Map<?, ?> values, String style, boolean explode) {
        List<String> entries = new java.util.ArrayList<>();
        List<String> exploded = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            String escapedKey = pathEncode(String.valueOf(key));
            String escapedValue = pathEncode(String.valueOf(value));
            if (explode) {
                if ("matrix".equals(style)) {
                    exploded.add(";" + escapedKey + "=" + escapedValue);
                } else {
                    exploded.add(escapedKey + "=" + escapedValue);
                }
            } else {
                entries.add(escapedKey);
                entries.add(escapedValue);
            }
        });
        if ("matrix".equals(style)) {
            if (explode) {
                return String.join("", exploded);
            }
            return ";" + name + "=" + String.join(",", entries);
        }
        if (explode) {
            String separator = "label".equals(style) ? "." : ",";
            return pathPrefix(name, style) + String.join(separator, exploded);
        }
        return pathPrefix(name, style) + String.join(",", entries);
    }

    private static String pathPrefix(String name, String style) {
        if ("label".equals(style)) {
            return ".";
        }
        if ("matrix".equals(style)) {
            return ";" + name;
        }
        return "";
    }

    private static String pathPrimitivePrefix(String name, String style) {
        if ("matrix".equals(style)) {
            return ";" + name + "=";
        }
        return pathPrefix(name, style);
    }

    private static String pathEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20");
    }

    private record QueryParameterSpec(String name, Object value, String style, boolean explode, boolean allowReserved, String contentType) {}

    private static String buildQueryString(List<QueryParameterSpec> parameters) throws Exception {
        List<String> pairs = new java.util.ArrayList<>();
        for (QueryParameterSpec parameter : parameters) {
            appendSerializedParameter(pairs, parameter);
        }
        return String.join("&", pairs);
    }

    private static void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) throws Exception {
        if (parameter.value() == null) {
            return;
        }
        if (parameter.contentType() != null && !parameter.contentType().isBlank()) {
            String json = clientObjectMapper().writeValueAsString(parameter.value());
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(json, parameter.allowReserved()));
            return;
        }

        String style = parameter.style() == null || parameter.style().isBlank() ? "form" : parameter.style();
        Object value = parameter.value();
        if ("deepObject".equals(style) && value instanceof Map<?, ?> map) {
            appendDeepObjectParameter(pairs, parameter.name(), map, parameter.allowReserved());
        } else if (value instanceof Iterable<?> iterable) {
            appendArrayParameter(pairs, parameter.name(), iterable, style, parameter.explode(), parameter.allowReserved());
        } else if (value instanceof Map<?, ?> map) {
            appendObjectParameter(pairs, parameter.name(), map, style, parameter.explode(), parameter.allowReserved());
        } else {
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(String.valueOf(value), parameter.allowReserved()));
        }
    }

    private static void appendArrayParameter(List<String> pairs, String name, Iterable<?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(String.valueOf(item));
            }
        }
        if (serialized.isEmpty()) {
            return;
        }
        if ("form".equals(style) && explode) {
            for (String item : serialized) {
                pairs.add(urlEncode(name) + "=" + encodeQueryValue(item, allowReserved));
            }
            return;
        }
        pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
    }

    private static void appendObjectParameter(List<String> pairs, String name, Map<?, ?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            if ("form".equals(style) && explode) {
                pairs.add(urlEncode(String.valueOf(key)) + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            } else {
                serialized.add(String.valueOf(key));
                serialized.add(String.valueOf(value));
            }
        });
        if (!serialized.isEmpty()) {
            pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
        }
    }

    private static void appendDeepObjectParameter(List<String> pairs, String name, Map<?, ?> values, boolean allowReserved) {
        values.forEach((key, value) -> {
            if (value != null) {
                pairs.add(urlEncode(name + "[" + key + "]") + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            }
        });
    }

    private static String encodeQueryValue(String value, boolean allowReserved) {
        String encoded = urlEncode(value);
        if (!allowReserved) {
            return encoded;
        }
        return encoded
            .replace("%3A", ":").replace("%2F", "/").replace("%3F", "?").replace("%23", "#")
            .replace("%5B", "[").replace("%5D", "]").replace("%40", "@").replace("%21", "!")
            .replace("%24", "$").replace("%26", "&").replace("%27", "'").replace("%28", "(")
            .replace("%29", ")").replace("%2A", "*").replace("%2B", "+").replace("%2C", ",")
            .replace("%3B", ";").replace("%3D", "=");
    }

    private static com.fasterxml.jackson.databind.ObjectMapper clientObjectMapper() {
        return new com.fasterxml.jackson.databind.ObjectMapper();
    }


    private static String urlEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8);
    }
}
