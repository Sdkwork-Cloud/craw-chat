using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class RequestNotification
    {
        public string? NotificationId { get; set; }
        public string? SourceEventId { get; set; }
        public string? SourceEventType { get; set; }
        public string? Category { get; set; }
        public string? Channel { get; set; }
        public string? RecipientId { get; set; }
        public string? RecipientKind { get; set; }
        public string? Title { get; set; }
        public string? Body { get; set; }
        public string? Payload { get; set; }
    }
}
