package com.sdkwork.im.backend.api.generated

data class BlockUserRequest(
    val blockId: String? = null,
    val blockedUserId: String? = null,
    val blockerUserId: String? = null,
    val directChatId: String? = null,
    val effectiveAt: String? = null,
    val eventId: String? = null,
    val expiresAt: String? = null,
    val scope: String? = null
)
