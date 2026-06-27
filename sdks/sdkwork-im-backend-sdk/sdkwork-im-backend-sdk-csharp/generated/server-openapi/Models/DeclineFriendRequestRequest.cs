using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class DeclineFriendRequestRequest
    {
        public string? DeclinedAt { get; set; }
        public string? DeclinedByUserId { get; set; }
        public string? EventId { get; set; }
    }
}
