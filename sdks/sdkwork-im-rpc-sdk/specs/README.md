# SDKWork IM RPC SDK Component

This component defines the Sdkwork IM communication gRPC SDK family. Root SDKWork standards remain authoritative.

## Contract

- RPC manifest: `../rpc/sdkwork-im-rpc.manifest.json`
- Proto root: `../../../apis/rpc`
- Canonical packages: `sdkwork.communication.app.v3`, `sdkwork.communication.backend.v3`, `sdkwork.communication.internal.v1`, and `sdkwork.common.v1`

## Generated SDK Workspaces

The RPC SDK family uses the SDKWork RPC baseline language set:

| Language | Workspace | Package name | RPC inspection evidence |
| --- | --- | --- | --- |
| Go | `sdkwork-im-rpc-sdk-go` | `github.com/sdkwork/im-rpc-sdk-go` | convention evidence |
| Java | `sdkwork-im-rpc-sdk-java` | `com.sdkwork.im.rpc` | convention evidence |
| Python | `sdkwork-im-rpc-sdk-python` | `sdkwork_im_rpc_sdk` | convention evidence |
| Rust | `sdkwork-im-rpc-sdk-rust` | `sdkwork-im-rpc-sdk-rust` | convention evidence |
| TypeScript | `sdkwork-im-rpc-sdk-typescript` | `@sdkwork/im-rpc-sdk` | convention evidence |

RPC SDK source workspaces use convention evidence by default: the family root, `rpc/sdkwork-im-rpc.manifest.json`, proto source, generated language workspace name, `rpc-methods.json`, and the native package manifest. `sdkgen inspect --protocol rpc` must report a healthy workspace. Optional generator evidence is emitted only for release, CI, audit, or migration workflows with `--emit-control-plane`; its paths are derived by generator convention and are not repeated in this component contract.

## Verification

- `node scripts/dev/sdkwork-im-rpc-contract.test.mjs`
- `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript --json`
- `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go --json`
- `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java --json`
- `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python --json`
- `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust --json`
- `npx -y @bufbuild/buf@1.70.0 lint`
- `cargo test -p sdkwork-im-rpc-service-rust`
- `cd sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript && npm run check && npm run build`
- `cd sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go && go test ./...`
- `cd sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java && mvn -q -DskipTests package`
- `cd sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python && python -m compileall src generated/proto`
- `cd sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust && cargo check`
