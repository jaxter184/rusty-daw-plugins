#![cfg_attr(feature = "portable-simd", feature(portable_simd))]

#[cfg(feature = "portable-simd")]
use std::simd::{f32x2, LaneCount, Simd, SupportedLaneCount};

use rusty_daw_core::{
	SampleRate,
};

mod test;

/// DSP structure for converting left/right signals to mid/side
///
/// Mid/Side processing does not currently have any parameters
pub struct MidSideSplitterDSP<const MAX_BLOCKSIZE: usize> {}

impl<const MAX_BLOCKSIZE: usize> MidSideSplitterDSP<MAX_BLOCKSIZE> {
    pub fn new(
    ) -> (MidSideSplitterDSP<MAX_BLOCKSIZE>, MidSideSplitterUiHandle) {
        (
            MidSideSplitterDSP {},
            MidSideSplitterUiHandle {},
        )
    }

    pub fn set_sample_rate(&mut self, _sample_rate: SampleRate) {
        // NO-OP
    }

    pub fn reset_buffers(&mut self) {
        // NO-OP
    }

    // does not make sense to have a mono processing function
    //pub fn process_replacing_mono_fb(&mut self, buf: &mut [f32]) {}

    /// Process a stereo channel.
    ///
    /// Convention states that side is (L - R) rather than (R - L).
    ///
    /// In order to retain amplitude when recombined, there must be a scaling
    /// factor of 0.5 applied at either the splitter output or the merger
    /// output (or a 1/sqrt(2) scaling factor at both). I have chosen to put
    /// that scaling factor at the output of the splitter because it makes more
    /// sense (to me) in terms of dynamics processing.
    ///
    /// M = 0.5*(L + R)  
    /// S = 0.5*(L - R)
    ///
    /// # Inputs
    /// * buf_1: left
    /// * buf_2: right
    ///
    /// # Outputs
    /// * buf_1: mid
    /// * buf_2: side
    pub fn process_replacing_stereo_fb(&self, buf_1: &mut [f32], buf_2: &mut [f32]) {

        let frames = buf_1.len().min(buf_2.len()).min(MAX_BLOCKSIZE);

        for i in 0..frames {
            let temp_mid = 0.5*(buf_1[i] + buf_2[i]);
            buf_2[i] = 0.5*(buf_1[i] - buf_2[i]);
            buf_1[i] = temp_mid;
        }
    }

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

    #[cfg(feature = "portable-simd")]
    pub fn process_replacing_stereo_v(&mut self, buf_1: &mut [f32], buf_2: &mut [f32]) {
        unimplemented!()
    }
}

/// Mid/Side processing does not currently have any parameters
pub struct MidSideSplitterUiHandle {}

/// DSP structure for converting mid/side signals to left/right
///
/// Mid/Side processing does not currently have any parameters
pub struct MidSideMergerDSP<const MAX_BLOCKSIZE: usize> {}

impl<const MAX_BLOCKSIZE: usize> MidSideMergerDSP<MAX_BLOCKSIZE> {
    pub fn new(
    ) -> (MidSideMergerDSP<MAX_BLOCKSIZE>, MidSideMergerUiHandle) {
        (
            MidSideMergerDSP {},
            MidSideMergerUiHandle {},
        )
    }

    pub fn set_sample_rate(&mut self, _sample_rate: SampleRate) {
        // NO-OP
    }

    pub fn reset_buffers(&mut self) {
        // NO-OP
    }

    // does not make sense to have a mono processing function
    //pub fn process_replacing_mono_fb(&mut self, buf: &mut [f32]) {}

    /// Process a stereo channel.
    ///
    /// Convention states that side is (L - R) rather than (R - L).
    ///
    /// In order to retain amplitude when recombined, there must be a scaling
    /// factor of 0.5 applied at either the splitter output or the merger
    /// output (or a 1/sqrt(2) scaling factor at both). I have chosen to put
    /// that scaling factor at the output of the splitter because it makes more
    /// sense (to me) in terms of dynamics processing.
    ///
    /// L = L + (0.5*R - 0.5*R) = (0.5*L + 0.5*R) + (0.5*L - 0.5*R) = M + S  
    /// R = (0.5*L - 0.5*L) + R = (0.5*L + 0.5*R) - (0.5*L - 0.5*R) = M - S
    ///
    /// # Inputs
    /// * buf_1: mid
    /// * buf_2: side
    ///
    /// # Outputs
    /// * buf_1: left
    /// * buf_2: right
    pub fn process_replacing_stereo_fb(&self, buf_1: &mut [f32], buf_2: &mut [f32]) {

        let frames = buf_1.len().min(buf_2.len()).min(MAX_BLOCKSIZE);

        for i in 0..frames {
            let temp_mid = buf_1[i] + buf_2[i];
            buf_2[i] = buf_1[i] - buf_2[i];
            buf_1[i] = temp_mid;
        }
    }

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

    #[cfg(feature = "portable-simd")]
    pub fn process_replacing_stereo_v(&mut self, buf_1: &mut [f32], buf_2: &mut [f32]) {
        unimplemented!()
    }
}

/// Mid/Side processing does not currently have any parameters
pub struct MidSideMergerUiHandle {}
