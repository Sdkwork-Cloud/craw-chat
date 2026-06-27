# Chat Window Real Login And RTC GUI Design

## Goal

Upgrade the local operator chat window into a realistic IM test surface that can:

- log in with visible account and password fields against the real `sdkwork-appbase login verification` backend
- keep and use issued bearer tokens for message and RTC operations
- drive RTC signaling flows from the same window
- let operators verify message timeline and RTC lifecycle events without mock media or fake auth

## Problem Summary

The current local chat toolchain is only partially operator-ready:

1. `chat-window-gui.ps1` can use real login only through launch arguments, not through a visible login form
2. the operator cannot create or control RTC sessions from the GUI even though the backend supports the full signaling contract
3. debugging login, message, and RTC issues requires switching between scripts, CLI commands, and logs instead of using one coherent surface

This blocks practical regression testing for the real local IM flow.

## Chosen Approach

Extend the existing `bin/chat-window-gui.ps1` window into a unified operator surface with four areas:

1. connection and conversation context
2. login form
3. message timeline and send box
4. RTC control and signaling panel

The same window will own the real login token, message actions, and RTC actions for one operator session.

### Why this approach

- preserves the existing operator entrypoint instead of fragmenting the workflow across multiple tools
- reuses current polling/timeline behavior, which is already stable for local testing
- keeps auth state local to the window that uses it, reducing bearer-token copy/paste mistakes
- gives the fastest route to a realistic manual test path for chat and RTC signaling

## Alternatives Considered

### 1. Separate RTC operator window

Rejected because it splits login, conversation, and RTC context across multiple windows and increases operator error.

### 2. CLI-only RTC workflow

Rejected because it does not satisfy the requirement for visible login and chat-window-based testing.

### 3. Full embedded WebRTC media preview

Rejected for this iteration because the architecture explicitly treats RTC here as a signaling concern first. Media-plane rendering would add browser/WebView complexity without improving backend signaling confidence.

## Architecture

### 1. Window Responsibilities

Each `chat-window-gui.ps1` instance will own one operator session:

- current base URL, tenant, conversation, session, and device
- login credentials entered by the operator
- resolved auth context after real login
- current polling state for timeline refresh
- current RTC session selection and signal payload drafts

The window will not persist secrets to disk. Diagnostics may log auth mode and user id, but never passwords or bearer tokens.

### 2. Login Panel

The top section of the window will expose:

- `Base URL`
- `Tenant`
- `Conversation`
- `User ID`
- `Login`
- `Password`
- `Session ID`
- `Client Route ID`
- `Login` button
- `Refresh Token` button is out of scope for this iteration

Behavior:

- when a bearer token is explicitly supplied on launch, the window starts in authenticated mode
- otherwise the operator may enter login and password and press `Login`
- seeded users continue to support inferred default passwords when launched non-interactively, but the visible UI requires explicit fields so the real flow is obvious
- successful login stores the returned access token in memory and updates the status bar
- failed login surfaces machine-readable backend errors in the status bar and diagnostics panel

### 3. Message Panel

The middle section keeps the existing transcript and send box but adds state guards:

- sending is disabled until the window has either a bearer token or an explicit offline skip mode
- refresh uses the same in-memory auth context as send
- the transcript continues to display message summaries, which allows RTC durable events to appear naturally in the same timeline

### 4. RTC Panel

Add a dedicated panel on the right or bottom side of the window with:

- `RTC Session ID`
- `Mode` selector with `voice` and `video`
- `Signaling Stream ID`
- `Artifact Message ID`
- `Signal Type`
- `Schema Ref`
- `Payload`
- buttons for `Create`, `Invite`, `Accept`, `Reject`, `End`, `Send Signal`, `Fetch Credentials`, `Fetch Recording`

Behavior:

