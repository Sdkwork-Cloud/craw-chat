package com.sdkwork.im.backend.api.generated

data class EffectiveProtocolSnapshotResponse(
    val allowedBindings: List<String>? = null,
    val allowedCodecs: List<String>? = null,
    val enabledCapabilities: List<String>? = null,
    val killSwitchActive: Boolean? = null,
    val precedence: List<String>? = null,
    val protocolVersion: String? = null,
    val quotaProfileId: String? = null,
    val releaseChannel: String? = null
)
