#![no_std]

#[cfg(not(target_os = "vexos"))]
compile_error!("`vex-sdk-jumptable` does not support this target.");

/// vexTasksRun is aliased to vexBackgroundProcessing on the partner SDK.
#[unsafe(no_mangle)]
unsafe extern "C" fn vexTasksRun() {
    #[link(name = "pros")]
    unsafe extern "C" {
        fn vexBackgroundProcessing();
    }

    unsafe {
        vexBackgroundProcessing()
    }
}
