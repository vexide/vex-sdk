# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each crate in this workspace is versioned independently, so releases are grouped
by crate below.

## [Unreleased]

## [2026-07-28] `vex-sdk` 0.29.0-rc.1

#### Changed

- `vexDeviceGenericValueGet` now returns `i32` rather than `c_double`, matching what VEXos actually returns. (**Breaking change**)

### `vex-sdk-jumptable` 0.2.0-rc.1

#### Changed

- `vexDeviceGenericValueGet` now returns `i32` rather than `c_double`, and is documented as returning `0` for devices VEXos does not treat as generic sensors. (**Breaking change**)
- Raised the minimum supported Rust version to 1.94.

### `vex-sdk-mock` 0.2.0-rc.1

#### Changed

- `vexDeviceGenericValueGet` now returns `i32` rather than `c_double`. (**Breaking change**)
- Raised the minimum supported Rust version to 1.94.

#### Fixed

- Added missing `no_mangle` attributes to several stubs in the device, display, distance, and system modules. Without them the symbols were not exported and linking a program against the mock could fail.

### `vex-sdk-pros` 0.1.0-rc.1

Moved off `0.0.x` versioning; there are no breaking changes relative to 0.0.1.

#### Fixed

- The unsupported-target `compile_error!` now names `vex-sdk-pros` instead of `vex-sdk-jumptable`.

### `vex-sdk-vexcode` 0.1.0-rc.1

#### Changed

- Updated `zip` from 4.5 to 8.6.
- Raised the minimum supported Rust version to 1.88.
