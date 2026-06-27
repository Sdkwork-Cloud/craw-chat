package com.sdkwork.im.sdk.generated

data class MediaContentPart(
    val kind: String,
    val drive: DriveReference,
    val resource: MediaResource,
    val mediaRole: String? = null
) : ContentPart
