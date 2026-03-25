use crate::dasp::dasp_modules::modules::equalizer::biquad::*;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqBand {
    /// Sampling frequency (hz)
    fs: f32,
    /// Center frequency (hz)
    f0: f32,
    /// either Q, BW, or S parameter for filters
    q_bw_s: f32,
    /// Gain parameter for filters
    gain: f32,
    /// kind of band
    kind: BandType,
    /// Is enabled
    enabled: bool,
}

impl EqBand {
    pub fn new(fs: f32, f0: f32, q_bw_s: f32, gain: f32, kind: BandType, enabled: bool) -> Self {
        Self {
            fs,
            f0,
            q_bw_s,
            gain,
            kind,
            enabled
        }
    }

    pub fn calc_coeffs(&mut self) -> BiquadCoeffs {
        match self.kind {
            BandType::LPF => {
                lpf(self.fs, self.f0, self.q_bw_s)
            },
            BandType::HPF => {
                hpf(self.fs, self.f0, self.q_bw_s)
            },
            BandType::BPF => {
                bpf(self.fs, self.f0, self.q_bw_s)
            },
            BandType::NOTCH => {
                notch(self.fs, self.f0, self.q_bw_s)
            },
            BandType::PEAK => {
                peak(self.fs, self.f0, self.q_bw_s, self.gain)
            },
            BandType::LSHELF => {
                low_shelf(self.fs, self.f0, self.q_bw_s, self.gain)
            },
            BandType::HSHELF => {
                high_shelf(self.fs, self.f0, self.q_bw_s, self.gain)
            },
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled
    }

    pub fn set_gain(&mut self, val: f32) { self.gain = val }
    pub fn set_fs(&mut self, val: f32) { self.fs = val }
    pub fn set_f0(&mut self, val: f32) { self.f0 = val }
    pub fn set_q_bw_s(&mut self, val: f32) { self.q_bw_s = val }

    pub fn get(&self) -> EqBand { self.clone() }
    pub fn state(&self) -> bool { return self.enabled }
}