# sdkwork-api-im-assembly Specs

This directory defines the local component contract for `sdkwork-api-im-assembly`.

The component owns IM application-plane router assembly. It composes route crates through
Cargo workspace dependencies and exposes `assemble_api_router` for standalone and
cloud gateway hosts.

Global SDKWork standards remain authoritative. This local spec only records the component
boundary, public runtime entrypoints, and verification commands.

