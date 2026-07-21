using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProviderBindingDriftItem
    {
        public string TenantId { get; set; }
        public string Domain { get; set; }
        public string BaselineSelectedPluginId { get; set; }
        public string SelectedPluginId { get; set; }
        public string BaselineSelectionSource { get; set; }
        public string SelectionSource { get; set; }
        public string DriftKind { get; set; }
    }
}
