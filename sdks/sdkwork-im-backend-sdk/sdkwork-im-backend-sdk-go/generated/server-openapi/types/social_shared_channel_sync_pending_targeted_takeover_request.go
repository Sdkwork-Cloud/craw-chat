package types


type SocialSharedChannelSyncPendingTargetedTakeoverRequest struct {
	AllowLegacyUntracked bool `json:"allowLegacyUntracked"`
	RequestKeys []string `json:"requestKeys"`
}
