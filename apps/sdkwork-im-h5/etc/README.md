# SDKWork IM H5 Source Configuration

This deployable renderer consumes the enclosing IM deployment profile from
`../../../etc/sdkwork.deployment.config.json`. `browser.runtime.json` owns only H5 renderer
bindings and the local Vite target; public domains and SDK Base URLs remain root deployment values.

Validate with `node ../../../sdkwork-specs/tools/check-source-config-standard.mjs --root .` from this app root.
