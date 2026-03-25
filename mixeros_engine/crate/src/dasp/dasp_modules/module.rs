use std::{fmt::Debug};

/// Core trait that all DSP processors must implement
pub trait DASPModule: Send + Sync + Debug {
	fn process(&mut self, samples: Vec<f32>) -> Vec<f32>;

	fn update(&mut self);

	/// Reset internal state (called when playback starts/stops, or when parameters change drastically)
	fn reset(&mut self);

	fn enabled(&self) -> bool;
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