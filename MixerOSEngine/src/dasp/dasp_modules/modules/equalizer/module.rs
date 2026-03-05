use std::error::Error;

use cubecl::prelude::Round;

use super::eq::{EqBand, BandType};

use crate::dasp::dasp_modules::{module::DASPModule, modules::equalizer::biquad::BiquadCoeffs, *};

#[derive(Debug)]
pub struct ParametricEq {
    bands: Vec<BiquadCoeffs>,
    w1: f32,
    w2: f32,
    pub enabled: bool,
}

impl DASPModule for ParametricEq {
    fn process(&mut self, input: f32) -> f32 {
        let mut output: Vec<f32> = Vec::with_capacity(6);

        for band in self.bands.iter() {
            let w = input - (band.a1 * self.w1) - (band.a2 * self.w2);

            let y = (band.b0 * w) + (band.b1 * self.w1) + (band.b2 * self.w2);
            output.push(y);
        }

        output.iter_mut().sum();
    }

    fn reset(&mut self) {
        todo!()
    }

    fn set_sample_rate(&mut self, sample_rate: i32) {
        todo!()
    }

    fn get_sample_rate(&self) -> f32 {
        todo!()
    }
    
    fn process_stereo(&mut self, left: Vec<f32>, right: Vec<f32>) -> (f32, f32) {
        
    }
    
    fn process_buffer(&mut self, buffer: &mut [f32]) {

    }
    
    fn process_stereo_buffer(&mut self, left: &mut [f32], right: &mut [f32]) {

    }
}