//! Brain Display

use core::ffi::{VaList, c_char};

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

    /// Sets the color (encoded as RGB8) used for all future non-erasing display draws.
    pub fn vexDisplayForegroundColor(col: u32);

    /// Sets the color (encoded as RGB8) used for all future erasing display draws.
    pub fn vexDisplayBackgroundColor(col: u32);

    /// Fills the entire framebuffer with the current background color.
    pub fn vexDisplayErase();

    /// Moves a region of the screen defined by all the pixels whose y-axis coordinate are
    /// within the range [nStartLine, 272) `nLines` pixels upwards, without affecting portions
    /// of the screen outside the specified scroll region.
    ///
    /// Since `nLines` is a signed integer, a negative value will move the pixels in the
    /// region downwards instead. Pixels that move outside the region being scrolled are
    /// discarded, and any portions of the region that no longer have a value after the
    /// operation are set to the background color.
    pub fn vexDisplayScroll(nStartLine: i32, nLines: i32);

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
    pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32);

    /// Draw a buffer of pixels to a rectangular region of the screen.
    ///
    /// Each u32 element in the buffer is considered a pixel and is parsed in the same format used by
    /// vexDisplayForegroundColor (RGB8). The function allows you to specify the region to write to, the
    /// pointer to your image buffer, and the stride (or the number of u32 pixels in your buffer per 1 row).
    pub fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32);

    /// Fills a given pixel of the screen with the current foreground color.
    pub fn vexDisplayPixelSet(x: u32, y: u32);

    /// Fills a given pixel of the screen with the current background color.
    pub fn vexDisplayPixelClear(x: u32, y: u32);

    /// Draws a one-pixel wide stroke line between two points with the current foreground color.
    pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Draws a one-pixel wide stroke line between two points with the current background color.
    pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Strokes a one-pixel wide rectangular region of the screen with the current foreground color.
    pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Fills a rectangular region of the screen with the current background color.
    pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Fills rectangular region of the screen with the current foreground color.
    pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Strokes a one-pixel wide circle defined by a center-point and a radius with the current foreground color.
    pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32);

    /// Fills a circular region of the screen with the current background color.
    pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32);

    /// Fills a circular region of the screen with the current foreground color.
    pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32);
    pub fn vexDisplayTextSize(n: u32, d: u32);
    pub fn vexDisplayFontNamedSet(pFontName: *const c_char);

    /// Gets the currently set foreground color as an RGB8 color.
    pub fn vexDisplayForegroundColorGet() -> u32;

    /// Gets the currently set background color as an RGB8 color.
    pub fn vexDisplayBackgroundColorGet() -> u32;

    /// Returns the calculated width (in pixels) of a string if it were to be drawn to the display.
    ///
    /// This function uses the text size of the last text drawing operation for calculating width.
    pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32;

    /// Returns the calculated height (in pixels) of a string if it were to be drawn to the display.
    ///
    /// This function uses the text size of the last text drawing operation for calculating height.
    pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32;

    /// Sets a rectangular region of the display's framebuffer that the current task is allowed to modify.
    ///
    /// When set, any draws to the display made by the calling task outside of its defined clip region will not be drawn.
    pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32);

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
    pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool);

    /// Disables double-buffered mode, switching back to immediate mode rendering.
    pub fn vexDisplayDoubleBufferDisable();

    /// Sets a rectangular region of the display's framebuffer that the a given task index is allowed to modify.
    ///
    /// When set, any draws to the display made by the target task outside of its defined clip region will not be drawn.
    ///
    /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L40>
    pub fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32);

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
    pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32;

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
    pub fn vexImagePngRead(
        ibuf: *const u8,
        oBuf: *mut v5_image,
        maxw: u32,
        maxh: u32,
        ibuflen: u32,
    ) -> u32;

    /// Draws a string of text to the display at a given top-left coordinate.
    ///
    /// Uses the current foreground color for the text itself, and the current background color if `bOpaque` is `true`.
    pub fn vexDisplayVPrintf(
        xpos: i32,
        ypos: i32,
        bOpaque: i32,
        format: *const c_char,
        args: VaList,
    );

    /// Draws a string of text to the display at a given line.
    ///
    /// Uses the current foreground color for the text itself, and the current background color if `bOpaque` is `true`.
    pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList);

    /// Draws a string of text to the display at a given top-left coordinate.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList);

    /// Draws a string of large-sized text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList);

    /// Draws a string of large-sized text to the display at a top-left coordinate.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVBigStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList);

    /// Draws a string of small-sized text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVSmallStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList);

    /// Draws a string of center-justified text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList);

    /// Draws a string of large-sized, center-justified text to the display at a given line.
    ///
    /// Uses the current foreground color as the text color.
    pub fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList);
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
