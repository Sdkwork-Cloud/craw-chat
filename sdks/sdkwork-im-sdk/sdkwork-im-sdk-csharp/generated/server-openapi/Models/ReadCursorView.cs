using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ReadCursorView
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string PrincipalId { get; set; }
        public int ReadSeq { get; set; }
        public string UpdatedAt { get; set; }
    }
}
