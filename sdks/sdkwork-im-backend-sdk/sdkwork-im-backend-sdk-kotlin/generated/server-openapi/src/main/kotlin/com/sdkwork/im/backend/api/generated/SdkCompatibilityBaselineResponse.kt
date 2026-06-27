package com.sdkwork.im.backend.api.generated

data class SdkCompatibilityBaselineResponse(
    val appSdkFamily: String? = null,
    val backendSdkFamily: String? = null,
    val imSdkFamily: String? = null,
    val rtcSdkFamily: String? = null,
    val matrixClientTypes: List<String>? = null,
    val protocolGovernancePath: String? = null,
    val protocolRegistryPath: String? = null
)
