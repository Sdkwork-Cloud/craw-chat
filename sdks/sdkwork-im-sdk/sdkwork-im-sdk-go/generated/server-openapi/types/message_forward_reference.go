package types

// Source tracing metadata for a forwarded message. Carries attribution to the original message across conversations so the UI can render a "Forwarded from <sender>" label and preserve audit provenance. The forwarder remains the Sender of the new message; this object only records where the content originated. Cross-conversation recall visibility is intentionally NOT cascaded — recipients of a forward see the original snapshot at forward-time.
type MessageForwardReference struct {
	OriginalMessageId string `json:"originalMessageId"`
	OriginalConversationId string `json:"originalConversationId"`
	OriginalSenderId string `json:"originalSenderId"`
	OriginalSenderKind string `json:"originalSenderKind"`
	OriginalSenderDisplayName string `json:"originalSenderDisplayName"`
	OriginalOccurredAt string `json:"originalOccurredAt"`
	ForwardedAt string `json:"forwardedAt"`
	ContentPreview string `json:"contentPreview"`
	ForwardCount int `json:"forwardCount"`
}
