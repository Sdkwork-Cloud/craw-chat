package types


type PortalDataAvailability struct {
	State string `json:"state"`
	Source string `json:"source"`
	Complete bool `json:"complete"`
	Reason string `json:"reason"`
}
