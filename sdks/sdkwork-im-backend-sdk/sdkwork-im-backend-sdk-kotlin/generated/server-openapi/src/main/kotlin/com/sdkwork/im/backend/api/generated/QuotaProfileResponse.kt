package com.sdkwork.im.backend.api.generated

data class QuotaProfileResponse(
    val maxConcurrentSessionsPerTenant: Int? = null,
    val maxInflightMessages: Int? = null,
    val maxPayloadBytes: Int? = null,
    val maxSubscriptionsPerSession: Int? = null,
    val profileId: String? = null
)
