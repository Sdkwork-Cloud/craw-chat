using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProviderBindingSnapshot
    {
        public string InterfaceVersion { get; set; }
        public string TenantId { get; set; }
        public List<ProviderBindingItem> EffectiveBindings { get; set; }
        public List<string> Precedence { get; set; }
    }
}