- all RTC buttons require authenticated state
- `Create` uses `/im/v3/api/calls/sessions`
- `Invite` uses `/im/v3/api/calls/sessions/{id}/invite`
- `Accept` uses `/im/v3/api/calls/sessions/{id}/accept`
- `Reject` uses `/im/v3/api/calls/sessions/{id}/reject`
- `End` uses `/im/v3/api/calls/sessions/{id}/end`
- `Send Signal` uses `/im/v3/api/calls/sessions/{id}/signals`
- `Fetch Credentials` and `Fetch Recording` are debugging helpers for provider validation
- responses are appended to a diagnostics box and critical durable outcomes are also visible through the timeline as `rtc.*` summaries

### 5. Shared Auth Support Layer

Refactor PowerShell helpers so both `chat-window.ps1` and `chat-window-gui.ps1` can:

- resolve base URL consistently
- perform real login
- cache bearer-token auth context in memory
- build authenticated CLI arguments
- issue JSON API requests or `chat-cli` commands safely

This avoids drifting logic between console and GUI entrypoints.

### 6. Launcher Behavior

`open-chat-test.ps1` remains the fast operator bootstrap tool, but gains clearer modes:

- auto-login mode for seeded demo operator windows
- manual-login mode that opens blank login-ready windows
- scripted validation mode for CI-style chat verification

This keeps manual and automated testing aligned on the same surface.

## Data Flow

### Login Flow

1. operator opens chat window
2. operator enters login credentials
3. window calls `chat-cli login` or direct shared helper that hits `sdkwork-appbase login verification`
4. returned `accessToken` is stored in memory only
5. status updates to authenticated and enables chat + RTC actions

### RTC Flow

1. operator selects conversation and RTC session id
2. owner window creates and invites
3. guest window refreshes timeline and sees `rtc.invite`
4. guest window may send `rtc.offer` or `rtc.answer` through `Send Signal`
5. guest window accepts or rejects
6. either side ends the session
7. both windows observe durable `rtc.*` messages in the conversation timeline

## Error Handling

The window must fail closed and remain debuggable:

- login failures show backend error code and message
- send/refresh/RTC failures append a timestamped diagnostics line
- malformed RTC payload JSON is rejected client-side before sending
- missing required fields keep the action button from running
- service-down errors keep current form state intact so the operator can retry after restart

## Security And Commercial-Readiness Constraints

- never log passwords or bearer tokens
- do not persist credentials in config or diagnostics files
- reuse issued bearer tokens instead of synthesizing local auth when the operator chose real login
- validate required identifiers before requests to reduce accidental cross-session misuse
- preserve current access controls; the GUI must not bypass the backend auth or authorization contract

## Testing Strategy

### Script And Wrapper Tests

Add failing tests first for:

- help output surfacing the new login and RTC operator controls
- GUI launch in manual mode without eager network actions
- GUI launch in seeded auto-login mode still working after UI expansion

### Auth And Operator Flow Tests

Add failing tests first for:

- real login from script support helpers
- authenticated timeline refresh after login
- unauthenticated send blocked before token acquisition

### RTC Operator Flow Tests

Add failing tests first for:

- owner creates conversation and RTC session from real login
- guest sends `rtc.offer`
- guest accepts or rejects
- owner ends
- timeline contains expected `rtc.invite`, `rtc.offer`, `rtc.accept` or `rtc.reject`, and `rtc.end`

### Manual Verification

Run a real local scenario with two windows:

- owner logs in with account/password
- guest logs in with account/password
- chat message exchange succeeds
- video-mode RTC session is created and invited
- signal payload is submitted
- accept/end are visible in the same timeline

## Non-Goals For This Iteration

- actual camera capture or media playback
- refresh-token lifecycle UI
- multi-conversation workspace UI
- long-term credential persistence

## Success Criteria

The work is complete when all of the following are true:

- the chat window shows visible account/password login controls
- the operator can obtain a real bearer token from the local backend without leaving the window
- chat message send and timeline refresh work with that real token
- the same window exposes RTC signaling controls for `create`, `invite`, `accept`, `reject`, `end`, and custom signals
- a two-window local validation demonstrates the RTC signaling lifecycle through real backend APIs and timeline events
