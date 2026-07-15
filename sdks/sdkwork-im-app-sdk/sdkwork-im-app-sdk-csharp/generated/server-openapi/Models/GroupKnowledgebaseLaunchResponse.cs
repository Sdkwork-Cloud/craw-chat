using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class GroupKnowledgebaseLaunchResponse
    {
        public string ConversationId { get; set; }
        public string LifecycleState { get; set; }
        public string? SpaceId { get; set; }
        public string? SpaceUuid { get; set; }
        public string? LaunchTicket { get; set; }
        public string? ExpiresAt { get; set; }
        public string MembershipEpoch { get; set; }
        public string UpstreamLinkGeneration { get; set; }
        public string? ProvisioningOperationId { get; set; }
    }
}
