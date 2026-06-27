package types


type ConversationInboxPreferencesView struct {
	IsPinned bool `json:"isPinned"`
	IsMuted bool `json:"isMuted"`
	IsMarkedUnread bool `json:"isMarkedUnread"`
	IsHidden bool `json:"isHidden"`
}
