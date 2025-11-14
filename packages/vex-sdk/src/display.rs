//! # Brain Display
//! This module contains SDK bindings for interacting with the Brain's display.
//! 
//! ## Coordinates
//! There is a 480 x 272 area accessible to user code, with the origin point of the display at the top left, and the point (479, 271) at the bottom right.
//! 
//! ## Colors
//! All VEXos functions represent RGB colors as a u32, with the format 0x00RRGGBB.
//! Most SDK display functions use either the foreground or background color.
//! The foreground color is `#c0c0ff` by default, and the background color is `#000000` by default; however this value can change based on the code signature:
//! - [`V5_SIG_OPTIONS_INDG`]: Inverts the background color to pure white.
//! - [`V5_SIG_OPTIONS_THDG`]: Sets the background color to pure white if VEXos is using the light theme, and black otherwise.
//! - If both options are set, the themed option ([`V5_SIG_OPTIONS_THDG`]) takes preference.

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
    /// Sets the foreground color for future SDK display calls.
    pub fn vexDisplayForegroundColor(col: u32);
    
    /// Sets the background color for future SDK display calls.
    pub fn vexDisplayBackgroundColor(col: u32);
    
    /// Erases the display.
    pub fn vexDisplayErase();
    
    /// Scrolls the region of the display defined by all pixels with a y-coordinate between `[nStartLine, 272)` up/down by `nLines` pixels. 
    /// The area exposed by the scroll operation is filled with the background color. Pixels outside of the provided region are not affected.
    pub fn vexDisplayScroll(nStartLine: i32, nLines: i32);
    
    /// Scrolls the region of the display defined by all pixels with an x-coordinate in `[x1, x2]` and y-coordinate in `[y1, y2]` up/dwown by `nLines` pixels.
    /// The area exposed by the scroll operation is filled with the background color. Pixels outside of the provided region are not affected.
    pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32);
    
    /// Copies a buffer of pixels provided by `pSrc` with a stride of `srcStride` to the region of the display defined by all pixels with an x-coordinate in `[x1, x2]` and y-coordinate in `[y1, y2]`.
    ///
    /// # Safety
    ///
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - pSrc must point to a buffer of u32s with a length of at least `srcStride * (y2 - y1 + 1)`
    pub fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32);
    
    pub fn vexDisplayPixelSet(x: u32, y: u32);
    
    pub fn vexDisplayPixelClear(x: u32, y: u32);
    
    /// Draws a line from `(x1, y1)` to `(x2, y2)`, using the previously set foreground color.
    pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Erases a line from `(x1, y1)` to `(x2, y2)`, using the previously set background color.
    pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Draws the outline of a rectangle from `(x1, y1)` to `(x2, y2)`, using the previously set foreground color.
    ///
    /// The parameters of this function are inclusive, meaning that the coordinate pairs you specify are inside the rectangle that is created.
    /// Thus, the area of a rectangle created with this set of SDK functions is (1 + x2 - x1) * (1 + y2 - y1) pixels.
    pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Fills the rectangular region from `(x1, y1)` to `(x2, y2)`, using the previously set background color.
    ///
    /// The parameters of this function are inclusive, meaning that the coordinate pairs you specify are inside the rectangle that is created.
    /// Thus, the area of a rectangle created with this set of SDK functions is (1 + x2 - x1) * (1 + y2 - y1) pixels.
    pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Fills the rectangular region from `(x1, y1)` to `(x2, y2)`, using the previously set foreground color.
    ///
    /// The parameters of this function are inclusive, meaning that the coordinate pairs you specify are inside the rectangle that is created.
    /// Thus, the area of a rectangle created with this set of SDK functions is (1 + x2 - x1) * (1 + y2 - y1) pixels.
    pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Draws the outline of a circle using the previously set foreground color.
    ///
    /// Circles are not antialiased.
    pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32);
    
    /// Fills a circle using the previously set background color.
    ///
    /// Circles are not antialiased.
    pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32);
    
    /// Fills a circle using the previously set foreground color.
    ///
    /// Circles are not antialiased.
    pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32);
    
    /// Sets the font size used for future display calls to a fraction `n / d` of the default font size.
    pub fn vexDisplayTextSize(n: u32, d: u32);
    
    /// Sets the font family used for future display calls.
    ///
    /// # Font families
    /// - `"monospace"`: This font at full size is 49pt Noto Mono.
    /// - `"proportional"`: This font at full size is 49pt Noto Sans.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    
    pub fn vexDisplayFontNamedSet(pFontName: *const c_char);
    
    /// Returns the previously set foreground color.
    pub fn vexDisplayForegroundColorGet() -> u32;
    
    /// Returns the previously set background color.
    pub fn vexDisplayBackgroundColorGet() -> u32;
    
    /// Returns the width in pixels of a given string using the currently set font size and family.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32;
    
    /// Returns the height in pixels of a given string using the currently set font size and family.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32;
    
    /// Sets the clip region for all future display calls within the current VEXos thread.
    ///
    /// ALL SDK display calls within a given VEXos thread will only modify pixels within the clip region for the current thread. 
    /// If no clip region is set, display calls can modify the entire screen.
    pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32);
    
    /// Enables double-buffering and renders all previous SDK display calls.
    ///
    /// # Parameters
    /// - `bVsyncWait`: if this is true, the current VEXos thread will be suspended until the display is flushed.
    /// - `bRunScheduler`: if this is true, VEXos scheduler background tasks will be ran while the current thread is suspended.
    pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool);
    
    /// Disables double buffering.
    pub fn vexDisplayDoubleBufferDisable();
    
    /// Sets the clip region for all future display calls for the VEXos thread with the given `index`.
    ///
    /// ALL SDK display calls within a given VEXos thread will only modify pixels within the clip region for the current thread. 
    /// If no clip region is set, display calls can modify the entire screen.
    pub fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32);
    
    pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32;
    
    pub fn vexImagePngRead(
        ibuf: *const u8,
        oBuf: *mut v5_image,
        maxw: u32,
        maxh: u32,
        ibuflen: u32,
    ) -> u32;
}

