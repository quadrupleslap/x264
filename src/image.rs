use {Colorspace, Encoding};
use std::marker::PhantomData;
use std::ptr;
use x264::*;

//TODO: Should I really be using `i32` for everything?

/// Input image data to be given to the encoder.
pub struct Image<'a> {
    raw: x264_image_t,
    width: i32,
    height: i32,
    spooky: PhantomData<&'a [u8]>,
}

impl<'a> Image<'a> {
    /// Makes an I420 (YUV 4:2:0) image.
    ///
    /// # Panics
    ///
    /// Panics if the width and height are not both even, or if the image
    /// length is not correct - there must be 12 bits per pixel.
    pub fn i420(w: i32, h: i32, img: &'a [u8]) -> Self {
        let half = w / 2;
        let size = w as usize * h as usize;
        let brek = size + size / 4;

        assert!(w % 2 == 0 && h % 2 == 0);
        assert_eq!(size * 3/2, img.len());

        unsafe {
            Self::new(Encoding::I420, w, h, &[
                Plane { stride: w, plane: &img[..size] },
                Plane { stride: half, plane: &img[size..brek] },
                Plane { stride: half, plane: &img[brek..] },
            ])
        }
    }

    /// Makes an I422 (YUV 4:2:2) image.
    ///
    /// # Panics
    ///
    /// Panics if the width is not even, or if the image length is not
    /// correct - there must be 16 bits per pixel.
    pub fn i422(w: i32, h: i32, img: &'a [u8]) -> Self {
        let half = w / 2;
        let size = w as usize * h as usize;
        let brek = size + size / 2;

        assert!(w % 2 == 0);
        assert_eq!(2 * size, img.len());

        unsafe {
            Self::new(Encoding::I422, w, h, &[
                Plane { stride: w, plane: &img[..size] },
                Plane { stride: half, plane: &img[size..brek] },
                Plane { stride: half, plane: &img[brek..] },
            ])
        }
    }

    /// Makes an I444 (YUV 4:4:4) image.
    ///
    /// # Panics
    ///
    /// Panics if the image length is incorrect - there must be 24 bits per
    /// pixel.
    pub fn i444(w: i32, h: i32, img: &'a [u8]) -> Self {
        let size = w as usize * h as usize;
        let brek = 2 * size;

        assert_eq!(3 * size, img.len());

        unsafe {
            Self::new(Encoding::I444, w, h, &[
                Plane { stride: w, plane: &img[..size] },
                Plane { stride: w, plane: &img[size..brek] },
                Plane { stride: w, plane: &img[brek..] },
            ])
        }
    }

    /// Makes a BGR image.
    ///
    /// # Panics
    ///
    /// Panics if the image length is incorrect - there must be 24 bits per
    /// pixel.
    pub fn bgr(w: i32, h: i32, img: &'a [u8]) -> Self {
        let size = w as usize * h as usize;
        let stride = 3 * w;

        assert_eq!(3 * size, img.len());

        unsafe {
            Self::new(Encoding::BGR, w, h, &[
                Plane { stride, plane: img },
            ])
        }
    }

    /// Makes a BGRA image.
    ///
    /// # Panics
    ///
    /// Panics if the image length is incorrect - there must be 32 bits per
    /// pixel.
    pub fn bgra(w: i32, h: i32, img: &'a [u8]) -> Self {
        let size = w as usize * h as usize;
        let stride = 4 * w;

        assert_eq!(4 * size, img.len());

        unsafe {
            Self::new(Encoding::BGRA, w, h, &[
                Plane { stride, plane: img },
            ])
        }
    }

    /// Makes a RGB image.
    ///
    /// # Panics
    ///
    /// Panics if the image length is incorrect - there must be 24 bits per
    /// pixel.
    pub fn rgb(w: i32, h: i32, img: &'a [u8]) -> Self {
        let size = w as usize * h as usize;
        let stride = 3 * w;

        assert_eq!(3 * size, img.len());

        unsafe {
            Self::new(Encoding::RGB, w, h, &[
                Plane { stride, plane: img },
            ])
        }
    }

    // Raw

    /// Makes a new image with the given planes and colorspace.
    ///
    /// # Unsafety
    ///
    /// The caller must ensure that there are no more than than 4 planes,
    /// and that the number and size of each plane is appropriate for the
    /// given colorspace.
    pub unsafe fn new<'b, C: Into<Colorspace>>(
        csp: C,
        width: i32,
        height: i32,
        planes: &'b [Plane<'a>],
    ) -> Self {
        //TODO: Can x264 mutate planes?

        let mut strides = [0; 4];
        let mut pointers = [ptr::null_mut(); 4];

        for (i, &Plane { stride, plane }) in planes.iter().enumerate() {
            strides[i] = stride;
            pointers[i] = plane.as_ptr() as *mut u8;
        }

        let raw = x264_image_t {
            i_csp: csp.into().into(),
            i_plane: planes.len() as i32,
            i_stride: strides,
            plane: pointers,
        };

        Self { raw, width, height, spooky: PhantomData }
    }

    // Getters

    /// The width of the image.
    pub fn width(&self) -> i32 { self.width }
    /// The height of the image.
    pub fn height(&self) -> i32 { self.height }
    /// The colorspace of the image.
    pub fn colorspace(&self) -> i32 { self.raw.i_csp }

    #[doc(hidden)]
    pub fn raw(&self) -> x264_image_t { self.raw }
}

/// A single plane of an image.
pub struct Plane<'a> {
    /// The plane's stride (the number of bytes for each row).
    pub stride: i32,
    /// The plane's pixel data.
    pub plane: &'a [u8],
}
