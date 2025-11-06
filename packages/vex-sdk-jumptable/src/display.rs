//! Brain Display

use core::ffi::{c_char, VaList};

pub use vex_sdk::v5_image;

use crate::map_jump_table;

const HEADER_HEIGHT: u32 = 32;

map_jump_table! {
    0x640 =>
        /// Sets the color (encoded as RGB8) used for all future non-erasing display draws.
        pub fn vexDisplayForegroundColor(col: u32),
    0x644 =>
        /// Sets the color (encoded as RGB8) used for all future erasing display draws.
        pub fn vexDisplayBackgroundColor(col: u32),
    0x648 =>
        /// Fills the entire framebuffer with the current background color.
        pub fn vexDisplayErase(),
    0x64c =>
        /// Moves a region of the screen defined by all the pixels whose y-axis coordinate are
        /// within the range [nStartLine, 272) `nLines` pixels upwards, without affecting portions
        /// of the screen outside the specified scroll region.
        ///
        /// Since `nLines` is a signed integer, a negative value will move the pixels in the
        /// region downwards instead. Pixels that move outside the region being scrolled are
        /// discarded, and any portions of the region that no longer have a value after the
        /// operation are set to the background color.
        pub fn vexDisplayScroll(nStartLine: i32, nLines: i32),
    0x6a8 => pub fn vexDisplayTextSize(n: u32, d: u32),
    0x6b4 => pub fn vexDisplayFontNamedSet(pFontName: *const c_char),
    0x6b8 =>
        /// Gets the currently set foreground color as an RGB8 color.
        pub fn vexDisplayForegroundColorGet() -> u32,
    0x6bc =>
        /// Gets the currently set background color as an RGB8 color.
        pub fn vexDisplayBackgroundColorGet() -> u32,
    0x6c0 =>
        /// Returns the calculated width (in pixels) of a string if it were to be drawn to the display.
        ///
        /// This function uses the text size of the last text drawing operation for calculating width.
        pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32,
    0x6c4 =>
        /// Returns the calculated height (in pixels) of a string if it were to be drawn to the display.
        ///
        /// This function uses the text size of the last text drawing operation for calculating height.
        pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32,
    0x7a0 =>
        /// Enables double-buffered mode on the display, flushing the intermediate framebuffer.
        ///
        /// The first time this function is called, double-buffered mode will be enabled. In order for future draws to be
        /// seen, this function will need to be called each frame to draw the secondary buffer to the display.
        ///
        /// To re-enable immediate-mode rendering (single-buffer), see [`vexDisplayDoubleBufferDisable`].
        ///
        /// # Arguments
        ///
        /// - `bVsyncWait`: Sleep the current task until the screen is ready to refresh.
        /// - `bRunScheduler`: Call [`vexTasksRun`](crate::task::vexTasksRun) while waiting for a refresh.
        pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool),
    0x7a4 =>
        /// Disables double-buffered mode, switching back to immediate mode rendering.
        pub fn vexDisplayDoubleBufferDisable(),
    0x990 =>
        /// Decodes a bitmap-encoded image passed to `ibuf` into a buffer of pixels that can be drawn to the display.
        ///
        /// # Arguments
        ///
        /// - `ibuf`: The PNG file as a buffer of bytes.
        /// - `obuf`: A decoded image encoded as RGB8 pixels that will be written to if the operation succeeds.
        /// - `maxw`: Width capacity of the image buffer.
        /// - `maxh`: Height capacity of the image buffer.
        ///
        /// # Return
        ///
        /// `1` if the operation is successful, `0` if it failed.
        ///
        /// # Safety
        ///
        /// - `oBuf` must point to an initialized [`v5_image`] struct or null.
        /// - `(*oBuf).data` must point to a mutable allocated image buffer that is at least `maxw * maxh * 4` bytes long or be null.
        pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32,
    0x994 =>
        /// Decodes a PNG file passed to `ibuf` into a buffer of pixels that can be drawn to the display. This function uses
        /// `libpng` internally to decode the file's contents.
        ///
        /// # Arguments
        ///
        /// - `ibuf`: The PNG file as a buffer of bytes.
        /// - `obuf`: A decoded image encoded as RGB8 pixels that will be written to if the operation succeeds.
        /// - `maxw`: Width capacity of the image buffer.
        /// - `maxh`: Height capacity of the image buffer.
        /// - `ibuflen`: Length of the input buffer.
        ///
        /// # Return
        ///
        /// `1` if the operation is successful, `0` if it failed.
        ///
        /// # Safety
        ///
        /// - `ibuf` must be null, OR point to a buffer of at least length ibuflen.
        /// - `oBuf` must point to an initialized [`v5_image`] struct or null.
        /// - `(*oBuf).data` must point to a mutable allocated image buffer that is at least `maxw * maxh * 4` bytes long or be null.
        pub fn vexImagePngRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32, ibuflen: u32) -> u32,
    0x684 =>
        /// Draws a string of text to the display at a given line.
        ///
        /// Uses the current foreground color for the text itself, and the current background color if `bOpaque` is `true`.
        pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x68c =>
        /// Draws a string of large-sized text to the display at a given line.
        ///
        /// Uses the current foreground color as the text color.
        pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x694 =>
        /// Draws a string of center-justified text to the display at a given line.
        ///
        /// Uses the current foreground color as the text color.
        pub fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList),
    0x698 =>
        /// Draws a string of large-sized, center-justified text to the display at a given line.
        ///
        /// Uses the current foreground color as the text color.
        pub fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList),
}

