//! Brain Display

use core::ffi::{VaList, c_char};

pub use vex_sdk::v5_image;

#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayForegroundColor(col: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayBackgroundColor(col: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayErase() {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayScroll(nStartLine: i32, nLines: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayCopyRect(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    pSrc: *mut u32,
    srcStride: i32,
) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayPixelSet(x: u32, y: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayPixelClear(x: u32, y: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayTextSize(n: u32, d: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayFontNamedSet(pFontName: *const c_char) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayForegroundColorGet() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayBackgroundColorGet() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayStringWidthGet(pString: *const c_char) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayStringHeightGet(pString: *const c_char) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayPenSizeSet(width: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayPenSizeGet() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayDoubleBufferDisable() {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexImageBmpRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexImagePngRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
    ibuflen: u32,
) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    mut args: VaList<'_, '_>,
) {
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVBigString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVBigStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVSmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayVBigCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVPrintf(xpos, ypos, bOpaque, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayString(nLineNumber: i32, format: *const c_char, mut args: ...) {
    unsafe { vexDisplayVString(nLineNumber, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayBigString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigString(nLineNumber, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayBigStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplaySmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVSmallStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVCenteredString(nLineNumber, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayBigCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigCenteredString(nLineNumber, format, args.as_va_list()) }
}
