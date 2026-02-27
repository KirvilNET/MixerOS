use cpal;
use dasp;
use num;

use super::biquad::{BiquadCoeffs, BiquadState};
use crate::dasp::dasp_modules::module::*;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BandType {
    Peaking,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
}

#[derive(Debug, Clone)]
pub struct EqBandParams {
    pub freq: f32,
    pub gain_db: f32,
    pub q: f32,
    pub kind: BandType,
    pub enabled: bool,
}

pub struct EqBand {
    pub params: Arc<RwLock<EqBandParams>>,
    coeffs: BiquadCoeffs,
    state: BiquadState,
    sample_rate: f32,
    dirty: bool,
}

impl EqBand {
    pub fn new(params: EqBandParams, sample_rate: f32) -> Self {
        let coeffs = Self::compute_coeffs(&params, sample_rate);

        Self {
            params: Arc::new(RwLock::new(params)),
            coeffs,
            state: BiquadState::default(),
            sample_rate,
            dirty: false,
        }
    }

    fn compute_coeffs(p: &EqBandParams, sr: f32) -> BiquadCoeffs {
        match p.kind {
            BandType::Peaking => BiquadCoeffs::peaking(p.freq, p.gain_db, p.q, sr),
            BandType::LowShelf => BiquadCoeffs::low_shelf(p.freq, p.gain_db, p.q, sr),
            BandType::HighShelf => BiquadCoeffs::high_shelf(p.freq, p.gain_db, p.q, sr),
            BandType::LowPass => BiquadCoeffs::low_pass(p.freq, p.q, sr),
            BandType::HighPass => BiquadCoeffs::high_pass(p.freq, p.q, sr),
        }
    }

    /// Call this at the start of each audio block to pick up param changes
    pub fn update(&mut self) {
        if self.dirty {
            if let Ok(p) = self.params.try_read() {
                self.coeffs = Self::compute_coeffs(&p, self.sample_rate);
            }
            self.dirty = false;
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Process a sample pair
    pub fn process(&mut self, sample: f32) -> f32 {
        let enabled = self.params.try_read().map(|p| p.enabled).unwrap_or(true);

        if !enabled {
            return sample
        }

        self.coeffs.process(sample, &mut self.state)
    }

    pub fn process_buffer(&mut self, buffer: &mut [f32]) {
      let mut output: Vec<f32> = Vec::new();

      for sample in &mut buffer.iter_mut() {

        output.push(self.process(sample));
 
      }
    }
}
