package types


type MediaResource struct {
	Id string `json:"id"`
	Kind MediaKind `json:"kind"`
	MediaKind MediaKind `json:"mediaKind"`
	Source MediaSource `json:"source"`
	Uri string `json:"uri"`
	PublicUrl string `json:"publicUrl"`
	Url string `json:"url"`
	Name string `json:"name"`
	Title string `json:"title"`
	FileName string `json:"fileName"`
	MimeType string `json:"mimeType"`
	Size int `json:"size"`
	SizeBytes string `json:"sizeBytes"`
	FileSize string `json:"fileSize"`
	DurationSeconds int `json:"durationSeconds"`
	Poster MediaResource `json:"poster"`
	Thumbnails []MediaResource `json:"thumbnails"`
}
