#[allow(missing_docs)]
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
/// Tunes the encoder for a certain kind of video.
pub enum Tune {
    None,
    Film,
    Animation,
    Grain,
    StillImage,
    Psnr,
    Ssim,
}

impl Tune {
    #[doc(hidden)]
    pub fn to_cstr(
        self,
        fast_decode: bool,
        zero_latency: bool,
    ) -> *const i8 {
        (if !fast_decode && !zero_latency {
            match self {
                Tune::None       => b"\0" as *const u8,
                Tune::Film       => b"film\0" as *const u8,
                Tune::Animation  => b"animation\0" as *const u8,
                Tune::Grain      => b"grain\0" as *const u8,
                Tune::StillImage => b"stillimage\0" as *const u8,
                Tune::Psnr       => b"psnr\0" as *const u8,
                Tune::Ssim       => b"ssim\0" as *const u8,
            }
        } else if fast_decode && !zero_latency {
            match self {
                Tune::None       => b"fastdecode\0" as *const u8,
                Tune::Film       => b"fastdecode,film\0" as *const u8,
                Tune::Animation  => b"fastdecode,animation\0" as *const u8,
                Tune::Grain      => b"fastdecode,grain\0" as *const u8,
                Tune::StillImage => b"fastdecode,stillimage\0" as *const u8,
                Tune::Psnr       => b"fastdecode,psnr\0" as *const u8,
                Tune::Ssim       => b"fastdecode,ssim\0" as *const u8,
            }
        } else if !fast_decode && zero_latency {
            match self {
                Tune::None       => b"zerolatency\0" as *const u8,
                Tune::Film       => b"zerolatency,film\0" as *const u8,
                Tune::Animation  => b"zerolatency,animation\0" as *const u8,
                Tune::Grain      => b"zerolatency,grain\0" as *const u8,
                Tune::StillImage => b"zerolatency,stillimage\0" as *const u8,
                Tune::Psnr       => b"zerolatency,psnr\0" as *const u8,
                Tune::Ssim       => b"zerolatency,ssim\0" as *const u8,
            }
        } else {
            match self {
                Tune::None =>
                    b"fastdecode,zerolatency\0" as *const u8,
                Tune::Film =>
                    b"fastdecode,zerolatency,film\0" as *const u8,
                Tune::Animation =>
                    b"fastdecode,zerolatency,animation\0" as *const u8,
                Tune::Grain =>
                    b"fastdecode,zerolatency,grain\0" as *const u8,
                Tune::StillImage =>
                    b"fastdecode,zerolatency,stillimage\0" as *const u8,
                Tune::Psnr =>
                    b"fastdecode,zerolatency,psnr\0" as *const u8,
                Tune::Ssim =>
                    b"fastdecode,zerolatency,ssim\0" as *const u8,
            }
        }) as *const i8
    }
}
