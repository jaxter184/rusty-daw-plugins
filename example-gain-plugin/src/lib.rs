//! Remember that the goal of this plugin project is **NOT** to create a reusable
//! shared DSP library (I believe that would be more hassle than it is worth). The
//! goal of this plugin project is to simply provide standalone "plugins", each with
//! their own optimized DSP implementation. We are however free to reference and
//! copy-paste portions of DSP across plugins as we see fit (as long as the other
//! plugins are also GPLv3).

#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use baseplug::{Plugin, ProcessContext};
use serde::{Deserialize, Serialize};

use example_gain_dsp::ExampleGainDSP;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct ExampleGainModel {
        // Make sure this model matches the parameters in `ExampleGainDSP`.

        #[model(min = -90.0, max = 6.0)]
        #[parameter(name = "gain", unit = "Generic",  // Do *NOT* use baseplug's
        // "Decibels" unit because we are doing our own unit conversions.
            gradient = "Power(0.15)")]  // Make sure that this gradient matches
        // the gradients in `ExampleGainDSP`.
        #[unsmoothed]  // Make sure *ALL* parameters are "unsmoothed" because we are
        // doing our own parameter smoothing.
        gain: f32,
    }
}

// Insert the default preset here.
impl Default for ExampleGainModel {
    fn default() -> Self {
        Self { gain: 0.0 }
    }
}

struct ExampleGainPlug {
    example_gain_dsp: ExampleGainDSP<MAX_BLOCKSIZE>,
}

impl ExampleGainPlug {
    fn process_replacing(
        &mut self,
        model: &ExampleGainModelProcess,
        buf_left: &mut [f32],
        buf_right: &mut [f32],
    ) {
        // Update our parameters.
        self.example_gain_dsp.gain.set_value(*model.gain);

        self.example_gain_dsp
            .process_replacing_stereo_h::<4>(buf_left, buf_right);

        // Alternate "fallback"/"auto-vectorized" version
        //self.example_gain_dsp.process_replacing_stereo_fb(buf_left, buf_right);

        // Alternate vertical-SIMD version (usually less efficient for gain)
        //self.example_gain_dsp.process_replacing_stereo_v(buf_left, buf_right);
    }
}

baseplug::vst2!(ExampleGainPlug, b"5432");

impl Plugin for ExampleGainPlug {
    const NAME: &'static str = "example gain plug";
    const PRODUCT: &'static str = "example gain plug";
    const VENDOR: &'static str = "RustyDAW";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = ExampleGainModel;

    #[inline]
    fn new(sample_rate: f32, model: &ExampleGainModel) -> Self {
        // If we had a UI we would also hold onto the Ui handle.
        let (example_gain_dsp, _) = ExampleGainDSP::new(
            -90.0, // min dB. Make sure this matches the parameters in the baseplug model.
            6.0,   // max dB. Make sure this matches the parameters in the baseplug model.
            model.gain,
            sample_rate.into(),
        );

        Self { example_gain_dsp }
    }

    // --- Boilerplate stuff: ------------------------------------------------------------

    #[inline]
    fn process(&mut self, model: &ExampleGainModelProcess, ctx: &mut ProcessContext<Self>) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        // Only process if the number of input/output buffers matches what we specified.
        if input.len() != 2 || output.len() != 2 {
            return;
        }

        // Copy input buffers to output buffers.
        output[0].copy_from_slice(input[0]);
        output[1].copy_from_slice(input[1]);

        let (out_left, out_right) = output.split_first_mut().unwrap();
        let out_right = &mut out_right[0];

        // Process in blocks <= `MAX_BLOCKSIZE`
        let mut f = 0;
        let mut frames_left = out_left.len();
        while frames_left > 0 {
            let frames = frames_left.min(MAX_BLOCKSIZE);

            let buf_left_part = &mut out_left[f..f + frames];
            let buf_right_part = &mut out_right[f..f + frames];

            self.process_replacing(model, buf_left_part, buf_right_part);

            frames_left -= frames;
            f += frames;
        }
    }
}

/// This must stay the same as baseplug's internal `MAX_BLOCKSIZE` (128)
const MAX_BLOCKSIZE: usize = 128;
