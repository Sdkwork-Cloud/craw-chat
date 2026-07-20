# sdkwork-api-im-standalone-gateway Specs

This directory defines the local component contract for
`sdkwork-api-im-standalone-gateway`.

The component owns only the thin HTTP host around `sdkwork-api-im-assembly`. The full SDKWork IM
standalone application gateway, dependency composition, and lifecycle authority remain outside
this crate.

Global SDKWork standards remain authoritative. This local spec records the component boundary,
runtime entrypoint, required assembly dependency, and verification command.
