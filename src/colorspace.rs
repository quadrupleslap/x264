use x264::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
/// An image encoding.
pub enum Encoding {
    /// A Y plane followed by 2x2 subsampled U and V planes.
    I420 = 0x0001,
    /// A Y plane followed by 2x2 subsampled V and U planes.
    YV12 = 0x0002,
    /// A Y plane followed by a packed 2x2 subsampled UV plane.
    NV12 = 0x0003,
    /// A Y plane followed by a packed 2x2 subsampled VU plane.
    NV21 = 0x0004,
    /// A Y plane followed by 2x1 subsampled U and V planes.
    I422 = 0x0005,
    /// A Y plane followed by 2x1 subsampled V and U planes.
    YV16 = 0x0006,
    /// A Y plane followed by a packed 2x1 subsampled UV plane.
    NV16 = 0x0007,
    /// A packed 32-bit UYVY plane with 10-bit components, where the latter two bits are padding.
    V210 = 0x0008,
    /// A Y plane followed by U and V planes.
    I444 = 0x0009,
    /// A Y plane followed by V and U planes.
    YV24 = 0x000A,
    /// A packed 24-bit BGR plane.
    BGR  = 0x000B,
    /// A packed 32-bit BGR plane, where the latter byte is padding.
    BGRA = 0x000C,
    /// A packed 24-bit RGB plane.
    RGB  = 0x000D,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
/// A colorspace, which consists of an encoding and some options.
pub struct Colorspace {
    raw: i32,
}

impl Colorspace {
    /// Doubles the pixel depth, from 8 to 16 bits per pixel.
    ///
    /// This has no effect for some encodings, such as V210.
    pub fn high_depth(mut self) -> Self {
        self.raw |= X264_CSP_HIGH_DEPTH as i32;
        self
    }

    /// Vertically flips the image.
    pub fn vflip(mut self) -> Self {
        self.raw |= X264_CSP_VFLIP as i32;
        self
    }
}

impl From<Colorspace> for i32 {
    fn from(csp: Colorspace) -> Self {
        csp.raw
    }
}

impl From<Encoding> for Colorspace {
    fn from(encoding: Encoding) -> Self {
        Self { raw: encoding as i32 }
    }
}
