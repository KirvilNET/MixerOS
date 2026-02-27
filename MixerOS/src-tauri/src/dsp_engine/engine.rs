use num;

use core::time;
use std::sync::{ Arc, Mutex };
use std::{ thread, time };
use std::error::Error;
use parking_lot::RwLock;


#[derive(Clone, Debug)]
pub struct DSPParameter {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub name: String,
    pub unit: String,
    /// Current smoothed value
    smoothed: f32,
    /// Smoothing coefficient (0.0 = no smoothing, 1.0 = instant)
    smoothing: f32,
}

impl DSPParameter {
    pub fn new(name: &str, default: f32, min: f32, max: f32, unit: &str) -> Self {
        Self {
            value: default,
            min,
            max,
            default,
            name: name.to_string(),
            unit: unit.to_string(),
            smoothed: default,
            smoothing: 0.01, // Default smoothing
        }
    }

    /// Get the smoothed value for processing
    pub fn get_smoothed(&mut self) -> f32 {
        self.smoothed += (self.value - self.smoothed) * self.smoothing;
        self.smoothed
    }

    /// Set value with clamping
    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Normalize value to 0.0-1.0 range
    pub fn normalized(&self) -> f32 {
        (self.value - self.min) / (self.max - self.min)
    }

    /// Set from normalized value (0.0-1.0)
    pub fn set_normalized(&mut self, normalized: f32) {
        let clamped = normalized.clamp(0.0, 1.0);
        self.value = self.min + clamped * (self.max - self.min);
    }
}

#[derive(Clone, Debug, Default)]
pub struct DSPMeteringData {
    pub peak_db: f32,
    pub rms_db: f32,
    pub gain_reduction_db: f32,
    pub is_clipping: bool,
}

pub enum DSPEngine_type {
    Local,
    Remote
}

pub struct DSPEngineConfig {
    pub id: String,
    pub name: String,
    pub mix: Arc<RwLock<DSPParameter>>,
    pub input_gain: Arc<RwLock<DSPParameter>>,
    pub output_gain: Arc<RwLock<DSPParameter>>,
    pub max_channels: Arc<RwLock<i8>>
}

pub struct DSPHealth {
    cummulative: String,
    data: Vec<DSPHealthData<i32>>,
    event_list: Vec<DSPHealthLog>
}

struct DSPHealthLog {
    label: String,
    data: DSPHealthData<i32>,
    timestamp: std::time::SystemTime
}

pub struct DSPHealthData<T> {
    param: String,
    data: T,
    threshold: T,
    critical: T
}

impl DSPHealth {
    fn new() -> Self {
        let data = Vec::new();
        let event_list = Vec::new();

        Self {
            cummulative: "OK".to_string(),
            data,
            event_list
        }
    }

    fn log(mut self, entry: DSPHealthLog) -> Result<(), String>{
        if self.event_list.len() > 500 {
            self.event_list.drain(0..50);
        }

        self.event_list.push(entry);
        Ok(())
    }

    fn check_latency(&self, latency: i32, threshold_val: i32, critical_val: i32) -> Result<DSPHealthData<i32>, String> {
        let data = DSPHealthData { param: "processing_latency".to_string(), data: latency, threshold: threshold_val, critical: critical_val };

        if latency > threshold_val && latency >! critical_val {
            let log = DSPHealthLog { label: "processing_latency".to_string(), data, timestamp: std::time::SystemTime::now()}
            self.log();
        } else if latency > critical_val && latency >! threshold_val{
            self.log();
        }

        Ok(data)
    }
}

pub struct DSPEngine {
    pub engine_type: DSPEngine_type,
    pub enabled: Arc<RwLock<bool>>,
    pub parameters: Arc<RwLock<Vec<DSPParameter>>>,
    pub health: Arc<RwLock<DSPHealth>>,
    pub config: Arc<RwLock<DSPEngineConfig>>,
    pub processors: Arc<RwLock<DSPModule>>,

    handle: std::thread::JoinHandle<()>,
    channels: i8
}

impl DSPEngine {
    fn new(engine_type: DSPEngine_type, parameters: Vec<DSPParameter>, config: DSPEngineConfig) -> Result<Self, Box<dyn Error>> {

        let builder = std::thread::Builder::new().name("DSP Engine".to_string());

        Ok(
            Self {
                engine_type,
                enabled: Arc::new(RwLock::new(true)),
                parameters: Arc::new(RwLock::new(parameters)),
                health: Arc::new(RwLock::new(DSPHealth::new())),
                config: Arc::new(RwLock::new(config)),
                handle: builder.spawn(|| { })?,
                channels: 0
            }
        )
    }

    fn start() {
        
    }
}