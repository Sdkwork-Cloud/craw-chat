package types

// Parsed @mention metadata extracted from message text parts. Allows notification fanout to determine who was mentioned and client rendering to highlight mentions without re-parsing.
type MessageMentions struct {
	UserIds []string `json:"userIds"`
	Scopes []MentionScope `json:"scopes"`
}
