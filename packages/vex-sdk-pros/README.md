# vex-sdk-pros

A crate that links to the PROS kernel as a provider for SDK functions.

This package satisfies the symbols declared by [`vex-sdk`](https://docs.rs/vex-sdk) by linking against the [PROS kernel](https://github.com/purduesigbots/pros), which bundles VEX's partner SDK (`libv5rts.a`). It also bridges the API incompatibilities between `vex-sdk` and the partner SDK by aliasing `vexTasksRun` to the partner SDK's `vexBackgroundProcessing`.

Use this crate if your project needs to use an official VEX SDK but you can't uphold VEXcode's stricter EULA and license. If you want an open-source implementation with no C dependencies, use [`vex-sdk-jumptable`](https://github.com/vexide/vex-sdk/tree/main/packages/vex-sdk-jumptable) instead; to link against a VEXcode SDK, use [`vex-sdk-vexcode`](https://github.com/vexide/vex-sdk/tree/main/packages/vex-sdk-vexcode).

> [!WARNING]
> This is fully unofficial and in no way affiliated, endorsed, supported, or created by VEX Robotics.

## Usage

Add this package to your project's dependencies alongside `vex-sdk`, then bring it into scope so that its symbols are linked in.

```toml
[dependencies]
vex-sdk = "*"
vex-sdk-pros = "*"
```

```rs
use vex_sdk_pros as _;
```

A prebuilt `libpros.a` is vendored in the `link/` directory and is linked statically by this crate's build script; no additional setup is required.

## Supported Targets

This crate only supports VEXos targets (`armv7a-vex-v5`). Building it for any other target raises a `compile_error!`.
