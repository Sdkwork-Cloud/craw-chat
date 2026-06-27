using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class BlockUserRequest
    {
        public string? BlockId { get; set; }
        public string? BlockedUserId { get; set; }
        public string? BlockerUserId { get; set; }
        public string? DirectChatId { get; set; }
        public string? EffectiveAt { get; set; }
        public string? EventId { get; set; }
        public string? ExpiresAt { get; set; }
        public string? Scope { get; set; }
    }
}
