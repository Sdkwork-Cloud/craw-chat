using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProviderBindingSnapshotPageData
    {
        public List<ProviderBindingSnapshot> Items { get; set; }
        public PageInfo PageInfo { get; set; }
    }
}
