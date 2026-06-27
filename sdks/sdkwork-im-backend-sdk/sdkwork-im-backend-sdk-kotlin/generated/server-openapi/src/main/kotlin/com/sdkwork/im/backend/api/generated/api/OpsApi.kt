package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class OpsApi(private val client: HttpClient) {

    /** Retrieve ops health */
    suspend fun healthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve cluster state */
    suspend fun clusterRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/cluster"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve projection lag */
    suspend fun lagRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/lag"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve replay status */
    suspend fun replayStatusRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/replay_status"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve commercial readiness */
    suspend fun commercialReadinessRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/commercial_readiness"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Inspect runtime directory */
    suspend fun runtimeDirRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/runtime_dir"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** List provider bindings */
    suspend fun providerBindingsList(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/provider_bindings"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve provider binding drift */
    suspend fun providerBindingsDriftRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/provider_bindings/drift"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve diagnostics */
    suspend fun diagnosticsRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/ops/diagnostics"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
