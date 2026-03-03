use cubecl::prelude::{Cos, Sin, Sinh, Sqrt};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self { 
          b0: 1f32,
          b1: 1f32, 
          b2: 1f32, 
          a1: 1f32, 
          a2: 1f32 
        }
    }
}

impl BiquadCoeffs {
    pub fn lpf(fs: f32, f0: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * f0 / fs;
        //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
        let alpha = Sin::sin(w0) / (2.0 * q);

        let b0 = (1.0 - Cos::cos(w0)) / 2.0;
        let b1 = 1.0 - Cos::cos(w0);
        let b2 = (1.0 - Cos::cos(w0)) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * Cos::cos(w0);
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    pub fn hpf(fs: f32, f0: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * f0 / fs;
        //let a = Sqrt::sqrt(Powf::powf((gain / 20.0).into(), 10.0));
        let alpha = Sin::sin(w0) / (2.0 * q);

        let b0 = (1.0 + Cos::cos(w0)) / 2.0;
        let b1 = 1.0 + Cos::cos(w0);
        let b2 = (1.0 + Cos::cos(w0)) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * Cos::cos(w0);
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    pub fn bpf(fs: f32, f0: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * f0 / fs;
        let alpha = Sin::sin(w0) / (2.0 * q);

        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * Cos::cos(w0);
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    pub fn notch(fs: f32, f0: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * f0 / fs;
        let alpha = Sin::sin(w0) / (2.0 * q);

        let b0 = 1.0;
        let b1 = -2.0 * Cos::cos(w0);
        let b2 = 1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * Cos::cos(w0);
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    pub fn peak(fs: f32, f0: f32, bw: f32, gain: f32) -> Self {
        let w0 = 2.0 * PI * f0 / fs;
        let alpha = Sin::sin(w0) * Sinh::sinh((2.0f32.log2() / 2.0) * bw * (w0 / Sin::sin(w0)));
        let a: f32 = 10.0f32.powf(gain / 40.0);

        let b0 =  1f32 + alpha * a;
        let b1 = -2f32 * Cos::cos(w0);
        let b2 = 1f32 - alpha * a;
        let a0 =  1f32 + (alpha / a);
        let a1 = -2f32 * Cos::cos(w0);
        let a2 = 1f32 - (alpha / a);

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    pub fn low_shelf(fs: f32, f0: f32, s: f32, gain_db: f32) -> Self {
        let a: f32 = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / fs;
        let cos_w0 = w0.cos();
        let alpha = (w0.sin() / 2.0) * Sqrt::sqrt((a + (1.0 / a)) * ((1.0 / s) - 1.0) + 2.0);
        let sqrt_a = a.sqrt();

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha);
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0, // a0 normalizes to 1.0 so we don't store it
            a2: a2 / a0,
        }
    }

    pub fn high_shelf(fs: f32, f0: f32, s: f32, gain_db: f32) -> Self {
        let a: f32 = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / fs;
        let cos_w0 = w0.cos();
        let alpha = (w0.sin() / 2.0) * Sqrt::sqrt((a + (1.0 / a)) * ((1.0 / s) - 1.0) + 2.0);
        let sqrt_a = a.sqrt();

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha);
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_a * alpha;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_a * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0, // a0 normalizes to 1.0 so we don't store it
            a2: a2 / a0,
        }
    }
}

