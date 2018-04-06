use core::marker::PhantomData;
use core::slice;
use x264::*;

//TODO: Iterator over the units.

/// The encoded data, to be used in chunks or in its entirety.
pub struct Data<'a> {
    ptr: *mut x264_nal_t,
    len: usize,
    spooky: PhantomData<&'a [x264_nal_t]>
}

impl<'a> Data<'a> {
    #[doc(hidden)]
    pub unsafe fn from_raw_parts(
        ptr: *mut x264_nal_t,
        len: usize
    ) -> Self {
        Data { ptr, len, spooky: PhantomData }
    }

    /// The length (in NAL units, **not** in bytes) of this data sequence.
    pub fn len(&self) -> usize {
        self.len
    }

    //TODO: Use index trait(s) once IndexMove becomes a thing.

    /// The `i`th unit.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out-of-bounds. In order to be within the bounds,
    /// `i` must be less than `len`.
    pub fn unit(&self, i: usize) -> Unit<'a> {
        const D: i32 = Priority::Disposable as i32;
        const L: i32 = Priority::Low as i32;
        const H: i32 = Priority::High as i32;

        assert!(i < self.len);

        let nal = unsafe {
            *self.ptr.offset(i as isize)
        };

        Unit {
            priority:
                match nal.i_ref_idc {
                    D => Priority::Disposable,
                    L => Priority::Low,
                    H => Priority::High,
                    _ => Priority::Highest,
                },
            payload:
                unsafe {
                    slice::from_raw_parts(
                        nal.p_payload,
                        nal.i_payload as usize
                    )
                }
        }
    }

    /// The entire chunk of data, as one big byte-slice.
    pub fn entirety(&self) -> &[u8] {
        if self.len == 0 {
            &[]
        } else {
            let (a, b) = unsafe {
                let a = *self.ptr;
                let b = *self.ptr.offset((self.len - 1) as isize);
                (a, b)
            };

            let start  = a.p_payload;
            let length = b.p_payload as usize
                       + b.i_payload as usize
                       - start as usize;

            unsafe { slice::from_raw_parts(start, length) }
        }
    }
}

/// A single NAL unit.
pub struct Unit<'a> {
    priority: Priority,
    payload: &'a [u8]
}

impl<'a> Unit<'a> {
    /// How crucial this unit is regarding the decoding of the video.
    pub fn priority(&self) -> Priority {
        self.priority
    }
}

impl<'a> AsRef<[u8]> for Unit<'a> {
    fn as_ref(&self) -> &[u8] {
        self.payload
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(i32)]
/// The importance of a given unit.
pub enum Priority {
    /// Not important at all.
    Disposable = nal_priority_e::NAL_PRIORITY_DISPOSABLE as i32,
    /// Not very important.
    Low = nal_priority_e::NAL_PRIORITY_LOW as i32,
    /// Pretty important.
    High = nal_priority_e::NAL_PRIORITY_HIGH as i32,
    /// Extremely important.
    Highest = nal_priority_e::NAL_PRIORITY_HIGHEST as i32,
}
