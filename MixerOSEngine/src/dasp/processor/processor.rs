use crate::dasp::processor::*;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub enum ProcessorType {
  CPU,
  GPU 
}

#[derive(Clone)]
pub enum ProcessorUnit {
  CPU(Arc<cpu::CPU>),
  GPU(Arc<gpu::GPU>)
}

#[derive(Clone)]
pub struct Processor {
  pub processor_type: ProcessorType,
  pub processor: ProcessorUnit
}

impl Processor {
  pub fn create_gpu_processor(channels: u32, buffer_size: u32) -> Result<Arc<gpu::GPU>, gpu::GPUError> {
    let gpu = match gpu::GPU::new(channels, buffer_size) {
        Ok(gpu) => gpu,
        Err(err) => return Err(err),
    };

    return Ok(Arc::new(gpu))
  }

  pub fn create_cpu_processor(channels: u32, buffer_size: u32) -> Arc<cpu::CPU> {
    let cpu = cpu::CPU::new();
    return Arc::new(cpu)
  }
}
