package types


type MentionContentPart struct {
	Kind string `json:"kind"`
	TargetKind string `json:"targetKind"`
	TargetId string `json:"targetId"`
	DisplayText string `json:"displayText"`
	AssignmentGeneration int `json:"assignmentGeneration"`
}
