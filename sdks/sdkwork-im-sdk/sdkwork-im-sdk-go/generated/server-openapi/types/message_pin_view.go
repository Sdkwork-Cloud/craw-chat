package types


type MessagePinView struct {
	PinnedBy InteractionActorView `json:"pinnedBy"`
	PinnedAt string `json:"pinnedAt"`
}
