use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::eq::{ EqBand };
use crate::dasp::dasp_modules::modules::equalizer::biquad::BiquadCoeffs;
use crate::dasp::dasp_modules::{
    module::*,
    module_param::*,
    *,
};

use crate::dasp::processor::gpu::{gpu, pipeline::*};
use crate::dasp::processor::cpu::cpu;

#[derive(Debug)]
pub struct ParametricEq {
    bands: Vec<EqBand>,
    coeffs: Vec<BiquadCoeffs>,
    spv_path: PathBuf,
    pipeline: Option<ComputePipeline>,
    enabled: bool,
}

impl ParametricEq {
    pub fn new() -> Self {
        
        Self {
            bands: Vec::with_capacity(6),
            coeffs: Vec::with_capacity(6),
            spv_path: Path::new("../../../../shaders/parametriceq.spv").canonicalize().unwrap().to_path_buf(),
            pipeline: None,
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

    fn register(&mut self, device: ash::Device, queue_family_index: u32) {

        let mut new_coeffs: Vec<BiquadCoeffs> = Vec::with_capacity(6);
        for (i, band) in self.bands.iter_mut().enumerate() {
            if band.state() == false {
                new_coeffs.insert(i, BiquadCoeffs { b0: 1.0f32, b1: 1.0f32, b2: 1.0f32, a1: 1.0f32, a2: 1.0f32 });
            } else {
                let coeff: BiquadCoeffs = band.calc_coeffs();
                new_coeffs.insert(i, coeff); 
            }
        }

        let pipeline = ComputePipeline::new(
            &device, 
            queue_family_index, 
            self.spv_path.to_str().expect("Could not find Parametric EQ compute shader"), 
            2, std::mem::size_of::<EqBand>() as u32
        );

        self.pipeline = Some(pipeline);
    }

    fn process_gpu(&mut self, input: Vec<f32>, gpu: Arc<gpu::GPU>) -> Vec<f32> {
        let total_size = (1 * input.len()) as u64 * 4;
        let compute_queue = gpu.base.compute_queue;
        let coeffs: [[f32; 5]; 6] = self.coeffs
            .iter()
            .map(|x| [x.a1, x.a2, x.b0, x.b1, x.b2])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        

        self.pipeline.clone().expect(format!("Module {} was used before being registered", self.name()).as_str()).dispatch(
            &gpu.device,
            compute_queue,
            &[
                BufferBinding { binding: 0, buffer: gpu.buffers.channel_input_buff.buffer,  size: total_size },
                BufferBinding { binding: 1, buffer: gpu.buffers.channel_output_buff.buffer, size: total_size },
            ],
            &PushConstants::new(&coeffs),
            (6, 1, 1),
        );

        let output_data: &[f32] = bytemuck::cast_slice(gpu.buffers.channel_output_buff.allocation.mapped_slice().unwrap());
        return output_data.to_vec();
    }

    fn process_cpu(&mut self, input: Vec<f32>, cpu: Arc<cpu::CPU>) -> Vec<f32> {
        vec![]
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
        self.bands.clear();
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


