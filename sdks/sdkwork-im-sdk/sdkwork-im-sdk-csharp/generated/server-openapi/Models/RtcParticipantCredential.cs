using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RtcParticipantCredential
    {
        public string TenantId { get; set; }
        public string RtcSessionId { get; set; }
        public string ParticipantId { get; set; }
        public string Credential { get; set; }
        public string ExpiresAt { get; set; }
    }
}
