package types


type NotificationTask struct {
	TenantId string `json:"tenantId"`
	NotificationId string `json:"notificationId"`
	SourceEventId string `json:"sourceEventId"`
	SourceEventType string `json:"sourceEventType"`
	Category string `json:"category"`
	Channel string `json:"channel"`
	RecipientId string `json:"recipientId"`
	RecipientKind string `json:"recipientKind"`
	Status NotificationStatus `json:"status"`
	Title string `json:"title"`
	Body string `json:"body"`
	Payload string `json:"payload"`
	RequestedAt string `json:"requestedAt"`
	DispatchedAt string `json:"dispatchedAt"`
	FailureReason string `json:"failureReason"`
}
