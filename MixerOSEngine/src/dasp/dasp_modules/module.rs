use std::fmt::Debug;

use crate::dasp::dasp_modules::module_param::{ ParameterDescriptor, Preset };

/// Core trait that all DSP processors must implement
pub trait DASPModule: Send + Sync + Debug {
		/// Process a single mono sample
		fn process(&mut self, input: f32) -> f32 {
				input // Default: pass-through
		}

		/// Process a stereo sample pair (left, right)
		/// 
		/// Override this for stereo-aware processing (e.g., stereo widener, mid-side processing)
		fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
				(self.process(left), self.process(right))
		}

		/// Process a buffer of mono samples
		/// 
		/// Override this for block-based processing (e.g., FFT-based effects, convolution)
		fn process_buffer(&mut self, buffer: &mut [f32]) {
			for sample in buffer.iter_mut() {
					*sample = self.process(*sample);
			}
		}

		/// Process stereo buffers
		/// 
		/// Override this for efficient stereo block processing
		fn process_stereo_buffer(&mut self, left: &mut [f32], right: &mut [f32]) {
				assert_eq!(left.len(), right.len(), "Stereo buffers must be same length");

				for (l, r) in left.iter_mut().zip(right.iter_mut()) {
						let (new_l, new_r) = self.process_stereo(*l, *r);
						*l = new_l;
						*r = new_r;
				}
		}

		/// Reset internal state (called when playback starts/stops, or when parameters change drastically)
		fn reset(&mut self);

		/// Update sample rate (called when audio device changes)
		fn set_sample_rate(&mut self, sample_rate: i32);

		/// Get the current sample rate
		fn get_sample_rate(&self) -> f32;
}

/// Extended trait for processors that have parameters
pub trait ParametricDASPModule: DASPModule {
		/// Get the list of parameter definitions
		fn get_parameters(&self) -> Vec<ParameterDescriptor>;

		/// Get a parameter value by ID
		fn get_parameter(&self, id: &str) -> Option<f32>;

		/// Set a parameter value by ID
		fn set_parameter(&mut self, id: &str, value: f32) -> Result<(), String>;

		/// Get parameter value normalized to 0.0-1.0 range
		fn get_parameter_normalized(&self, id: &str) -> Option<f32> {
				self.get_parameter(id).and_then(|value| {
						self.get_parameters()
								.iter()
								.find(|p| p.id == id)
								.map(|p| (value - p.min) / (p.max - p.min))
				})
		}

		/// Set parameter from normalized 0.0-1.0 value
		fn set_parameter_normalized(&mut self, id: &str, normalized: f32) -> Result<(), String> {
				let param = self.get_parameters()
						.iter()
						.find(|p| p.id == id)
						.ok_or_else(|| format!("Parameter '{}' not found", id))?
						.clone();

				let value = param.min + normalized.clamp(0.0, 1.0) * (param.max - param.min);
				self.set_parameter(id, value)
		}
}

/// Trait for processors that provide metering/analysis data
pub trait Metering {
		/// Get current metering data
		fn get_metering(&self) -> MeteringData;

		/// Get gain reduction (for dynamics processors)
		fn get_gain_reduction(&self) -> Option<f32> {
				None
		}
}

/// Trait for processors that introduce latency
pub trait Latency {
		/// Get latency in samples introduced by this processor
		fn get_latency_samples(&self) -> usize;

		/// Get latency in milliseconds
		fn get_latency_ms(&self, sample_rate: f32) -> f32 {
				self.get_latency_samples() as f32 / sample_rate * 1000.0
		}
}

/// Trait for processors that can be bypassed
pub trait Bypassable {
		/// Check if processor is bypassed
		fn is_bypassed(&self) -> bool;

		/// Set bypass state
		fn set_bypassed(&mut self, bypassed: bool);
}

/// Trait for processors that support presets
pub trait PresetSupport {
		/// Get current state as a preset
		fn save_preset(&self) -> Preset;

		/// Load a preset
		fn load_preset(&mut self, preset: &Preset) -> Result<(), String>;

		/// Get list of factory presets
		fn get_factory_presets(&self) -> Vec<Preset> {
				Vec::new()
		}
}

/// Trait for processors that can be serialized/deserialized
pub trait Serializable {
		/// Serialize processor state to bytes
		fn serialize(&self) -> Result<Vec<u8>, String>;

		/// Deserialize processor state from bytes
		fn deserialize(&mut self, data: &[u8]) -> Result<(), String>;
}

/// Metering data
#[derive(Debug, Clone, Default)]
pub struct MeteringData {
		pub peak_db: f32,
		pub rms_db: f32,
		pub gain_reduction_db: f32,
		pub is_clipping: bool,
}

/// Trait for processors that need to know the buffer size
pub trait BufferSizeAware {
		/// Called when the buffer size changes
		fn set_buffer_size(&mut self, buffer_size: usize);

		/// Get current buffer size
		fn get_buffer_size(&self) -> usize;
}