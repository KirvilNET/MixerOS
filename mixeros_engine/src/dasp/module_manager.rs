
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::dasp::dasp_modules::module::{ DASPModule };
use crate::dasp::processor::*;

/// Type alias for boxed modules
pub type BoxedModule = Box<dyn DASPModule>;

pub enum ModuleManagerError {
	SamplesNotSameLength,
	ModuleDoesNotExist,
	ModuleAlreadyExist,
	BufferLengthMismatch,
}

/// The module manager for each channel
#[derive(Clone)]
pub struct ModuleManager {
		/// All registered modules
		modules: HashMap<String, Arc<RwLock<BoxedModule>>>,
		/// Processing order
		chain: Vec<String>,
		sample_size: usize,
}

unsafe impl Send for ModuleManager {}
unsafe impl Sync for ModuleManager {}

impl ModuleManager {
		pub fn new(sample_size: usize) -> Self {

			Self {
				modules: HashMap::new(),
				chain: Vec::new(),
				sample_size,
			}
		}

		/// Add a module to the manager
		pub fn add_module(&mut self, id: &str, module: BoxedModule) -> Result<(), ModuleManagerError> {
			if self.modules.contains_key(id) {
				return Err(ModuleManagerError::ModuleAlreadyExist)
			}
			
			self.modules.insert(id.to_string(), Arc::new(RwLock::new(module)));
			Ok(())
		}

		/// Remove a module
		pub fn remove_module(&mut self, id: &str) -> Result<(), ModuleManagerError> {
			if self.modules.contains_key(id) {
				self.modules.remove(id);
				return Ok(())
			} else {
				return Err(ModuleManagerError::ModuleDoesNotExist)
			}
		}

		/// Get a module by ID
		pub fn get_module(&mut self, id: &str) -> Option<&Arc<RwLock<BoxedModule>>> {
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
			let output: Vec<f32> = Vec::with_capacity(self.sample_size);

			if samples.len() as usize != self.sample_size {
				println!("Buffer length mismatch");
			}

			for id in &self.chain {
				if let Some(module) = self.modules.get_mut(id) {
					let mut unlocked_module = module.write();

					if unlocked_module.enabled() {
						let out = unlocked_module.process(samples.clone());
						let _ = output.iter().enumerate().map(|(i, x)| x + out[i]);
					}
				}
			}

			return output
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

		/// Get list of all module IDs
		pub fn list_modules(&self) -> Vec<String> {
			todo!()
		}
}