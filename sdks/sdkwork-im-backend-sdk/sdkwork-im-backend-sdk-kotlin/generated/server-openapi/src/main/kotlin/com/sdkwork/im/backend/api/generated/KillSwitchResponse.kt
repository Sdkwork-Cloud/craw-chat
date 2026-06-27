package com.sdkwork.im.backend.api.generated

data class KillSwitchResponse(
    val active: Boolean? = null,
    val disabledBindings: List<String>? = null,
    val disabledCapabilities: List<String>? = null,
    val disabledCodecs: List<String>? = null,
    val reason: String? = null,
    val ruleId: String? = null
)
