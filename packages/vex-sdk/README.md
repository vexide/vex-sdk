# vex-sdk

This crate contains raw C-level bindings to VEX's system APIs.

You can use this crate by adding it to your project's Cargo.toml with `cargo add vex-sdk`.

## Linkage

> [!NOTE]
> If you are using a higher-level library such as [vexide](https://crates.io/crates/vexide), this is likely already handled for you!

`vex-sdk` provides declarations for functions included in VEX's platform SDKs, but doesn't link to any runtime library itself. Your project will need an implementation of these functions to compile properly.

One option is to depend on [`vex-sdk-jumptable`](https://crates.io/crates/vex-sdk-jumptable), an open-source reimplementation of VEX's runtime library, then bring it into scope in your project:

```rs
// Bring runtime library into scope
use vex_sdk_jumptable as _;

fn main() { /* ... */ }
```

Alternatively, you may wish to provide your own SDK or link to an official SDK distributed by VEX. To do this, use a build script to add the SDK into your link search path, then link to `libv5rt.a` or the equivalent for your platform. There are several other crates which implement this process to allow you to link with external SDKs, including:

- [`vex-sdk-vexcode`](https://crates.io/crates/vex-sdk-vexcode), a build-time crate that downloads and links a copy of VEX's proprietary VEXcode SDK
- [`vex-sdk-pros`](https://crates.io/crates/vex-sdk-pros), which links to the VEX Partner SDK via the PROS kernel

For more information, see the [Rust target documentation](https://doc.rust-lang.org/stable/rustc/platform-support/thumbv7a-vex-v5.html) for `thumbv7a-vex-v5`.
