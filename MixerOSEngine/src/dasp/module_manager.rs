
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::dasp::dasp_modules::module::{ DASPModule };
use crate::system::util::*;

/// Type alias for boxed modules
pub type BoxedModule = Box<dyn DASPModule>;

pub enum ModuleManagerError {
	SamplesNotSameLength
}

/// The module manager for each channel
#[derive(Debug, Clone)]
pub struct ModuleManager {
		/// All registered modules
		modules: HashMap<String, Arc<RwLock<BoxedModule>>>,

		/// Processing order
		chain: Vec<String>,

		/// Global sample rate
		sample_rate: SampleRate,
		
		output_buff: Vec<f32>
}

impl ModuleManager {
		pub fn new(sample_rate: SampleRate) -> Self {
				Self {
						modules: HashMap::new(),
						chain: Vec::new(),
						output_buff: vec![],
						sample_rate,
				}
		}

		/// Add a processor to the manager
		pub fn add_processor(&mut self, id: String, processor: BoxedModule) {
				let wrapped = Arc::new(RwLock::new(processor));
				self.modules.insert(id.clone(), wrapped);
				self.chain.push(id);
		}

		/// Remove a processor
		pub fn remove_processor(&mut self, id: &str) -> Option<Arc<RwLock<BoxedModule>>> {
				self.chain.retain(|item| item != id);
				self.modules.remove(id)
		}

		/// Get a processor by ID
		pub fn get_processor(&self, id: &str) -> Option<Arc<RwLock<BoxedModule>>> {
				self.modules.get(id).cloned()
		}

		///Process mono sample through the entire chain
		pub fn process_chain_mono(&self, sample: f32) -> f32 {
			let mut chain_mono = sample;

			for id in &self.chain {
				if let Some(processor) = self.modules.get(id) {
					let mut proc = processor.write();
					let result = proc.process(chain_mono);
					
					chain_mono = result
				}
			}

			chain_mono
		}

		/// Process stereo buffers through the entire chain
		pub fn process_chain_buffer_mono(&self, samples: Vec<f32>) -> Vec<f32> {
			let mut output: Vec<f32> = Vec::new();

			for (sampl, data) in samples.iter().enumerate() {
				let proc_signal = self.process_chain_mono(*data);
				output.insert(sampl, proc_signal);
			}
			
			output
		}

		/// Reorder the processing chain
		pub fn set_chain_order(&mut self, new_order: Vec<String>) {
				// Validate that all IDs exist
				for id in &new_order {
						if !self.modules.contains_key(id) {
								return; // Invalid order, don't change
						}
				}
				self.chain = new_order;
		}

		/// Get the current chain order
		pub fn get_chain_order(&self) -> &[String] {
				&self.chain
		}

		/// Reset all modules
		pub fn reset_all(&self) {
				for processor in self.modules.values() {
						processor.write().reset();
				}
		}

		/// Get list of all processor IDs
		pub fn list_modules(&self) -> Vec<String> {
				self.modules.keys().cloned().collect()
		}
}