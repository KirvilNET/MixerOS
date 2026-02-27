
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::dasp::dasp_modules::module::{ DASPModule };
use crate::system::util::*;

/// Type alias for boxed modules
pub type BoxedModule = Box<dyn DASPModule>;

/// The module manager for each channel
#[derive(Debug, Clone)]
pub struct ModuleManager {
		/// All registered modules
		modules: HashMap<String, Arc<RwLock<BoxedModule>>>,

		/// Processing order
		chain: Vec<String>,

		/// Global sample rate
		sample_rate: SampleRate,
}

impl ModuleManager {
		pub fn new(sample_rate: SampleRate) -> Self {
				Self {
						modules: HashMap::new(),
						chain: Vec::new(),
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

		/// Process a stereo sample through the entire chain
		pub fn process_chain_stereo(&self, mut left: f32, mut right: f32) -> (f32, f32) {
				
			for id in &self.chain {
				if let Some(processor) = self.modules.get(id) {
						let mut proc = processor.write();
						let result = proc.process_stereo(left, right);
						left = result.0;
						right = result.1;
				}
			}
			(left, right)

		}

		///Process mono sample through the entire chain
		pub fn process_chain_mono(&self, mut sample: f32) -> f32 {

			for id in &self.chain {
				if let Some(processor) = self.modules.get(id) {
					let mut proc = processor.write();
					let result = proc.process(sample);
					sample = result
				}
			}
			sample
		}

		/// Process stereo buffers through the entire chain
		pub fn process_chain_buffer_stereo(&self, left: &mut [f32], right: &mut [f32]) {
				for id in &self.chain {
						if let Some(processor) = self.modules.get(id) {
								let mut proc = processor.write();
								proc.process_stereo_buffer(left, right);
						}
				}
		}

		/// Process stereo buffers through the entire chain
		pub fn process_chain_buffer_mono(&self, samples: &mut [f32]) {
				for id in &self.chain {
						if let Some(processor) = self.modules.get(id) {
								let mut proc = processor.write();
								proc.process_buffer(samples);
						}
				}
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

		/// Update sample rate for all modules
		pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
				self.sample_rate = sample_rate;
				for processor in self.modules.values() {
						processor.write().set_sample_rate(sample_rate as i32);
				}
		}

		/// Get list of all processor IDs
		pub fn list_modules(&self) -> Vec<String> {
				self.modules.keys().cloned().collect()
		}
}

/// Factory for creating modules
pub struct ProcessorFactory;

impl ProcessorFactory {
	// TODO
}

/// Preset manager for saving/loading entire chains
pub struct PresetManager {
		presets: HashMap<String, ChainPreset>,
}

#[derive(Clone, Debug)]
pub struct ChainPreset {
		pub name: String,
		pub chain_order: Vec<String>,
		pub processor_states: HashMap<String, Vec<(String, f32)>>, // processor_id -> parameters
}

impl PresetManager {
		pub fn new() -> Self {
				Self {
						presets: HashMap::new(),
				}
		}

		/// Save current state of plugin manager as a preset
		pub fn save_preset(
				&mut self,
				name: String,
				plugin_manager: &ModuleManager,
		) -> Result<(), String> {
				let mut processor_states = HashMap::new();

				for (id, processor) in &plugin_manager.modules {
						let proc = processor.read();

						// TODO
				}

				let preset = ChainPreset {
						name: name.clone(),
						chain_order: plugin_manager.chain.clone(),
						processor_states,
				};

				self.presets.insert(name, preset);
				Ok(())
		}

		/// Load a preset into the plugin manager
		pub fn load_preset(
				&self,
				name: &str,
				plugin_manager: &mut ModuleManager,
		) -> Result<(), String> {
				let preset = self.presets
						.get(name)
						.ok_or_else(|| format!("Preset '{}' not found", name))?;

				// Set chain order
				plugin_manager.set_chain_order(preset.chain_order.clone());

				// Apply processor parameters
				for (id, params) in &preset.processor_states {
						if let Some(processor) = plugin_manager.get_processor(id) {
								let mut proc = processor.write();
								// TODO
						}
				}

				Ok(())
		}

		/// List available presets
		pub fn list_presets(&self) -> Vec<String> {
				self.presets.keys().cloned().collect()
		}
}