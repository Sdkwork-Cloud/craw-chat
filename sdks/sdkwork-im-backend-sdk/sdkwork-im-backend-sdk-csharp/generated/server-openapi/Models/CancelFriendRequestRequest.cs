using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class CancelFriendRequestRequest
    {
        public string? CanceledAt { get; set; }
        public string? CanceledByUserId { get; set; }
        public string? EventId { get; set; }
    }
}
