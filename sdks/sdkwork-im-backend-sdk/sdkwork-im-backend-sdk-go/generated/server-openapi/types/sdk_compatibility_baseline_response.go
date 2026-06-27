package types


type SdkCompatibilityBaselineResponse struct {
	AppSdkFamily string `json:"appSdkFamily"`
	BackendSdkFamily string `json:"backendSdkFamily"`
	ImSdkFamily string `json:"imSdkFamily"`
	RtcSdkFamily string `json:"rtcSdkFamily"`
	MatrixClientTypes []string `json:"matrixClientTypes"`
	ProtocolGovernancePath string `json:"protocolGovernancePath"`
	ProtocolRegistryPath string `json:"protocolRegistryPath"`
}
