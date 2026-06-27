package com.sdkwork.im.backend.api.generated

import com.sdkwork.common.core.SdkConfig

class SdkworkBackendClient : SdkworkImBackendClient {
    constructor(baseUrl: String) : super(baseUrl)
    constructor(config: SdkConfig) : super(config)
}
