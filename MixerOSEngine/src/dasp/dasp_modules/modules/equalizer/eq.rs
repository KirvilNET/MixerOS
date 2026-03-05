use anyhow::Ok;

use super::biquad::BiquadCoeffs;
use crate::dasp::dasp_modules::module::*;
use std::{any::{Any, TypeId}, error::Error, sync::{Arc, RwLock}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BandType {
    /// Low pass filter
    LPF,
    /// High pass filter
    HPF,
    /// Band pass filter
    BPF,
    /// Notch filter
    NOTCH,
    /// Peak filter
    PEAK,
    /// Low shelf filter
    LSHELF,
    /// High shelf filter
    HSHELF,
}

pub enum ModuleError {
    MaxBands,
    NotFound,
    ParameterError,
    ReadError
}

#[derive(Debug, Clone)]
pub struct EqBand {
    /// id of the band
    pub id: usize,
    /// Sampling frequency (hz)
    pub fs: f32,
    /// Center frequency (hz)
    pub f0: f32,
    /// Q parameter for filters
    pub q: Option<f32>,
    /// Bandwidth parameter for filters
    pub bw: Option<f32>,
    /// Gain parameter for filters
    pub gain: Option<f32>,
    /// kind of band
    pub kind: BandType,
    /// Is enabled
    pub enabled: bool,
}

pub struct ParametricEq {
    bands: Arc<RwLock<Vec<EqBand>>>,
    enabled: bool,
}

impl ParametricEq {
    pub fn new(bands: Vec<EqBand>) -> Self {
        let mut bands = Arc::new(RwLock::new(Vec::<EqBand>::with_capacity(6)));

        Self {
            bands,
            enabled: true
        }
    }

    pub fn add_band(&mut self, band: EqBand) -> Result<(), ModuleError> {
        let mut band_array = match self.bands.try_read() {
            Result::Ok(bands) => bands,
            Err(_) => return Result::Err(ModuleError::ReadError),
        };

        if band_array.iter().len() == 6 {
            return Err(ModuleError::MaxBands)
        }

        if band_array.iter().len() < 6 {
            self.bands.try_write().expect("Could not write new band").insert(1, band);
        }
        
        self.bands.try_write().expect("Could not sort array").sort_by_key(|k| k.id);

        Result::Ok(())
    }

    pub fn find_by_id(&mut self, id: usize) -> Result<Arc<EqBand>, ModuleError> {
        for band in self.bands.read().expect("Cant unwrap band").iter() {
            if band.id == id {
                return Result::Ok(Arc::new(band.clone()))
            }
        }

        return Err(ModuleError::NotFound)
    }

    pub fn update(&mut self, id: usize, new_band: EqBand) -> Result<(), ModuleError> {
        let curr_band = self.find_by_id(id);

        match curr_band {
            Result::Ok(_) => {
                _ = new_band;
                return Result::Ok(())
            },
            Err(e) => return Err(e),
        }
    }
}