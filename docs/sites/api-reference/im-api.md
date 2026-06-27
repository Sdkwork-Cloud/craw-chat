# IM Standard API Overview

<p class="api-page-intro">
  The IM Standard API is the standardized development HTTP surface for instant messaging
  capabilities under <code>/im/v3/api/*</code>. It maps to the <code>sdkwork-im-sdk</code>
  family and must not include backend management, control, admin, or non-IM app-business APIs.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Transport and Presence</h3>
    <p>Client route heartbeat, realtime subscriptions, event polling, and WebSocket upgrade.</p>
    <p><a href="/api-reference/im/session-and-realtime">Open Realtime APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Conversation Runtime</h3>
    <p>Inbox, conversation creation, agent dialogs, handoffs, membership management, read cursors, and messages.</p>
    <p><a href="/api-reference/im/conversations">Open Conversation APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Rooms</h3>
    <p>Live, chat, and game room binding, self-serve enter/leave, and capacity metadata on group conversations.</p>
    <p><a href="/api-reference/im/rooms">Open Room APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Media and Streams</h3>
    <p>Media upload lifecycle, media-to-message attachment, open streams, frame append, checkpoint, complete, and abort.</p>
    <p><a href="/api-reference/im/media">Open Media APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Calls</h3>
    <p>Create, invite, accept, reject, signal, and credential issuance routes for IM-owned call workflows.</p>
    <p><a href="/api-reference/im/calls">Open Calls APIs</a></p>
  </div>
</div>

## SDK Alignment

- IM standard endpoints under `/im/v3/api/*` map to `sdkwork-im-sdk`.
- The TypeScript package is `@sdkwork/im-sdk`; Flutter uses `im_sdk`.
- Non-management HTTP APIs outside the IM standardized surface belong to [App API](/api-reference/app-api) and `sdkwork-im-app-sdk`.
- Backend management, control, admin, operator, and audit APIs belong to [Backend API](/api-reference/backend-api) and `sdkwork-im-backend-sdk`.
- RTC provider runtime and native driver concerns belong to [RTC SDK](/sdk/rtc-sdk), not this OpenAPI HTTP family.

## IM Standard API Domains

<div class="api-link-list">
  <a href="/api-reference/im/session-and-realtime"><code>Realtime Presence</code> Presence heartbeat, realtime subscriptions, and event delivery</a>
  <a href="/api-reference/im/conversations"><code>Conversation</code> Conversation creation, system channels, and agent handoff flows</a>
  <a href="/api-reference/im/rooms"><code>Rooms</code> Live, chat, and game room lifecycle on group conversations</a>
  <a href="/api-reference/im/membership-and-read-state"><code>Membership</code> Member roster operations and read-cursor updates</a>
  <a href="/api-reference/im/messages"><code>Messages</code> Timeline reads, message send, edit, recall, and system-channel publish</a>
  <a href="/api-reference/im/media"><code>Media</code> Upload initiation, completion, media lookup, signed download, and attach</a>
  <a href="/api-reference/im/streams"><code>Streams</code> Stream open, frame append, list, checkpoint, complete, and abort</a>
  <a href="/api-reference/im/calls"><code>Calls</code> IM call signaling lifecycle, credentials, and RTC media handoff</a>
</div>
