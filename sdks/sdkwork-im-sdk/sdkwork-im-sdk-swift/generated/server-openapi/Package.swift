// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "ImSDK",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15),
    ],
    products: [
        .library(
            name: "ImSDK",
            targets: ["ImSDK"]
        ),
    ],
    dependencies: [
        .package(url: "https://github.com/sdkwork/sdk-common-swift.git", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "ImSDK",
            dependencies: ["SDKworkCommon"],
            path: "Sources"
        )
    ]
)
