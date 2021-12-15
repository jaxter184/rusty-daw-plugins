#![cfg_attr(feature = "portable-simd", feature(portable_simd))]

#[cfg(feature = "portable-simd")]
use std::simd::{f32x2, LaneCount, Simd, SupportedLaneCount};

use rusty_daw_core::{
    ParamF32, ParamF32UiHandle, SampleRate, Unit, Gradient, DEFAULT_SMOOTH_SECS,
};

mod test;

/// DSP structure for adding a variable DC offset to a signal
pub struct DcOffsetDSP<const MAX_BLOCKSIZE: usize> {
    pub dc_offset: ParamF32<MAX_BLOCKSIZE>,
}

impl<const MAX_BLOCKSIZE: usize> DcOffsetDSP<MAX_BLOCKSIZE> {
    pub fn new(
        min: f32, // potentially unnecessary to have `min` and `max`
        max: f32,
        initial: f32,
        sample_rate: SampleRate,
    ) -> (DcOffsetDSP<MAX_BLOCKSIZE>, DcOffsetUiHandle) {
        let (dc_offset, dc_offset_handle) = ParamF32::from_value(
            initial,
            min,
            max,
            Gradient::Linear,
            Unit::Generic,
            DEFAULT_SMOOTH_SECS,
            sample_rate,
        );

        (
            DcOffsetDSP { dc_offset },
            DcOffsetUiHandle { dc_offset: dc_offset_handle },
        )
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        self.dc_offset.set_sample_rate(sample_rate);
    }

    pub fn reset_buffers(&mut self) {
        self.dc_offset.reset();
    }

    /// Process a single channel.
    pub fn process_replacing_mono_fb(&mut self, buf: &mut [f32]) {
        let frames = buf.len().min(MAX_BLOCKSIZE);

        let dc_offset = self.dc_offset.smoothed(frames.into());

        if !dc_offset.is_smoothing() {
            let dc_offset = dc_offset[0];

            for i in 0..frames {
                buf[i] += dc_offset;
            }
        } else {
            for i in 0..frames {
                buf[i] += dc_offset[i];
            }
        }
    }

    /// Process a stereo channel.
    pub fn process_replacing_stereo_fb(&mut self, buf_1: &mut [f32], buf_2: &mut [f32]) {
        let frames = buf_1.len().min(buf_2.len()).min(MAX_BLOCKSIZE);

        let dc_offset = self.dc_offset.smoothed(frames.into());

        if !dc_offset.is_smoothing() {
            let dc_offset = dc_offset[0];

            for i in 0..frames {
                buf_1[i] += dc_offset;
                buf_2[i] += dc_offset;
            }
        } else {
            for i in 0..frames {
                buf_1[i] += dc_offset[i];
                buf_2[i] += dc_offset[i];
            }
        }
    }

    /// Process a single stereo signal.
    #[cfg(feature = "portable-simd")]
    pub fn process_replacing_stereo_h<const LANES: usize>(
        &mut self,
        buf_1: &mut [f32],
        buf_2: &mut [f32],
    ) where
        LaneCount<LANES>: SupportedLaneCount,
    {
        unimplemented!()
    }

    /// Process two channels at a time.
    #[cfg(feature = "portable-simd")]
    pub fn process_replacing_stereo_v(&mut self, buf_1: &mut [f32], buf_2: &mut [f32]) {
        unimplemented!()
    }
}

/// UI handle for DC offset DSP module
pub struct DcOffsetUiHandle {
    pub dc_offset: ParamF32UiHandle,
}
