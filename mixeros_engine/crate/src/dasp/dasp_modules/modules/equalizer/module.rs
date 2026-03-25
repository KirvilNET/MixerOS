use super::eq::{ EqBand };
use crate::dasp::dasp_modules::modules::equalizer::biquad::BiquadCoeffs;
use crate::dasp::dasp_modules::{ module::*, module_param::*, * };
use crate::dasp::processor::{ BufferAccess, ClBuffer, KernelArg, KernelManager };

use std::sync::Arc;

#[derive(Debug)]
pub struct ParametricEq {
    kernel_manager: Arc<KernelManager>,
    bands: Vec<EqBand>,
    coeffs: Vec<BiquadCoeffs>,
    enabled: bool,
}

impl ParametricEq {
    pub fn new(kernel_manager: Arc<KernelManager>) -> Self {
        
        Self {
            kernel_manager,
            bands: Vec::with_capacity(6),
            coeffs: Vec::with_capacity(6),
            enabled: true,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        self.reset();
    }

    pub fn load(&mut self, data: Vec<EqBand>) {
        self.bands = data;
    }

    pub fn save(&mut self) -> Vec<EqBand> {
        return self.bands.clone();
    }
}

impl ModuleInfo for ParametricEq {
    fn name(&self) -> &str {
        "Parametric EQ"
    }

    fn category(&self) -> module_param::ModuleCategory {
        ModuleCategory::Equalization
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn unique_id(&self) -> &str {
        "peq"
    }
}

impl DASPModule for ParametricEq {

    fn process(&mut self, samples: Vec<f32>) -> Vec<f32> {
        let input_buffer = ClBuffer::from_slice(&self.kernel_manager.context, samples.as_slice(), BufferAccess::ReadOnly)
            .expect(format!("Error creating opencl buffer for: {}", self.name()).as_str());
        let output_buffer = ClBuffer::new(&self.kernel_manager.context, samples.len(), BufferAccess::ReadWrite) 
            .expect(format!("Error creating opencl buffer for: {}", self.name()).as_str());

        let bands_flat: Vec<f32> = self.coeffs.iter().flat_map(|band| vec![band.b0, band.b1, band.b2, band.a1, band.a2]).collect();
        let bands_buffer = ClBuffer::from_slice(&self.kernel_manager.context, &bands_flat, BufferAccess::ReadOnly)
            .expect(format!("Error creating opencl buffer for: {}", self.name()).as_str());

        self.kernel_manager.dispatch(
            "eq", 
            &[
                KernelArg::Buffer(&input_buffer),
                KernelArg::Buffer(&output_buffer),
                KernelArg::Buffer(&bands_buffer),
                KernelArg::U32(samples.len() as u32)
            ], 
            1, 
            Some(64)
        ).expect(format!("Error dispatching kernel for: {}", self.name()).as_str());

        self.kernel_manager.read_buffer(&output_buffer).expect(format!("Error reading output buffer for: {}", self.name()).as_str())
    }

    fn update(&mut self) {
        let mut new_coeffs: Vec<BiquadCoeffs> = Vec::with_capacity(6);

        for (i, band) in self.bands.iter_mut().enumerate() {
            if band.state() == false {
                new_coeffs.insert(i, BiquadCoeffs { b0: 1.0f32, b1: 1.0f32, b2: 1.0f32, a1: 1.0f32, a2: 1.0f32 });
            } else {
                let coeff: BiquadCoeffs = band.calc_coeffs();
                new_coeffs.insert(i, coeff); 
            }
        }

        drop(new_coeffs);
    }

    fn reset(&mut self) {
        self.coeffs.clear();
    }

    fn enabled(&self) -> bool {
        return self.enabled
    }
}

impl Latency for ParametricEq {
    fn get_latency_samples(&self) -> usize {
        3
    }
}

/*
#[cube]
fn process_band(a1: f32, a2: f32, b0: f32, b1: f32, b2: f32, input: Array<f32> ) -> f32 {
    let mut w1: f32 = 0.0;
    let mut w2: f32 = 0.0;

    let w: f32 = input[0] - (a1 * w1) - (a2 * w2);
    w1 = input[1] - (a1 * w1) - (a2 * w2);
    w2 = input[2] - (a1 * w1) - (a2 * w2);
    let y: f32 = (b0 * w) + (b1 * w1) + (b2 * w2);

    y as f32
}
*/
/* 
#[cube]
fn process(bands: Array<f32>, input: Array<f32>) -> f32 {
    if bands.len() / 6 != 5 {
        terminate!()
    }

    let band: usize = ABSOLUTE_POS_X as usize;
    let base: usize = band * 5;
    let mut output: f32 = 0.0;

    let b0 = bands[base];
    let b1 = bands[base + 1];
    let b2 = bands[base + 2];
    let a1 = bands[base + 3];
    let a2 = bands[base + 4];

    output += process_band(a1, a2, b0, b1, b2, input);

    output
}
*/
/* 
fn processor_buffer(params: Vec<f32>, input: Vec<f32>, output: &mut Vec<f32>) {
    if params.len() / 6 != 5 {
        terminate!()
    }

    let i: u32 = ABSOLUTE_POS as u32;
    let input_base: usize = ABSOLUTE_POS * 3usize;
    let mut chunk: Vec<f32> = Array::new(3usize);

    chunk[0u32] = input[input_base];
    chunk[1u32] = input[input_base + 1u32];
    chunk[2u32] = input[input_base + 2u32];
    output[i] = process(params, chunk);
}
*/

/* 
#[cube]
fn process(module: &mut ParametricEq, input: Array<f32> ) -> f32 {
    let mut out: f32 = 0.0;

    for i in 0..module.bands.len() {
        let mut w1: f32 = 0.0;
        let mut w2: f32 = 0.0;
        let mut band = module.bands[i];

        for i in 0..input.len() {
            let sample = input[i];

            if band.state() == false {
                break;
            } else {
                let coeffs = band.calc_coeffs();

                let w: f32 = sample - (a1 * w1) - (a2 * w2);
                let y: f32 = (b0 * w) + (b1 * w1) + (b2 * w2);

                if i == 0 {
                    w1 = w
                } else if i == 1 {
                    w2 = w
                }

                out += y;
            }
        }

    }

    out
    
}
*/


