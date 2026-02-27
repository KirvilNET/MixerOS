/// Parameter descriptor
#[derive(Debug, Clone)]
pub struct ParameterDescriptor {
		pub id: String,
		pub name: String,
		pub min: f32,
		pub max: f32,
		pub default: f32,
		pub unit: String,
		pub step: Option<f32>,  // For discrete parameters
		pub value_strings: Option<Vec<String>>,  // For enum-like parameters
		pub is_logarithmic: bool,  // For frequency, gain, etc.
}

impl ParameterDescriptor {
		pub fn new(id: &str, name: &str, default: f32, min: f32, max: f32, unit: &str) -> Self {
				Self {
						id: id.to_string(),
						name: name.to_string(),
						min,
						max,
						default,
						unit: unit.to_string(),
						step: None,
						value_strings: None,
						is_logarithmic: false,
				}
		}

		pub fn with_step(mut self, step: f32) -> Self {
				self.step = Some(step);
				self
		}

		pub fn logarithmic(mut self, is_log: bool) -> Self {
				self.is_logarithmic = is_log;
				self
		}

		pub fn with_value_strings(mut self, strings: Vec<String>) -> Self {
				self.value_strings = Some(strings);
				self
		}
}

/// Preset data structure
#[derive(Debug, Clone)]
pub struct Preset {
		pub name: String,
		pub author: Option<String>,
		pub description: Option<String>,
		pub parameters: Vec<(String, f32)>,  // (parameter_id, value)
}

impl Preset {
		pub fn new(name: &str) -> Self {
				Self {
						name: name.to_string(),
						author: None,
						description: None,
						parameters: Vec::new(),
				}
		}

		pub fn add_parameter(&mut self, id: &str, value: f32) {
				self.parameters.push((id.to_string(), value));
		}
}

/// Processor capability flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleCapabilities {
		pub supports_mono: bool,
		pub supports_stereo: bool,
		pub supports_multi_channel: bool,
		pub is_real_time: bool,
		pub requires_fft: bool,
		pub can_tail: bool,  // Effect has a tail (reverb, delay)
}

impl Default for ModuleCapabilities {
		fn default() -> Self {
				Self {
						supports_mono: true,
						supports_stereo: true,
						supports_multi_channel: false,
						is_real_time: true,
						requires_fft: false,
						can_tail: false,
				}
		}
}

/// Trait for getting processor metadata
pub trait ModuleInfo {
		/// Get processor name
		fn name(&self) -> &str;

		/// Get processor version
		fn version(&self) -> &str {
				"1.0.0"
		}

		/// Get processor category
		fn category(&self) -> ModuleCategory;

		/// Get processor capabilities
		fn capabilities(&self) -> ModuleCapabilities {
				ModuleCapabilities::default()
		}

		/// Get unique identifier
		fn unique_id(&self) -> &str;
}

/// Processor categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleCategory {
		Dynamics,       // Compressor, limiter, gate, expander
		Equalization,   // EQ, filters
		Modulation,     // Chorus, flanger, phaser, tremolo
		Delay,          // Delay, echo
		Reverb,         // Reverb, room simulation
		Distortion,     // Saturation, overdrive, distortion
		Spatial,        // Stereo width, panning, 3D audio
		Utility,        // Gain, phase, DC offset removal
		Analysis,       // Spectrum analyzer, metering
		Generator,      // Oscillator, noise generator
		Other,
}

impl std::fmt::Display for ModuleCategory {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
						ModuleCategory::Dynamics => write!(f, "Dynamics"),
						ModuleCategory::Equalization => write!(f, "Equalization"),
						ModuleCategory::Modulation => write!(f, "Modulation"),
						ModuleCategory::Delay => write!(f, "Delay"),
						ModuleCategory::Reverb => write!(f, "Reverb"),
						ModuleCategory::Distortion => write!(f, "Distortion"),
						ModuleCategory::Spatial => write!(f, "Spatial"),
						ModuleCategory::Utility => write!(f, "Utility"),
						ModuleCategory::Analysis => write!(f, "Analysis"),
						ModuleCategory::Generator => write!(f, "Generator"),
						ModuleCategory::Other => write!(f, "Other"),
				}
		}
}