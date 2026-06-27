package types


type MediaContentPart struct {
	Kind string `json:"kind"`
	Drive DriveReference `json:"drive"`
	Resource MediaResource `json:"resource"`
	MediaRole string `json:"mediaRole"`
}
