# vex-sdk-jumptable

Raw bindings to VEXos user jump table functions.

This repository serves as a partial open-source reimplementation of VEX's V5 runtime SDK (libv5rt.a), but doesn't necessarily aim to be API or ABI compatible. The intent of this crate is to be used as building blocks for higher-level rust tooling to run on the V5 brain without linking to C libraries in any way.

- Jumptable and firmware address offsets are largely derived from [cetio's VEXAPI research repository](https://github.com/cetio/VEXAPI).
- Symbols and function signatures are taken from publicly available sources, such as the PROS kernel and projects such as v5rtmod.

> [!WARNING]
> This is fully unofficial and in no way affiliated, endorsed, supported, or created by VEX Robotics.

## Stability

> [!CAUTION]
> Since `vex_sdk` simply provides bindings over firmware address offsets, no stability guarantees can be made around these APIs. Use this crate at your own risk - these could change any any point in time with a firmware update (although they probably wont in a major way, since this would break existing user programs).

## SDK Coverage

This project aims to cover 100% of the publicly available symbols present in the `v5_api` SDK, as well as any private symbols that have been released to the public (such as `vexTaskAdd`).

Functions present in `libv5rt` that are currently are **NOT** in `vex_sdk`:
- Most things outside of `v5_api.h`and `v5_types.h`. This includes the "user functions", which are basically convenience methods for passing in port indexes directly (e.g. `vexImuReset` vs. `vexDeviceImuReset`).
- Startup code like `vexStartup` and `vexMain` (rust code should handle this instead!), as well as C runtime shims.
- `vexSdkVersion`, since we aren't an official SDK.
- Any C++ `stdlib0`-related functions.
- Most private API symbols*.
- The `vexDeviceAiCamX` family of functions is currently missing bindings, but support is planned.

# Private API Symbols

Many symbols in libv5rt are left *private* (as in, not included in public headers). This is usually done either for stability reasons (VEX not wanting to commit to a stable API yet), or to not expose some lowlevel/debugging functions. In general, this repo tries to avoid exposing anything in the private API. The exceptions to this are functions that have been publicly released by VEX or partner developers for various reasons or demonstrations. This includes the following functions:

- `vexTaskAdd`
- `vexTaskGetCallbackAndId`
- `vexTaskSleep`
- `vexDeviceAdiAddrledSet`
- `vexTasksRun` (known as `vexBackgroundProcessing` in the partner SDK)
- `vexDisplayClipRegionSetWithIndex`

Usage of these APIs in particular is *highly* discouraged, as they could break at any point in time from a firmware update. Use them with caution (or better yet, not at all...)
