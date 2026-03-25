use std::f32::consts::PI;

use num::*;
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Default for BiquadCoeffs {
    fn default() -> BiquadCoeffs {
        BiquadCoeffs { 
          b0: 1f32,
          b1: 1f32, 
          b2: 1f32, 
          a1: 1f32, 
          a2: 1f32 
        }
    }
}

pub fn lpf(fs: f32, f0: f32, q: f32) -> BiquadCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
    let alpha = (w0).sin() / (2.0 * q);

    let b0 = (1.0 - (w0).cos()) / 2.0;
    let b1 = 1.0 - (w0).cos();
    let b2 = (1.0 - (w0).cos()) / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * (w0).cos();
    let a2 = 1.0 - alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

pub fn hpf(fs: f32, f0: f32, q: f32) -> BiquadCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
    let alpha = (w0).sin() / (2.0 * q);

    let b0 = (1.0 + (w0).cos()) / 2.0;
    let b1 = 1.0 + (w0).cos();
    let b2 = (1.0 + (w0).cos()) / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * (w0).cos();
    let a2 = 1.0 - alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

pub fn bpf(fs: f32, f0: f32, q: f32) -> BiquadCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    let alpha = (w0).sin() / (2.0 * q);

    let b0 = alpha;
    let b1 = 0.0;
    let b2 = -alpha;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * (w0).cos();
    let a2 = 1.0 - alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

pub fn notch(fs: f32, f0: f32, q: f32) -> BiquadCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    let alpha = (w0).sin() / (2.0 * q);

    let b0 = 1.0;
    let b1 = -2.0 * (w0).cos();
    let b2 = 1.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * (w0).cos();
    let a2 = 1.0 - alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

pub fn peak(fs: f32, f0: f32, bw: f32, gain: f32) -> BiquadCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    let alpha = (w0).sin() * ((2.0f32.log2() / 2.0) * bw * (w0 / (w0).sin())).sinh();
    let a: f32 = (10.0.ln() * (gain / 40.0)).exp();
    //let a: f32 = pow(10.0f32, (gain / 40.0) as usize);

    let b0 =  1f32 + alpha * a;
    let b1 = -2f32 * (w0).cos();
    let b2 = 1f32 - alpha * a;
    let a0 =  1f32 + (alpha / a);
    let a1 = -2f32 * (w0).cos();
    let a2 = 1f32 - (alpha / a);

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

pub fn low_shelf(fs: f32, f0: f32, s: f32, gain: f32) -> BiquadCoeffs {
    let a: f32 = (10.0 * (gain / 40.0)).exp();
    let w0 = 2.0 * std::f32::consts::PI * f0 / fs;
    let cos_w0 = w0.cos();
    let alpha = (w0.sin() / 2.0) * ((a + (1.0 / a)) * ((1.0 / s) - 1.0) + 2.0).sqrt();
    let sqrt_a = a.sqrt();

    let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha);
    let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
    let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha);
    let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha;
    let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
    let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0, // a0 normalizes to 1.0 so we don't store it
        a2: a2 / a0,
    }
}

pub fn high_shelf(fs: f32, f0: f32, s: f32, gain: f32) -> BiquadCoeffs {
    let a: f32 = (10.0 * (gain / 40.0)).exp();
    let w0 = 2.0 * std::f32::consts::PI * f0 / fs;
    let cos_w0 = w0.cos();
    let alpha = (w0.sin() / 2.0) * ((a + (1.0 / a)) * ((1.0 / s) - 1.0) + 2.0).sqrt();
    let sqrt_a = a.sqrt();

    let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha);
    let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
    let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha);
    let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha;
    let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
    let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha;

    BiquadCoeffs {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0, // a0 normalizes to 1.0 so we don't store it
        a2: a2 / a0,
    }
}

