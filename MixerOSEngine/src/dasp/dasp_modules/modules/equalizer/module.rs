use super::eq::{EqBand, EqBandParams, BandType};

pub struct ParametricEq {
    bands: Vec<EqBand>,
    pub enabled: bool,
}

impl ParametricEq {
  pub fn new(sample_rate: f32) -> Self {
    let default_bands = vec![
        EqBandParams { freq:   120.0, gain_db: 0.0, q: 1.0,   kind: BandType::LowShelf,  enabled: true },
        EqBandParams { freq:   300.0, gain_db: 0.0, q: 1.0,   kind: BandType::Peaking,   enabled: true },
        EqBandParams { freq:   800.0, gain_db: 0.0, q: 1.0,   kind: BandType::Peaking,   enabled: true },
        EqBandParams { freq:  2000.0, gain_db: 0.0, q: 1.0,   kind: BandType::Peaking,   enabled: true },
        EqBandParams { freq:  5000.0, gain_db: 0.0, q: 1.0,   kind: BandType::Peaking,   enabled: true },
        EqBandParams { freq: 10000.0, gain_db: 0.0, q: 1.0,   kind: BandType::HighShelf, enabled: true },
    ];

    Self {
        bands: default_bands.into_iter()
            .map(|p| EqBand::new(p, sample_rate))
            .collect(),
        enabled: true,
    }
  }

  pub fn set_band(&mut self, index: usize, freq: f32, gain_db: f32, q: f32, kind: BandType) {
      if let Some(band) = self.bands.get_mut(index) {
          if let Ok(mut p) = band.params.write() {
              p.freq    = freq;
              p.gain_db = gain_db;
              p.q       = q;
              p.kind    = kind;
          }
          band.mark_dirty();
      }
  }

  pub fn set_band_enabled(&mut self, index: usize, enabled: bool) {
      if let Some(band) = self.bands.get_mut(index) {
          if let Ok(mut p) = band.params.write() {
              p.enabled = enabled;
          }
      }
  }

  pub fn update(&mut self) {
      for band in &mut self.bands {
          band.update();
      }
  }

  pub fn process_buffer(&mut self, buffer: &mut [f32]) {
      if !self.enabled { return; }

      self.update();

      for sample in buffer.iter_mut() {
          let mut smpl = *sample;

          for band in &mut self.bands {
              smpl = band.process(smpl);
          }

          *sample  = smpl;
    
      }
  }
}