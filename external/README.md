# external

This directory is reserved for external source references. It must not copy or
fork business implementation code into Sdkwork IM.

## xiaozhi

- Official source: `https://github.com/78/xiaozhi-esp32.git`
- Standard location: `external/xiaozhi-esp32`
- Standard command: `git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`

## Alignment

- AIoT device, telemetry, command, twin, and protocol behavior is owned by the
  sibling `sdkwork-aiot` workspace.
- Sdkwork IM may consume published `sdkwork-aiot` SDKs, Rust crates, or mounted
  API bridges, but must not recreate local device/provider adapters here.
