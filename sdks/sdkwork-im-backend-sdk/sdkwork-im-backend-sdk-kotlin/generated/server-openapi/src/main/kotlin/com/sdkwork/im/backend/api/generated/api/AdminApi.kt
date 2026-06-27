package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class AdminApi(private val client: HttpClient) {

    /** listApiKeyGroups */
    suspend fun apiKeyGroupsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/api_key_groups"))
    }

    /** createApiKeyGroup */
    suspend fun apiKeyGroupsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/api_key_groups"), body, null, null, "application/json")
    }

    /** updateApiKeyGroup */
    suspend fun apiKeyGroupsUpdate(groupId: String, body: Map<String, Any>): Any? {
        return client.patch(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json")
    }

    /** deleteApiKeyGroup */
    suspend fun apiKeyGroupsDelete(groupId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
    }

    /** updateApiKeyGroupStatus */
    suspend fun apiKeyGroupsStatus(groupId: String, body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/status"), body, null, null, "application/json")
    }

    /** listApiKeys */
    suspend fun apiKeysList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/api_keys"))
    }

    /** createApiKey */
    suspend fun apiKeysCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/api_keys"), body, null, null, "application/json")
    }

    /** updateApiKey */
    suspend fun apiKeysUpdate(hashedKey: String, body: Map<String, Any>): Any? {
        return client.put(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}"), body, null, null, "application/json")
    }

    /** deleteApiKey */
    suspend fun apiKeysDelete(hashedKey: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}"))
    }

    /** updateApiKeyStatus */
    suspend fun apiKeysStatus(hashedKey: String, body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}/status"), body, null, null, "application/json")
    }

    /** listBillingEvents */
    suspend fun billingEventsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/billing/events"))
    }

    /** getBillingEventSummary */
    suspend fun billingEventsSummaryRetrieve(): Any? {
        return client.get(ApiPaths.backendPath("/admin/billing/events/summary"))
    }

    /** getBillingSummary */
    suspend fun billingSummaryRetrieve(): Any? {
        return client.get(ApiPaths.backendPath("/admin/billing/summary"))
    }

    /** listChannelModels */
    suspend fun channelModelsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/channel_models"))
    }

    /** saveChannelModel */
    suspend fun channelModelsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/channel_models"), body, null, null, "application/json")
    }

    /** deleteChannelModel */
    suspend fun channelModelsDelete(channelId: String, modelId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/channel_models/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/models/${serializePathParameter(modelId, PathParameterSpec("modelId", "simple", false))}"))
    }

    /** listChannels */
    suspend fun channelsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/channels"))
    }

    /** saveChannel */
    suspend fun channelsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/channels"), body, null, null, "application/json")
    }

    /** deleteChannel */
    suspend fun channelsDelete(channelId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
    }

    /** listCredentials */
    suspend fun credentialsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/credentials"))
    }

    /** saveCredential */
    suspend fun credentialsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/credentials"), body, null, null, "application/json")
    }

    /** deleteCredential */
    suspend fun credentialsProvidersKeysDelete(tenantId: String, providerId: String, keyReference: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/credentials/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}/keys/${serializePathParameter(keyReference, PathParameterSpec("keyReference", "simple", false))}"))
    }

    /** reloadExtensionRuntimes */
    suspend fun extensionsRuntimeReloadsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json")
    }

    /** listRuntimeStatuses */
    suspend fun extensionsRuntimeStatusesList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/extensions/runtime_statuses"))
    }

    /** listRateLimitPolicies */
    suspend fun gatewayRateLimitPoliciesList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"))
    }

    /** createRateLimitPolicy */
    suspend fun gatewayRateLimitPoliciesCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json")
    }

    /** listRateLimitWindows */
    suspend fun gatewayRateLimitWindowsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"))
    }

    /** listMarketingCampaigns */
    suspend fun marketingCampaignsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/marketing/campaigns"))
    }

    /** saveMarketingCampaign */
    suspend fun marketingCampaignsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body, null, null, "application/json")
    }

    /** updateMarketingCampaignStatus */
    suspend fun marketingCampaignsStatus(marketingCampaignId: String, body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, PathParameterSpec("marketingCampaignId", "simple", false))}/status"), body, null, null, "application/json")
    }

    /** listModelPrices */
    suspend fun modelPricesList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/model_prices"))
    }

    /** saveModelPrice */
    suspend fun modelPricesCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/model_prices"), body, null, null, "application/json")
    }

    /** deleteModelPrice */
    suspend fun modelPricesProvidersDelete(channelId: String, modelId: String, proxyProviderId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/model_prices/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/models/${serializePathParameter(modelId, PathParameterSpec("modelId", "simple", false))}/providers/${serializePathParameter(proxyProviderId, PathParameterSpec("proxyProviderId", "simple", false))}"))
    }

    /** listModels */
    suspend fun modelsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/models"))
    }

    /** saveModel */
    suspend fun modelsCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/models"), body, null, null, "application/json")
    }

    /** deleteModel */
    suspend fun modelsProvidersDelete(externalName: String, providerId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/models/${serializePathParameter(externalName, PathParameterSpec("externalName", "simple", false))}/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}"))
    }

    /** listProviders */
    suspend fun providersList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/providers"))
    }

    /** saveProvider */
    suspend fun providersCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/providers"), body, null, null, "application/json")
    }

    /** deleteProvider */
    suspend fun providersDelete(providerId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}"))
    }

    /** listRoutingDecisionLogs */
    suspend fun routingDecisionLogsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/routing/decision_logs"))
    }

    /** listProviderHealthSnapshots */
    suspend fun routingHealthSnapshotsRetrieve(): Any? {
        return client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"))
    }

    /** listRoutingProfiles */
    suspend fun routingProfilesList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/routing/profiles"))
    }

    /** createRoutingProfile */
    suspend fun routingProfilesCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/routing/profiles"), body, null, null, "application/json")
    }

    /** listCompiledRoutingSnapshots */
    suspend fun routingSnapshotsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/routing/snapshots"))
    }

    /** listStorageAuditTrail */
    suspend fun storageAuditList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/storage/audit"))
    }

    /** getGlobalStorageConfig */
    suspend fun storageConfigRetrieve(): Any? {
        return client.get(ApiPaths.backendPath("/admin/storage/config"))
    }

    /** saveGlobalStorageConfig */
    suspend fun storageConfigCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/storage/config"), body, null, null, "application/json")
    }

    /** getTenantStorageConfig */
    suspend fun storageConfigTenantsRetrieve(tenantId: String): Any? {
        return client.get(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
    }

    /** saveTenantStorageConfig */
    suspend fun storageConfigTenantsCreate(tenantId: String, body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json")
    }

    /** deleteTenantStorageConfig */
    suspend fun storageConfigTenantsDelete(tenantId: String): Any? {
        return client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
    }

    /** getTenantEffectiveStorageConfig */
    suspend fun storageEffectiveTenantsRetrieve(tenantId: String): Any? {
        return client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
    }

    /** listStorageProviders */
    suspend fun storageProvidersList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/storage/providers"))
    }

    /** validateGlobalStorageConfig */
    suspend fun storageValidationCreate(body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/storage/validate"), body, null, null, "application/json")
    }

    /** validateTenantStorageConfig */
    suspend fun storageValidationTenantsCreate(tenantId: String, body: Map<String, Any>): Any? {
        return client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json")
    }

    /** listUsageRecords */
    suspend fun usageRecordsList(): Any? {
        return client.get(ApiPaths.backendPath("/admin/usage/records"))
    }

    /** getUsageSummary */
    suspend fun usageSummaryRetrieve(): Any? {
        return client.get(ApiPaths.backendPath("/admin/usage/summary"))
    }

    private data class PathParameterSpec(val name: String, val style: String, val explode: Boolean)

    private fun serializePathParameter(value: Any?, spec: PathParameterSpec): String {
        if (value == null) return ""
        val style = spec.style.ifBlank { "simple" }
        return when (value) {
            is Iterable<*> -> serializePathArray(spec.name, value, style, spec.explode)
            is Map<*, *> -> serializePathObject(spec.name, value, style, spec.explode)
            else -> pathPrimitivePrefix(spec.name, style) + pathEncode(value.toString())
        }
    }

    private fun serializePathArray(name: String, values: Iterable<*>, style: String, explode: Boolean): String {
        val serialized = values.mapNotNull { it?.toString()?.let(::pathEncode) }
        if (serialized.isEmpty()) return pathPrefix(name, style)
        if (style == "matrix") {
            if (explode) {
                return serialized.joinToString("") { ";$name=$it" }
            }
            return ";$name=" + serialized.joinToString(",")
        }
        val separator = if (explode) "." else ","
        return pathPrefix(name, style) + serialized.joinToString(separator)
    }

    private fun serializePathObject(name: String, values: Map<*, *>, style: String, explode: Boolean): String {
        val entries = mutableListOf<String>()
        val exploded = mutableListOf<String>()
        values.forEach { (key, value) ->
            if (value == null) return@forEach
            val escapedKey = pathEncode(key.toString())
            val escapedValue = pathEncode(value.toString())
            if (explode) {
                if (style == "matrix") {
                    exploded += ";$escapedKey=$escapedValue"
                } else {
                    exploded += "$escapedKey=$escapedValue"
                }
            } else {
                entries += escapedKey
                entries += escapedValue
            }
        }
        if (style == "matrix") {
            if (explode) return exploded.joinToString("")
            return ";$name=" + entries.joinToString(",")
        }
        if (explode) {
            val separator = if (style == "label") "." else ","
            return pathPrefix(name, style) + exploded.joinToString(separator)
        }
        return pathPrefix(name, style) + entries.joinToString(",")
    }

    private fun pathPrefix(name: String, style: String): String {
        return when (style) {
            "label" -> "."
            "matrix" -> ";$name"
            else -> ""
        }
    }

    private fun pathPrimitivePrefix(name: String, style: String): String {
        return if (style == "matrix") ";$name=" else pathPrefix(name, style)
    }

    private fun pathEncode(value: String): String {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20")
    }


}
