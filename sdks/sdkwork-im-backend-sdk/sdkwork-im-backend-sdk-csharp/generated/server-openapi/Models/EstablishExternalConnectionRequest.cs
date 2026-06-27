using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class EstablishExternalConnectionRequest
    {
        public string? ConnectionId { get; set; }
        public string? ConnectionKind { get; set; }
        public string? EstablishedAt { get; set; }
        public string? EventId { get; set; }
        public string? ExternalOrgName { get; set; }
        public string? ExternalTenantId { get; set; }
    }
}
