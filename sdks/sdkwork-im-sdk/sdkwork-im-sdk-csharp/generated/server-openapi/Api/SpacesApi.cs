using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.Sdk.Generated.Models;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;

namespace Sdkwork.Im.Sdk.Generated.Api
{
    public class SpacesApi
    {
        private readonly SdkHttpClient _client;

        public SpacesApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Create a space
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesCreateResponse201?> CreateAsync(Sdkwork.Im.Sdk.Generated.Models.SpaceCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesCreateResponse201>(ApiPaths.ImPath("/spaces"), body, null, null, "application/json");
        }

        /// <summary>
        /// List spaces
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesListResponse?> ListAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/spaces"), queryString));
        }

        /// <summary>
        /// Retrieve a space
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesRetrieveResponse?> RetrieveAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}"));
        }

        /// <summary>
        /// Update a space
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesUpdateResponse?> UpdateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesUpdateResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete a space
        /// </summary>
        public async Task DeleteAsync(string spaceId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersListResponse?> MembersListAsync(string spaceId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members"), queryString));
        }

        /// <summary>
        /// Create spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersCreateResponse201?> MembersCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceMemberCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersRetrieveResponse?> MembersRetrieveAsync(string spaceId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersUpdateResponse?> MembersUpdateAsync(string spaceId, string userId, Sdkwork.Im.Sdk.Generated.Models.SpaceMemberUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesMembersUpdateResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete spaces members
        /// </summary>
        public async Task MembersDeleteAsync(string spaceId, string userId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsListResponse?> GroupsListAsync(string spaceId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups"), queryString));
        }

        /// <summary>
        /// Create spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsCreateResponse201?> GroupsCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsRetrieveResponse?> GroupsRetrieveAsync(string spaceId, string groupId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsUpdateResponse?> GroupsUpdateAsync(string spaceId, string groupId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsUpdateResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete spaces groups
        /// </summary>
        public async Task GroupsDeleteAsync(string spaceId, string groupId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersListResponse?> GroupsMembersListAsync(string spaceId, string groupId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members"), queryString));
        }

        /// <summary>
        /// Create spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersCreateResponse201?> GroupsMembersCreateAsync(string spaceId, string groupId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersRetrieveResponse?> GroupsMembersRetrieveAsync(string spaceId, string groupId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersUpdateResponse?> GroupsMembersUpdateAsync(string spaceId, string groupId, string userId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesGroupsMembersUpdateResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete spaces groups members
        /// </summary>
        public async Task GroupsMembersDeleteAsync(string spaceId, string groupId, string userId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsListResponse?> ChannelsListAsync(string spaceId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels"), queryString));
        }

        /// <summary>
        /// Create spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsCreateResponse201?> ChannelsCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsRetrieveResponse?> ChannelsRetrieveAsync(string spaceId, string channelId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsUpdateResponse?> ChannelsUpdateAsync(string spaceId, string channelId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsUpdateResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete spaces channels
        /// </summary>
        public async Task ChannelsDeleteAsync(string spaceId, string channelId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces channels access Rules
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsAccessRulesListResponse?> ChannelsAccessRulesListAsync(string spaceId, string channelId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsAccessRulesListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/access_rules"), queryString));
        }

        /// <summary>
        /// Create spaces channels access Rules
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsAccessRulesCreateResponse201?> ChannelsAccessRulesCreateAsync(string spaceId, string channelId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesChannelsAccessRulesCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/access_rules"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete spaces channels access Rules
        /// </summary>
        public async Task ChannelsAccessRulesDeleteAsync(string spaceId, string channelId, string ruleId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/access_rules/{SerializePathParameter(ruleId, new PathParameterSpec("ruleId", "simple", false))}"));
        }

        /// <summary>
        /// List spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesListResponse?> InvitesListAsync(string spaceId, string? status = null, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("status", status, "form", true, false, null),
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites"), queryString));
        }

        /// <summary>
        /// Create spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesCreateResponse201?> InvitesCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceInviteCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesRetrieveResponse?> InvitesRetrieveAsync(string spaceId, string inviteCode)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesInvitesRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}"));
        }

        /// <summary>
        /// Delete spaces invites
        /// </summary>
        public async Task InvitesDeleteAsync(string spaceId, string inviteCode)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}"));
        }

        /// <summary>
        /// Accept spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SdkWorkCommandResponse?> InvitesAcceptAsync(string spaceId, string inviteCode)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SdkWorkCommandResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}/accept"), null);
        }

        /// <summary>
        /// List spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesBansListResponse?> BansListAsync(string spaceId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesBansListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans"), queryString));
        }

        /// <summary>
        /// Create spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesBansCreateResponse201?> BansCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceBanCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesBansCreateResponse201>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans"), body, null, null, "application/json");
        }

        /// <summary>
        /// retrieve spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpacesBansRetrieveResponse?> BansRetrieveAsync(string spaceId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpacesBansRetrieveResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// Delete spaces bans
        /// </summary>
        public async Task BansDeleteAsync(string spaceId, string userId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
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
