
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::dasp::dasp_modules::module::{ DASPModule };
use crate::dasp::processor::processor::*;

use crate::dasp::processor::*;

/// Type alias for boxed modules
pub type BoxedProcessor = Box<dyn DASPModule>;

pub enum ModuleManagerError {
	SamplesNotSameLength,
	ModuleDoesNotExist,
	ModuleAlreadyExist
}

/// The module manager for each channel
#[derive(Clone)]
pub struct ModuleManager {
		/// All registered modules
		modules: HashMap<String, Arc<RwLock<BoxedProcessor>>>,
		/// Processing order
		chain: Vec<String>,
		processor: Arc<Processor>,
}

unsafe impl Send for ModuleManager {}
unsafe impl Sync for ModuleManager {}

impl ModuleManager {
		pub fn new(processor: Processor) -> Self {

			Self {
				modules: HashMap::new(),
				chain: Vec::new(),
				processor: Arc::new(processor),
			}
		}

		/// Add a processor to the manager
		pub fn add_processor(&mut self, id: &str, processor: BoxedProcessor) -> Result<(), ModuleManagerError> {
			if self.modules.contains_key(id) {
				return Err(ModuleManagerError::ModuleAlreadyExist)
			}
			
			self.modules.insert(id.to_string(), Arc::new(RwLock::new(processor)));
			Ok(())
		}

		/// Remove a processor
		pub fn remove_processor(&mut self, id: &str) -> Result<(), ModuleManagerError> {
			if self.modules.contains_key(id) {
				self.modules.remove(id);
				return Ok(())
			} else {
				return Err(ModuleManagerError::ModuleDoesNotExist)
			}
		}

		/// Get a processor by ID
		pub fn get_processor(&mut self, id: &str) -> Option<&Arc<RwLock<BoxedProcessor>>> {
			if self.modules.contains_key(id) {
				match self.modules.get(id) {
						Some(module) => {
							return Some(module)
						},
						None => {
							return None
						},
				}
			} else {
				return None
			}
		}

		/// Process stereo buffers through the entire chain
		pub fn process_chain_buffer_mono(&mut self, samples: Vec<f32>) -> Vec<f32> {
			let mut output: Vec<f32> = Vec::new();

			match &self.processor.processor {
					ProcessorUnit::GPU(gpu) => {
						let gpu_ptr: std::sync::Arc<gpu::GPU> = Arc::clone(gpu);
						

						for id in &self.chain {
							if let Some(processor) = self.modules.get_mut(id) {
								let out = processor.write().process_gpu(samples.to_vec(), gpu_ptr.clone());
								let _ = &output.iter_mut().enumerate().map(|(index, i)| *i + out[index]);
							}
						}
						
					},
					ProcessorUnit::CPU(cpu) => {
						let cpu_ptr: std::sync::Arc<cpu::CPU> = Arc::clone(cpu);

						for id in &self.chain {
							if let Some(processor) = self.modules.get_mut(id) {
								let out: Vec<f32> = processor.write().process_cpu(samples.to_vec(), cpu_ptr.clone());
								let _ = output.iter_mut().enumerate().map(|(index, i)| *i + out[index]);
							}
						}

					},
			}
			
			return output.iter().map(|smpl| smpl / self.chain.len() as f32).collect::<Vec<f32>>()
		}

		/// Reorder the processing chain
		pub fn set_chain_order(&mut self, new_order: Vec<String>) {
			todo!()
		}

		/// Get the current chain order
		pub fn get_chain_order(&self) -> &[String] {
				&self.chain
		}

		/// Reset all modules
		pub fn reset_all(&self) {
			todo!()
		}

		/// Get list of all processor IDs
		pub fn list_modules(&self) -> Vec<String> {
			todo!()
		}
}