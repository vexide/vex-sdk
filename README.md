# vex-sdk

Rust bindings to VEXos system APIs.

This repository provides libraries for interacting with low-level VEXos system APIs from Rust. These crates are intended to to serve as a foundation for building higher level VEX libraries, as well as an effort to document VEXos system functions.

## Contents

- [`vex-sdk`]: FFI bindings to VEX's platform SDKs.
- [`vex-sdk-build`]: A build script helper to download and link to official SDKs from VEX.
- [`vex-sdk-jumptable`]: Open-source implementation of VEXos system APIs using firmware jump addresses.
- [`vex-sdk-mock`]: A stubbed implementation of the VEX SDK for testing programs using the `vex-sdk` crate on non-VEXos targets.

[`vex-sdk`]: ./packages/vex-sdk
[`vex-sdk-build`]: ./packages/vex-sdk-build
[`vex-sdk-jumptable`]: ./packages/vex-sdk-jumptable
[`vex-sdk-mock`]: ./packages/vex-sdk-mock
