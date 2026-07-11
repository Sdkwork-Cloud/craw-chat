# sdkwork-im-gateway-assembly Specs

This directory defines the local component contract for `sdkwork-im-gateway-assembly`.

The component owns IM application-plane router assembly. It composes route crates through
Cargo workspace dependencies and exposes `assemble_application_router` for standalone and
cloud gateway hosts.

Global SDKWork standards remain authoritative. This local spec only records the component
boundary, public runtime entrypoints, and verification commands.

