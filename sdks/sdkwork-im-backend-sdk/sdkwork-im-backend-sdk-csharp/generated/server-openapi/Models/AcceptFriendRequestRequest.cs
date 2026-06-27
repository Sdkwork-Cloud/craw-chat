using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class AcceptFriendRequestRequest
    {
        public string? AcceptedAt { get; set; }
        public string? AcceptedByUserId { get; set; }
        public string? EventId { get; set; }
    }
}
