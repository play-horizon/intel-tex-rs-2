//! The encoders refuse settings their kernels can't use: a count larger than the array it
//! selects from would make the kernel read and write past that array.

use intel_tex_2::{bc6h, bc7, etc1, RgbaSurface};

/// A 4x4 block of the RGBA16F input of BC6H; the RGBA8 surfaces use its first half.
const DATA: [u8; 4 * 4 * 8] = [0x40; 4 * 4 * 8];

fn rgba8() -> RgbaSurface<'static> {
    RgbaSurface {
        data: &DATA,
        width: 4,
        height: 4,
        stride: 16,
    }
}

fn rgba16f() -> RgbaSurface<'static> {
    RgbaSurface {
        data: &DATA,
        width: 4,
        height: 4,
        stride: 32,
    }
}

#[test]
fn every_preset_and_the_largest_counts_are_accepted() {
    for settings in [
        bc6h::very_fast_settings(),
        bc6h::fast_settings(),
        bc6h::basic_settings(),
        bc6h::slow_settings(),
        bc6h::very_slow_settings(),
        bc6h::EncodeSettings {
            fast_skip_threshold: 32,
            ..bc6h::very_slow_settings()
        },
    ] {
        bc6h::compress_blocks(&settings, &rgba16f());
    }
    for settings in [
        bc7::opaque_ultra_fast_settings(),
        bc7::opaque_very_fast_settings(),
        bc7::opaque_fast_settings(),
        bc7::opaque_basic_settings(),
        bc7::opaque_slow_settings(),
        bc7::alpha_ultra_fast_settings(),
        bc7::alpha_very_fast_settings(),
        bc7::alpha_fast_settings(),
        bc7::alpha_basic_settings(),
        bc7::alpha_slow_settings(),
        bc7::EncodeSettings {
            fast_skip_threshold_mode1: 64,
            fast_skip_threshold_mode3: 64,
            fast_skip_threshold_mode7: 64,
            mode45_channel0: 3,
            ..bc7::alpha_slow_settings()
        },
    ] {
        bc7::compress_blocks(&settings, &rgba8());
    }
    for fast_skip_threshold in [1, etc1::slow_settings().fast_skip_threshold, 165] {
        etc1::compress_blocks(
            &etc1::EncodeSettings {
                fast_skip_threshold,
            },
            &rgba8(),
        );
    }
}

#[test]
#[should_panic(expected = "fast_skip_threshold 33 is larger than the 32 partitions")]
fn bc6h_fast_skip_threshold_above_32_panics() {
    let settings = bc6h::EncodeSettings {
        fast_skip_threshold: 33,
        ..bc6h::basic_settings()
    };
    bc6h::compress_blocks(&settings, &rgba16f());
}

#[test]
#[should_panic(expected = "channels is 5, not 3 or 4")]
fn bc7_channels_above_4_panic() {
    let settings = bc7::EncodeSettings {
        channels: 5,
        ..bc7::alpha_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "channels is 2, not 3 or 4")]
fn bc7_channels_below_3_panic() {
    let settings = bc7::EncodeSettings {
        channels: 2,
        ..bc7::opaque_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "mode45_channel0 4 is not a channel from 0 to 3")]
fn bc7_mode45_channel0_above_3_panics() {
    let settings = bc7::EncodeSettings {
        mode45_channel0: 4,
        ..bc7::alpha_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "fast_skip_threshold_mode1 65 is larger than the 64 partitions")]
fn bc7_fast_skip_threshold_mode1_above_64_panics() {
    let settings = bc7::EncodeSettings {
        fast_skip_threshold_mode1: 65,
        ..bc7::opaque_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "fast_skip_threshold_mode3 65 is larger than the 64 partitions")]
fn bc7_fast_skip_threshold_mode3_above_64_panics() {
    let settings = bc7::EncodeSettings {
        fast_skip_threshold_mode3: 65,
        ..bc7::opaque_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "fast_skip_threshold_mode7 65 is larger than the 64 partitions")]
fn bc7_fast_skip_threshold_mode7_above_64_panics() {
    let settings = bc7::EncodeSettings {
        fast_skip_threshold_mode7: 65,
        ..bc7::alpha_basic_settings()
    };
    bc7::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "fast_skip_threshold 166 is outside 1..=165")]
fn etc1_fast_skip_threshold_above_165_panics() {
    let settings = etc1::EncodeSettings {
        fast_skip_threshold: 166,
    };
    etc1::compress_blocks(&settings, &rgba8());
}

#[test]
#[should_panic(expected = "fast_skip_threshold 0 is outside 1..=165")]
fn etc1_fast_skip_threshold_of_0_panics() {
    let settings = etc1::EncodeSettings {
        fast_skip_threshold: 0,
    };
    etc1::compress_blocks(&settings, &rgba8());
}
