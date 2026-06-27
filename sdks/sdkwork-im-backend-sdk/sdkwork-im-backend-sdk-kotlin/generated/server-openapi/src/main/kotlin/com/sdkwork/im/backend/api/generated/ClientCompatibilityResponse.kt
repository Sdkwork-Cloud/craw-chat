package com.sdkwork.im.backend.api.generated

data class ClientCompatibilityResponse(
    val blockedExperimentalCapabilities: List<String>? = null,
    val clientType: String? = null,
    val minimumProtocolVersion: String? = null,
    val supportedBindings: List<String>? = null,
    val supportedCapabilities: List<String>? = null,
    val supportedCodecs: List<String>? = null
)
