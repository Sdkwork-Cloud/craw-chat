package types


type SocialWritePersistence struct {
	JournalAuthority bool `json:"journalAuthority"`
	SnapshotStatus SocialDerivedSnapshotStatus `json:"snapshotStatus"`
}
