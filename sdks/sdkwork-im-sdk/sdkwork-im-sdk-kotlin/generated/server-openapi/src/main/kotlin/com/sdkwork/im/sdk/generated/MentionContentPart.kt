package com.sdkwork.im.sdk.generated

data class MentionContentPart(
    val kind: String,
    val targetKind: String,
    val targetId: String,
    val displayText: String,
    val assignmentGeneration: Int
) : ContentPart