unsafe extern "C" {
    /// Prints the given format string to the display, with the top-left corner at `(xpos, ypos)`, using the foreground color for the text and the background color for the text background.
    /// If `bOpaque` is 0, the background will be transparent.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the variadic parameters provided must match the format string provided.
    pub unsafe fn vexDisplayPrintf(xpos: i32, ypos: i32, bOpaque: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display at the provided line, using the foreground color for the text and the bacground color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(1, 3)`.
    ///
    /// # Coordinates
    /// A line number of `nLineNumber` maps to a top-left coordinate of `(0, 34 + (20 * nLineNumber))`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the parameters provided must match the format string provided.
    pub unsafe fn vexDisplayString(nLineNumber: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display, with the top-left corner at `(xpos, ypos)`, using the foreground color for the text and the background color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(1, 3)`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the variadic parameters provided must match the format string provided.
    pub unsafe fn vexDisplayStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display at the provided line, using the foreground color for the text and the bacground color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(1, 3)`.
    ///
    /// # Coordinates
    /// A line number of `nLineNumber` maps to a top-left coordinate of `(0, 34 + (20 * nLineNumber))`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the parameters provided must match the format string provided.
    pub unsafe fn vexDisplayBigString(nLineNumber: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display, with the top-left corner at `(xpos, ypos)`, using the foreground color for the text and the background color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(2, 3)`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the variadic parameters provided must match the format string provided.
    pub unsafe fn vexDisplayBigStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display, with the top-left corner at `(xpos, ypos)`, using the foreground color for the text and the background color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(1, 4)`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the variadic parameters provided must match the format string provided.
    pub unsafe fn vexDisplaySmallStringAt(xpos: i32, ypos: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display at the provided line, using the foreground color for the text and the bacground color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(1, 3)`.
    ///
    /// # Coordinates
    /// A line number of `nLineNumber` maps to a y coordinate of `34 + (20 * nLineNumber)`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the parameters provided must match the format string provided.
    pub unsafe fn vexDisplayCenteredString(nLineNumber: i32, format: *const c_char, ...);
    
    /// Prints the given format string to the display at the provided line, using the foreground color for the text and the bacground color for the text background.
    /// If `bOpaque` is 0, the background will be transparent. The font size is preset to `vexDisplayTextSize(2, 3)`.
    ///
    /// # Coordinates
    /// A line number of `nLineNumber` maps to a y coordinate of `34 + (20 * nLineNumber)`.
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `pString` must point to a null-terminated C-string.
    /// - The types of the parameters provided must match the format string provided.
    pub unsafe fn vexDisplayBigCenteredString(nLineNumber: i32, format: *const c_char, ...);
}
