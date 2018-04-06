use x264::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
/// The colorspace of an image, which details how its colors are represented.
pub enum Colorspace {
    /// A Y plane followed by 2x2 subsampled U and V planes.
    I420 = X264_CSP_I420,
    /// A Y plane followed by 2x2 subsampled V and U planes.
    YV12 = X264_CSP_YV12,
    /// A Y plane followed by a packed 2x2 subsampled UV plane.
    NV12 = X264_CSP_NV12,
    /// A Y plane followed by a packed 2x2 subsampled VU plane.
    NV21 = X264_CSP_NV21,
    /// A Y plane followed by 2x1 subsampled U and V planes.
    I422 = X264_CSP_I422,
    /// A Y plane followed by 2x1 subsampled V and U planes.
    YV16 = X264_CSP_YV16,
    /// A Y plane followed by a packed 2x1 subsampled UV plane.
    NV16 = X264_CSP_NV16,
    /// A single plane whose bytes follow the pattern YUYV pattern, which means
    /// the U and V parts are 2x1 subsampled.
    #[cfg(feature = "yuyv")]
    YUYV = X264_CSP_YUYV,
    /// A single plane whose bytes follow the pattern UYVY pattern, which means
    /// the U and V parts are 2x1 subsampled.
    #[cfg(feature = "yuyv")]
    UYVY = X264_CSP_UYVY,
    /// A packed 32-bit UYVY plane with 10-bit components, and 2 padding bits.
    V210 = X264_CSP_V210,
    /// A Y plane followed by U and V planes.
    I444 = X264_CSP_I444,
    /// A Y plane followed by V and U planes.
    YV24 = X264_CSP_YV24,
    /// A packed 24-bit BGR plane.
    BGR  = X264_CSP_BGR,
    /// A packed 32-bit BGR plane, where the latter byte is padding.
    BGRA = X264_CSP_BGRA,
    /// A packed 24-bit RGB plane.
    RGB  = X264_CSP_RGB,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
/// The image's colorspace plus some extra encoding options.
pub struct Encoding {
    raw: i32,
}

impl Encoding {
    /// Add an encoding option.
    pub fn add(mut self, modifier: Modifier) -> Self {
        self.raw |= modifier as i32;
        self
    }

    /// Remove an encoding option.
    pub fn remove(mut self, modifier: Modifier) -> Self {
        self.raw &= !(modifier as i32);
        self
    }

    /// Check if an encoding option has been set.
    pub fn has(self, modifier: Modifier) -> bool {
        self.raw & modifier as i32 != 0
    }

    /// Gets the colorspace of the encoding.
    pub fn colorspace(self) -> Colorspace {
        use core::mem;
        unsafe { mem::transmute(self.raw as u32 % X264_CSP_MAX) }
    }

    #[doc(hidden)]
    pub fn into_raw(self) -> i32 {
        self.raw
    }

    #[doc(hidden)]
    pub unsafe fn from_raw(raw: i32) -> Self {
        Self { raw }
    }
}

impl From<Colorspace> for Encoding {
    fn from(csp: Colorspace) -> Self {
        Self { raw: csp as i32 }
    }
}

#[repr(i32)]
/// Some extra encoding options.
pub enum Modifier {
    /// Doubles the pixel depth, from 8 to 16 bits per pixel.
    HighDepth = X264_CSP_HIGH_DEPTH as i32,
    /// Vertically flips the image.
    VerticalFlip = X264_CSP_VFLIP as i32,
}
