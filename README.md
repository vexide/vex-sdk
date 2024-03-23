# vex_sys

Raw bindings to vexos user jump table functions.

This repository serves as a partial open-source reimplementation of VEX's V5 runtime SDK (libv5rt.a), but doesn't aim to be API or ABI compatible (different naming conventions, etc...) The intent of this crate is to be used as building blocks for higher-level rust tooling to run on the V5 brain. 

- Jumptable address offsets are derived from [cetio's VEXAPI research repository](https://github.com/cetio/VEXAPI).
- Symbols and function signatures are taken from publicly available sources, such as the PROS kernel.

> This is fully unofficial and in no way affiliated, endorsed, supported, or created by VEX Robotics.