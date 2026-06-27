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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceView?> CreateAsync(Sdkwork.Im.Sdk.Generated.Models.SpaceCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceView>(ApiPaths.ImPath("/spaces"), body, null, null, "application/json");
        }

        /// <summary>
        /// List spaces
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceListResponse?> ListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceListResponse>(ApiPaths.ImPath("/spaces"));
        }

        /// <summary>
        /// Get a space
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceView?> GetAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}"));
        }

        /// <summary>
        /// Update a space
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceView?> UpdateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberListResponse?> MembersListAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members"));
        }

        /// <summary>
        /// Create spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView?> MembersCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceMemberCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView?> MembersGetAsync(string spaceId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView?> MembersUpdateAsync(string spaceId, string userId, Sdkwork.Im.Sdk.Generated.Models.SpaceMemberUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceMemberView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupListResponse?> GroupsListAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups"));
        }

        /// <summary>
        /// Create spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView?> GroupsCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView?> GroupsGetAsync(string spaceId, string groupId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces groups
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView?> GroupsUpdateAsync(string spaceId, string groupId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberListResponse?> GroupsMembersListAsync(string spaceId, string groupId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members"));
        }

        /// <summary>
        /// Create spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberView?> GroupsMembersCreateAsync(string spaceId, string groupId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces groups members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberView?> GroupsMembersGetAsync(string spaceId, string groupId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces groups members
        /// </summary>
        public async Task GroupsMembersUpdateAsync(string spaceId, string groupId, string userId, Sdkwork.Im.Sdk.Generated.Models.SpaceGroupMemberUpdateRequest body)
        {
            await _client.PatchAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/members/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelListResponse?> ChannelsListAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels"));
        }

        /// <summary>
        /// Create spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView?> ChannelsCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView?> ChannelsGetAsync(string spaceId, string channelId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"));
        }

        /// <summary>
        /// Update spaces channels
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView?> ChannelsUpdateAsync(string spaceId, string channelId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelUpdateRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleListResponse?> ChannelsAccessRulesListAsync(string spaceId, string channelId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/access_rules"));
        }

        /// <summary>
        /// Create spaces channels access Rules
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleView?> ChannelsAccessRulesCreateAsync(string spaceId, string channelId, Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceChannelAccessRuleView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/access_rules"), body, null, null, "application/json");
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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteListResponse?> InvitesListAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites"));
        }

        /// <summary>
        /// Create spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteView?> InvitesCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceInviteCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces invites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteView?> InvitesGetAsync(string spaceId, string inviteCode)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceInviteView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}"));
        }

        /// <summary>
        /// Revoke spaces invites
        /// </summary>
        public async Task InvitesRevokeAsync(string spaceId, string inviteCode)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}"));
        }

        /// <summary>
        /// Accept spaces invites
        /// </summary>
        public async Task InvitesAcceptAsync(string spaceId, string inviteCode)
        {
            await _client.PostAsync<object>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/invites/{SerializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false))}/accept"), null);
        }

        /// <summary>
        /// List spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceBanListResponse?> BansListAsync(string spaceId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceBanListResponse>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans"));
        }

        /// <summary>
        /// Create spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceBanView?> BansCreateAsync(string spaceId, Sdkwork.Im.Sdk.Generated.Models.SpaceBanCreateRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceBanView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get spaces bans
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SpaceBanView?> BansGetAsync(string spaceId, string userId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SpaceBanView>(ApiPaths.ImPath($"/spaces/{SerializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false))}/bans/{SerializePathParameter(userId, new PathParameterSpec("userId", "simple", false))}"));
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


    }
}
