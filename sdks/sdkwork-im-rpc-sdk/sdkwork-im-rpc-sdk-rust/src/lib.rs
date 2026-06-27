pub const RPC_SDK_PROTOCOL: &str = "rpc";
pub const GENERATED_PROTO_ROOT: &str = "generated/proto";

pub mod sdkwork {
    pub mod common {
        pub mod v1 {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/generated/proto/sdkwork/common/v1/sdkwork.common.v1.rs"
            ));
        }
    }
    pub mod communication {
        pub mod app {
            pub mod v3 {
                include!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/generated/proto/sdkwork/communication/app/v3/sdkwork.communication.app.v3.rs"
                ));
            }
        }
        pub mod backend {
            pub mod v3 {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/proto/sdkwork/communication/backend/v3/sdkwork.communication.backend.v3.rs"));
            }
        }
        pub mod internal {
            pub mod v1 {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/proto/sdkwork/communication/internal/v1/sdkwork.communication.internal.v1.rs"));
            }
        }
    }
}
