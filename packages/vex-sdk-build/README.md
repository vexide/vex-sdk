# vex-sdk-build

A build script helper to download and link to official SDKs from VEX.

This package provides a simple helper function for downloading and linking to official proprietary runtime SDKs distributed by VEX from Rust `build.rs` scripts.

> [!NOTE]
> SDKs distributed by VEX are proprietary software with limitations regarding ownership and distribution. Please familiarize yourself with these terms in VEX's [software EULA](https://www.vexrobotics.com/software-eula) and the `license.pdf` file included in VEX's' SDK.

> [!WARNING]
> This is fully unofficial and in no way affiliated, endorsed, supported, or created by VEX Robotics.

## Usage

This package should be added to your project's `build-dependencies`.

```toml
[build-dependencies]
vex-sdk-build = "0.1.0"
```

In order to download and link a certain SDK, simply call `vex_sdk_build::link_sdk` with your desired [SDK version](https://content.vexrobotics.com/vexos/public/V5/vscode/sdk/cpp/manifest.json) from a `build.rs` script in your project.

```rs
// build.rs
fn main() {
    vex_sdk_build::link_sdk("V5_20240802_15_00_00");
}
```

This will download the `V5_20240802_15_00_00` runtime SDK from VEX's CDN and link your project to `libv5rt.a`. From there, you can use [`vex-sdk`](https://docs.rs/vex-sdk) to call SDK functions over FFI from your Rust project!