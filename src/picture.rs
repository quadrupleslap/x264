use x264::*;

/// Output information about an encoded frame.
pub struct Picture {
    raw: x264_picture_t
}

impl Picture {
    /// Whether the picture is a keyframe.
    pub fn keyframe(&self) -> bool {
        self.raw.b_keyframe != 0
    }

    /// The presentation timestamp.
    pub fn pts(&self) -> i64 {
        self.raw.i_pts
    }

    /// The decoding timestamp.
    pub fn dts(&self) -> i64 {
        self.raw.i_dts
    }

    #[doc(hidden)]
    pub unsafe fn from_raw(raw: x264_picture_t) -> Self {
        Self { raw }
    }
}
