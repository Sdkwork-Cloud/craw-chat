using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class RemoveFriendshipRequest
    {
        public string? EventId { get; set; }
        public string? RemovedAt { get; set; }
        public string? RemovedByUserId { get; set; }
    }
}
