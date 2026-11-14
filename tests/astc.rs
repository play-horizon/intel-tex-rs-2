// The ASTC kernel calls into `ispc_texcomp_astc`, which is C++, so this test only links if the
// C++ standard library is linked too. A program that never calls the ASTC kernel can still link
// it, depending on how its code is split into codegen units.

#[test]
fn astc_kernel_links() {
    let program_count = unsafe { intel_tex_2::bindings::kernel_astc::get_programCount() };

    assert!(program_count > 0);
}

// The surface checks run before the ASTC kernels, so they can be tested even though
// `astc::compress_blocks` is unimplemented.

fn compress_8x8(data: &[u8], stride: u32) {
    let surface = intel_tex_2::RgbaSurface {
        data,
        width: 8,
        height: 8,
        stride,
    };
    intel_tex_2::astc::compress_blocks(&intel_tex_2::astc::alpha_fast_settings(4, 4), &surface);
}

#[test]
#[should_panic(expected = "is smaller than a row")]
fn stride_smaller_than_a_row_panics() {
    compress_8x8(&[0; 31 * 8], 31);
}

#[test]
#[should_panic(expected = "doesn't fit in an i32")]
fn size_beyond_i32_panics() {
    compress_8x8(&[0; 32 * 8], (i32::MAX as u32) / 8 + 1);
}

#[test]
#[should_panic(expected = "is smaller than stride * height")]
fn too_little_input_data_panics() {
    compress_8x8(&[0; 32 * 8 - 1], 32);
}
