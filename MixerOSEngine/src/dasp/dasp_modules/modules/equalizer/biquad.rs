use std::f32::consts::PI;
use cubecl::prelude::{Cos, Powf, Sin, Sqrt};

#[derive(Debug, Clone, Copy)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
}

impl BiquadCoeffs {

  fn a(peak_shelf: bool, gain: f32) -> f32 {
    if peak_shelf {
      let a = Powf::powf((gain / 20.0).into(), 10.0);
      a
    } else {
      let a = Powf::powf((gain / 20.0).into(), 10.0);
      Sqrt::sqrt(a)
    }
  }

  pub fn lpf(fs: f32, f0: f32, q: f32) -> Self {
    let w0 = 2.0 * PI * f0 / fs;
    //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
    let alpha = Sin::sin(w0) / (2.0 * q);

    Self {
      b0: (1.0 - Cos::cos(w0)) / 2.0,
      b1: 1.0 - Cos::cos(w0),
      b2: (1.0 - Cos::cos(w0)) / 2.0,
      a0: 1.0 + alpha,
      a1: -2.0 * Cos::cos(w0),
      a2: 1.0 - alpha
    }
    
  }

  pub fn hpf(fs: f32, f0: f32, q: f32) -> Self {
    let w0 = 2.0 * PI * f0 / fs;
    //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
    let alpha = Sin::sin(w0) / (2.0 * q);

    Self {
      b0: (1.0 + Cos::cos(w0)) / 2.0,
      b1: 1.0 + Cos::cos(w0),
      b2: (1.0 + Cos::cos(w0)) / 2.0,
      a0: 1.0 + alpha,
      a1: -2.0 * Cos::cos(w0),
      a2: 1.0 - alpha
    }
    
  }

  pub fn bpf(fs: f32, f0: f32, q: f32) -> Self {
    let w0 = 2.0 * PI * f0 / fs;
    let alpha = Sin::sin(w0) / (2.0 * q);

    Self { 
      b0: alpha, 
      b1: 0.0, 
      b2: -alpha, 
      a0: 1.0 + alpha, 
      a1: -2.0 * Cos::cos(w0), 
      a2: 1.0 - alpha 
    }
  }

  

}
