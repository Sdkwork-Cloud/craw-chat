package types


type RequestNotification struct {
	NotificationId string `json:"notificationId"`
	SourceEventId string `json:"sourceEventId"`
	SourceEventType string `json:"sourceEventType"`
	Category string `json:"category"`
	Channel string `json:"channel"`
	RecipientId string `json:"recipientId"`
	RecipientKind string `json:"recipientKind"`
	Title string `json:"title"`
	Body string `json:"body"`
	Payload string `json:"payload"`
}
