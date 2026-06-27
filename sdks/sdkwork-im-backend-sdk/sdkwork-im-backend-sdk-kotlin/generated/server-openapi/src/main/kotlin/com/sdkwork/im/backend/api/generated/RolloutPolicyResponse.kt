package com.sdkwork.im.backend.api.generated

data class RolloutPolicyResponse(
    val cellSelector: String? = null,
    val operatorOverride: Boolean? = null,
    val policyId: String? = null,
    val regionSelector: String? = null,
    val releaseChannel: String? = null,
    val tenantAllowlist: List<String>? = null,
    val trafficPercent: Int? = null
)
