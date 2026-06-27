package com.sdkwork.im.backend.api.generated

data class ProtocolRegistryResponse(
    val bindings: List<String>? = null,
    val codecs: List<String>? = null,
    val compatibilityMatrix: List<ClientCompatibilityResponse>? = null,
    val protocolVersion: String? = null,
    val schemas: List<ProtocolSchemaResponse>? = null
)
