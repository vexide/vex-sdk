# vex_sdk

Raw bindings to vexos user jump table functions.

This repository serves as a partial open-source reimplementation of VEX's V5 runtime SDK (libv5rt.a), but doesn't necessarily aim to be API or ABI compatible. The intent of this crate is to be used as building blocks for higher-level rust tooling to run on the V5 brain. 

- Jumptable and firmware address offsets are derived from [cetio's VEXAPI research repository](https://github.com/cetio/VEXAPI).
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
- `vexSystemVersion`, `vexStdlibVersion`, `vexSdkVersion`, `vexStdlibVersionLinked`, and `vexStdlibVersionVerify` since these aren't actual functions on the jumptable, just data at addresses. Support might be added in the future, though.
- Most private API symbols*.
- The `vexDeviceAiCamX` family of functions is currently missing bindings, but support is planned.

# Private API Symbols

Many symbols in libv5rt are left *private* (as in, not included in public headers). This is usually done either for stability reasons (VEX not wanting to commit to a stable API yet), or to not expose some lowlevel/debugging functions. In general, this repo tries to avoid exposing anything in the private API. The exceptions to this are:
	- Functions that have been publicly released by VEX employees for various reason, such as `vexDeviceAdiAddrLedSet` or `vexTaskAdd`.
	- Function signatures are obviously guessable and harmless to competition integrity. For example, most smart devices have a `vexDeviceXTemperatureGet` function that always has the function signature `fn(V5_DeviceT) -> c_double`. For some reason, some of these functions are left private (such as `vexDeviceImuTemperatureGet`) while others are public (such as `vexDeviceAiVisionTemperatureGet`). This also includes status code getters which are always `fn(V5_DeviceT) -> u32`.
