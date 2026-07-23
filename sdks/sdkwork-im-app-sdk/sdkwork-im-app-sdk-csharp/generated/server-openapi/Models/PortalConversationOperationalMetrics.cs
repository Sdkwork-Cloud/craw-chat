using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalConversationOperationalMetrics
    {
        public string LaggingScopeCount { get; set; }
        public string MaxOperationalLag { get; set; }
        public string PendingOutboxEventCount { get; set; }
        public string FailedOutboxAttemptCount { get; set; }
    }
}
