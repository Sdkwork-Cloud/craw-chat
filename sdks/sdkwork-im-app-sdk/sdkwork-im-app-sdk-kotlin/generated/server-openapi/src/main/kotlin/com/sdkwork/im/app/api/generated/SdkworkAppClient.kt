package com.sdkwork.im.app.api.generated

import com.sdkwork.common.core.SdkConfig

class SdkworkAppClient : SdkworkImAppClient {
    constructor(baseUrl: String) : super(baseUrl)
    constructor(config: SdkConfig) : super(config)
}
