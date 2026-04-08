use crate::dasp::dasp_modules::modules::equalizer::biquad::BiquadCoeffs;
use crate::dasp::dasp_modules::{ module::*, module_param::*, * };
use crate::dasp::processor::{ BufferAccess, ClBuffer, KernelArg, KernelManager };

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Compressor {
  kernel_manager: Arc<KernelManager>,
  attack: f32,
  release: f32,
  threshold: f32,
  look_ahead_delay: f32,
  ratio: f32,
  makeup_gain: f32,
  mix_level: f32,
  sidechain_filter: bool,
  auto_makeup: bool,
  enabled: bool
}

