#[allow(missing_docs)]
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
/// An encoder preset, which should handle most of the configuration.
pub enum Preset {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
    Placebo,
}

impl Preset {
    #[doc(hidden)]
    pub fn to_cstr(self) -> *const i8 {
        use self::Preset::*;

        (match self {
            Ultrafast => b"ultrafast\0" as *const u8,
            Superfast => b"superfast\0" as *const u8,
            Veryfast  => b"veryfast\0" as *const u8,
            Faster    => b"faster\0" as *const u8,
            Fast      => b"fast\0" as *const u8,
            Medium    => b"medium\0" as *const u8,
            Slow      => b"slow\0" as *const u8,
            Slower    => b"slower\0" as *const u8,
            Veryslow  => b"veryslow\0" as *const u8,
            Placebo   => b"placebo\0" as *const u8,
        }) as *const i8
    }
}
