using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class OpsApi
    {
        private readonly SdkHttpClient _client;

        public OpsApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Retrieve ops health
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.HealthRetrieveResponse?> HealthRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.HealthRetrieveResponse>(ApiPaths.BackendPath("/ops/health"));
        }

        /// <summary>
        /// Retrieve cluster state
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ClusterRetrieveResponse?> ClusterRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ClusterRetrieveResponse>(ApiPaths.BackendPath("/ops/cluster"));
        }

        /// <summary>
        /// Retrieve operational lag
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.LagListResponse?> LagRetrieveAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.LagListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/ops/lag"), queryString));
        }

        /// <summary>
        /// Retrieve commercial readiness
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.CommercialReadinessRetrieveResponse?> CommercialReadinessRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.CommercialReadinessRetrieveResponse>(ApiPaths.BackendPath("/ops/commercial_readiness"));
        }

        /// <summary>
        /// Inspect runtime directory
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RuntimeDirRetrieveResponse?> RuntimeDirRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.RuntimeDirRetrieveResponse>(ApiPaths.BackendPath("/ops/runtime_dir"));
        }

        /// <summary>
        /// List provider bindings
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingSnapshotListResponse?> ProviderBindingsListAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingSnapshotListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/ops/provider_bindings"), queryString));
        }

        /// <summary>
        /// Retrieve provider binding drift
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingDriftListResponse?> ProviderBindingsDriftRetrieveAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingDriftListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/ops/provider_bindings/drift"), queryString));
        }

        /// <summary>
        /// Retrieve diagnostics
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.DiagnosticsRetrieveResponse?> DiagnosticsRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.DiagnosticsRetrieveResponse>(ApiPaths.BackendPath("/ops/diagnostics"));
        }


        private sealed record QueryParameterSpec(
            string Name,
            object? Value,
            string Style,
            bool Explode,
            bool AllowReserved,
            string? ContentType);

        private static string BuildQueryString(IEnumerable<QueryParameterSpec> parameters)
        {
            var pairs = new List<string>();
            foreach (var parameter in parameters)
            {
                AppendSerializedParameter(pairs, parameter);
            }
            return string.Join("&", pairs);
        }

        private static void AppendSerializedParameter(List<string> pairs, QueryParameterSpec parameter)
        {
            if (parameter.Value is null)
            {
                return;
            }

            if (!string.IsNullOrWhiteSpace(parameter.ContentType))
            {
                var json = System.Text.Json.JsonSerializer.Serialize(parameter.Value);
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(json, parameter.AllowReserved));
                return;
            }

            var style = string.IsNullOrWhiteSpace(parameter.Style) ? "form" : parameter.Style;
            if (style == "deepObject" && parameter.Value is System.Collections.IDictionary deepObject)
            {
                AppendDeepObjectParameter(pairs, parameter.Name, deepObject, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IEnumerable enumerable && parameter.Value is not string && parameter.Value is not System.Collections.IDictionary)
            {
                AppendArrayParameter(pairs, parameter.Name, enumerable, style, parameter.Explode, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IDictionary dictionary)
            {
                AppendObjectParameter(pairs, parameter.Name, dictionary, style, parameter.Explode, parameter.AllowReserved);
            }
            else
            {
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(parameter.Value.ToString() ?? string.Empty, parameter.AllowReserved));
            }
        }

        private static void AppendArrayParameter(List<string> pairs, string name, System.Collections.IEnumerable values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(item.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count == 0)
            {
                return;
            }
            if (style == "form" && explode)
            {
                foreach (var item in serialized)
                {
                    pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(item, allowReserved));
                }
                return;
            }
            pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
        }

        private static void AppendObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                if (style == "form" && explode)
                {
                    pairs.Add(Uri.EscapeDataString(item.Key.ToString() ?? string.Empty) + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
                else
                {
                    serialized.Add(item.Key.ToString() ?? string.Empty);
                    serialized.Add(item.Value.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count > 0)
            {
                pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
            }
        }

        private static void AppendDeepObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, bool allowReserved)
        {
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is not null)
                {
                    pairs.Add(Uri.EscapeDataString(name + "[" + item.Key + "]") + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
            }
        }

        private static string EncodeQueryValue(string value, bool allowReserved)
        {
            var encoded = Uri.EscapeDataString(value);
            if (!allowReserved)
            {
                return encoded;
            }
            return encoded
                .Replace("%3A", ":").Replace("%2F", "/").Replace("%3F", "?").Replace("%23", "#")
                .Replace("%5B", "[").Replace("%5D", "]").Replace("%40", "@").Replace("%21", "!")
                .Replace("%24", "$").Replace("%26", "&").Replace("%27", "'").Replace("%28", "(")
                .Replace("%29", ")").Replace("%2A", "*").Replace("%2B", "+").Replace("%2C", ",")
                .Replace("%3B", ";").Replace("%3D", "=");
        }

    }
}
