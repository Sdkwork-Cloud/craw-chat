# SDKWork IM Flutter Source Configuration

This deployable mobile root consumes the enclosing IM deployment profile from
`../../../etc/sdkwork.deployment.config.json`. Flutter build/runtime materialization owns target
bindings only; public application and SDK origins remain root deployment values. Secrets and local
`dart-define` overlays are not committed here.
