using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class BlockUserRequest
    {
        public string BlockedUserId { get; set; }
        public string Scope { get; set; }
        public string? DirectChatId { get; set; }
        public string? ExpiresAt { get; set; }
    }
}
