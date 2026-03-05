use std::error::Error;

use cubecl::prelude::Round;

use super::eq::{EqBand, BandType};

use crate::dasp::dasp_modules::{module::*, module_param::*, modules::equalizer::biquad::BiquadCoeffs, *};

#[derive(Debug)]
pub struct ParametricEq {
    bands: Vec<BiquadCoeffs>,
    w1: f32,
    w2: f32,
    pub enabled: bool,
}

impl ModuleInfo for ParametricEq {
    fn name(&self) -> &str { "Parametric EQ" }

    fn category(&self) -> module_param::ModuleCategory { ModuleCategory::Equalization }

    fn version(&self) -> &str { "1.0.0" }

    fn unique_id(&self) -> &str { "peq" }
}

impl DASPModule for ParametricEq {
    fn process(&mut self, input: f32) -> f32 {
        let mut output: Vec<f32> = Vec::with_capacity(6);

        for band in self.bands.iter() {
            let w = input - (band.a1 * self.w1) - (band.a2 * self.w2);

            let y = (band.b0 * w) + (band.b1 * self.w1) + (band.b2 * self.w2);
            output.push(y);
        }

        output.iter().sum()
    }

    fn reset(&mut self) {
       self.bands.clear();
       self.w1 = 0.0;
       self.w2 = 0.0;
    }
}

impl Latency for ParametricEq {
    fn get_latency_samples(&self) -> usize { 3 }
}