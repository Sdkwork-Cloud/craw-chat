package com.sdkwork.im.backend.api.generated

data class QuotaProfileResponse(
    val maxConcurrentSessionsPerTenant: String? = null,
    val maxInflightMessages: String? = null,
    val maxPayloadBytes: String? = null,
    val maxSubscriptionsPerSession: String? = null,
    val profileId: String? = null
)
