# vex_sys

Raw bindings to vexos user jump table functions.

This repository serves as a partial open-source reimplementation of VEX's V5 runtime SDK (libv5rt.a), but doesn't necessarily aim to be API or ABI compatible. The intent of this crate is to be used as building blocks for higher-level rust tooling to run on the V5 brain. 

- Jumptable and firmware address offsets are derived from [cetio's VEXAPI research repository](https://github.com/cetio/VEXAPI).
- Symbols and function signatures are taken from publicly available sources, such as the PROS kernel and projects such as v5rtmod.

> [!WARNING]
> This is fully unofficial and in no way affiliated, endorsed, supported, or created by VEX Robotics.

## SDK Coverage

This project aims to cover 100% of the publicly available symbols present in the `v5_api` SDK, as well as any private symbols that have been released to the public (such as `vexTaskAdd`).

Functions that currently are not included:
- C-style varadic functions (vex_printf, etc...), or functions taking vararg structures (e.g. `va_list`)
- `vexSystemVersion`, `vexStdlibVersion`, `vexSdkVersion`, `vexStdlibVersionLinked`, and`vexStdlibVersionVerify` since these aren't actual functions on the jumptable, just data at addresses. Support might be added in the future, though.
- Most private API symbols are missing, excluding definitions that are either obvious (e.g. `vexDisplayPenSizeSet`) or made publicly available for various reasons.
- The `vexDeviceAiCamX` family of functions is currently missing bindings, but support is planned.