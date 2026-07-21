using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProviderBindingItem
    {
        public string Domain { get; set; }
        public string DefaultPluginId { get; set; }
        public string SelectedPluginId { get; set; }
        public string SelectionSource { get; set; }
        public bool TenantOverrideAllowed { get; set; }
    }
}
