//! Brain Display

use core::ffi::{c_char, VaList};

use crate::map_jump_table;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct v5_image {
    pub width: u16,
    pub height: u16,
    pub data: *mut u32,
    pub p: *mut u32,
}

map_jump_table! {
    0x640 => pub fn vexDisplayForegroundColor(col: u32),
    0x644 => pub fn vexDisplayBackgroundColor(col: u32),
    0x648 => pub fn vexDisplayErase(),
    0x64c => pub fn vexDisplayScroll(nStartLine: i32, nLines: i32),
    0x650 => pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32),
    0x654 => pub fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32),
    0x658 => pub fn vexDisplayPixelSet(x: u32, y: u32),
    0x65c => pub fn vexDisplayPixelClear(x: u32, y: u32),
    0x660 => pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32),
    0x664 => pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32),
    0x668 => pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32),
    0x66c => pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32),
    0x670 => pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32),
    0x674 => pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32),
    0x678 => pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32),
    0x67c => pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32),
    0x6a8 => pub fn vexDisplayTextSize(n: u32, d: u32),
    0x6b4 => pub fn vexDisplayFontNamedSet(pFontName: *const c_char),
    0x6b8 => pub fn vexDisplayForegroundColorGet() -> u32,
    0x6bc => pub fn vexDisplayBackgroundColorGet() -> u32,
    0x6c0 => pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32,
    0x6c4 => pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32,
    0x794 => pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32),
    0x7a0 => pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool),
    0x7a4 => pub fn vexDisplayDoubleBufferDisable(),
    0x7a8 =>
        /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L40>
        pub fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32),
    0x990 => pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32,
    0x994 => pub fn vexImagePngRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32, ibuflen: u32) -> u32,

    0x680 => pub fn vexDisplayVPrintf(xpos: i32, ypos: i32, bOpaque: i32, format: *const c_char, args: VaList),
    0x684 => pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x688 => pub fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList),
    0x68c => pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x690 => pub fn vexDisplayVBigStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList),
    0x6b0 => pub fn vexDisplayVSmallStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList),
    0x694 => pub fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x698 => pub fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList),
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVPrintf(xpos, ypos, bOpaque, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayString(nLineNumber: i32, format: *const c_char, mut args: ...) {
    unsafe { vexDisplayVString(nLineNumber, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayBigString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigString(nLineNumber, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayBigStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplaySmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVSmallStringAt(xpos, ypos, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVCenteredString(nLineNumber, format, args.as_va_list()) }
}

#[no_mangle]
pub unsafe extern "C" fn vexDisplayBigCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe { vexDisplayVBigCenteredString(nLineNumber, format, args.as_va_list()) }
}
