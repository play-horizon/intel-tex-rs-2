#[allow(deref_nullptr)]
pub mod bindings {
    use ispc_rt::ispc_module;
    ispc_module!(kernel);
    ispc_module!(kernel_astc);
}

pub mod astc;
pub mod bc1;
pub mod bc3;
pub mod bc4;
pub mod bc5;
pub mod bc6h;
pub mod bc7;
pub mod etc1;

/// Describes a 2D image to block-compress.
#[derive(Debug, Copy, Clone)]
pub struct Surface<'a, const COMPONENTS: usize> {
    /// The pixel data for the image.
    /// The data does not need to be tightly packed: `stride` gives the distance between rows.
    ///
    /// Must be at least `stride * height` bytes.
    pub data: &'a [u8],
    /// The width of the image in texels.
    pub width: u32,
    /// The height of the image in texels.
    pub height: u32,
    /// The stride between the rows of the image, in bytes.
    /// If `data` is tightly packed, this is `width` times the size of a texel (4 bytes for RGBA8,
    /// 8 for the RGBA16F input of BC6H, 2 for BC5, 1 for BC4).
    ///
    /// Must be at least the size of a row, and `stride * height` must fit in an `i32`: the kernels
    /// compute their offsets into `data` with 32-bit integers.
    pub stride: u32,
}

impl<const COMPONENTS: usize> Surface<'_, COMPONENTS> {
    /// Panics unless every row the kernels read, `width` texels of `bytes_per_texel` bytes each,
    /// lies inside `data` at an offset the kernels can address.
    pub(crate) fn assert_readable(&self, bytes_per_texel: u32) {
        let row = u64::from(self.width) * u64::from(bytes_per_texel);
        let size = u64::from(self.height) * u64::from(self.stride);
        assert!(
            u64::from(self.stride) >= row,
            "stride {} is smaller than a row of {row} bytes",
            self.stride
        );
        assert!(
            size <= i32::MAX as u64,
            "stride * height ({size} bytes) doesn't fit in an i32"
        );
        assert!(
            self.data.len() as u64 >= size,
            "data ({} bytes) is smaller than stride * height ({size} bytes)",
            self.data.len()
        );
    }
}

pub type RgbaSurface<'a> = Surface<'a, 4>;
pub type RgSurface<'a> = Surface<'a, 2>;
pub type RSurface<'a> = Surface<'a, 1>;
