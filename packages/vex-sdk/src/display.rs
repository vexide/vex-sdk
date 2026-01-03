//! Brain Display

use core::ffi::c_char;

/// A decoded image written to by VEXos.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct v5_image {
    /// Definitive width of the output image.
    pub width: u16,

    /// Definitive height of the output image.
    pub height: u16,

    /// Buffer of RGB8 pixels that containing the image's data.
    ///
    /// This field must be set before the read operation as a pointer to the pre-allocated pixel buffer.
    /// After an image read operation, said image’s pixels are written to the location specified by this field.
    pub data: *mut u32,

    /// Points to the first pixel of the second row in the pixel buffer.
    ///
    /// Only set by the SDK after a [`vexImageBmpRead`] call.
    pub p: *mut u32,
}

unsafe extern "system" {
    pub safe fn vexDisplayForegroundColor(col: u32);
    pub safe fn vexDisplayBackgroundColor(col: u32);
    pub safe fn vexDisplayErase();
    pub safe fn vexDisplayScroll(nStartLine: i32, nLines: i32);
    pub safe fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32);
    pub unsafe fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32);
    pub safe fn vexDisplayPixelSet(x: u32, y: u32);
    pub safe fn vexDisplayPixelClear(x: u32, y: u32);
    pub safe fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32);
    pub safe fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32);
    pub safe fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32);
    pub safe fn vexDisplayTextSize(n: u32, d: u32);
    pub unsafe fn vexDisplayFontNamedSet(pFontName: *const c_char);
    pub safe fn vexDisplayForegroundColorGet() -> u32;
    pub safe fn vexDisplayBackgroundColorGet() -> u32;
    pub unsafe fn vexDisplayStringWidthGet(pString: *const c_char) -> i32;
    pub unsafe fn vexDisplayStringHeightGet(pString: *const c_char) -> i32;
    pub safe fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32);
    pub safe fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool);
    pub safe fn vexDisplayDoubleBufferDisable();
    pub safe fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32);
    pub unsafe fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32;
    pub unsafe fn vexImagePngRead(
        ibuf: *const u8,
        oBuf: *mut v5_image,
        maxw: u32,
        maxh: u32,
        ibuflen: u32,
    ) -> u32;
}

unsafe extern "C" {
    pub unsafe fn vexDisplayPrintf(xpos: i32, ypos: i32, bOpaque: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayString(nLineNumber: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayBigString(nLineNumber: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayBigStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplaySmallStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayCenteredString(nLineNumber: i32, format: *const c_char, ...);
    pub unsafe fn vexDisplayBigCenteredString(nLineNumber: i32, format: *const c_char, ...);
}