/// Moves a rectangular region of the screen `nLines` pixels upwards, without affecting
/// portions of the screen outside the specified scroll region.
///
/// Since `nLine` is a signed integer, a negative value will move the pixels in the
/// region downwards instead. Pixels that move outside the region being scrolled are
/// discarded, and any portions of the region that no longer have a value after the
/// operation are set to the background color.
///
/// # Bugs
///
/// It appears that this function is somewhat bugged at the time of writing (on VEXos 1.1.4),
/// as it will overwrite one too many lines, setting the bottommost row of scroll data to the
/// background color.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayScrollRect(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    nLines: i32,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x650) as *const extern "system" fn(i32, i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
            nLines,
        )
    }
}

/// Draw a buffer of pixels to a rectangular region of the screen.
///
/// Each u32 element in the buffer is considered a pixel and is parsed in the same format
/// used by vexDisplayForegroundColor (RGB8). The function allows you to specify the region
/// to write to, the pointer to your image buffer, and the stride (or the number of u32
/// pixels in your buffer per 1 row).
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayCopyRect(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    pSrc: *mut u32,
    srcStride: i32,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x654)
            as *const extern "system" fn(i32, i32, i32, i32, *mut u32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
            pSrc,
            srcStride,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayPixelSet(x: u32, y: u32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x658) as *const extern "system" fn(u32, u32)))(
            x,
            y + HEADER_HEIGHT,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayPixelClear(x: u32, y: u32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x65c) as *const extern "system" fn(u32, u32)))(
            x,
            y + HEADER_HEIGHT,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x660) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x664) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x668) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x66c) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x670) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x674) as *const extern "system" fn(i32, i32, i32)))(
            xc,
            yc + (HEADER_HEIGHT as i32),
            radius,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x678) as *const extern "system" fn(i32, i32, i32)))(
            xc,
            yc + (HEADER_HEIGHT as i32),
            radius,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x67c) as *const extern "system" fn(i32, i32, i32)))(
            xc,
            yc + (HEADER_HEIGHT as i32),
            radius,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x794) as *const extern "system" fn(i32, i32, i32, i32)))(
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayClipRegionSetWithIndex(
    index: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x7a8) as *const extern "system" fn(i32, i32, i32, i32, i32)))(
            index,
            x1,
            y1 + (HEADER_HEIGHT as i32),
            x2,
            y2 + (HEADER_HEIGHT as i32),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayVPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    args: VaList,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x680)
            as *const extern "system" fn(i32, i32, i32, *const c_char, VaList)))(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            bOpaque,
            format,
            args,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayVStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x688)
            as *const extern "system" fn(i32, i32, *const c_char, VaList)))(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayVBigStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x690)
            as *const extern "system" fn(i32, i32, *const c_char, VaList)))(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDisplayVSmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList,
) {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x6b0)
            as *const extern "system" fn(i32, i32, *const c_char, VaList)))(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplayPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe {
        vexDisplayVPrintf(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            bOpaque,
            format,
            args.as_va_list(),
        )
    }
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
    unsafe {
        vexDisplayVStringAt(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args.as_va_list(),
        )
    }
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
    unsafe {
        vexDisplayVBigStringAt(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args.as_va_list(),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDisplaySmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    mut args: ...
) {
    unsafe {
        vexDisplayVSmallStringAt(
            xpos,
            ypos + (HEADER_HEIGHT as i32),
            format,
            args.as_va_list(),
        )
    }
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
