use cpal::traits::{ DeviceTrait, StreamTrait };
use cpal::{ BuildStreamError, Device, StreamConfig };

use std::collections::HashMap;
use std::ops::Div;
use std::sync::Arc;
use std::time::Duration;

use crate::dasp::module_manager::ModuleManager;
use crate::router::error::BusError;
use crate::system::util::*;
use crate::system::util::{ChannelType, SampleRate};
use crate::dasp::processor::processor::*;

#[derive(Clone)]
pub struct Bus {
    name: String,
    status: DASPStatus,
    config: StreamConfig,
    id: usize,
    ch_type: ChannelType,
    level: i8,
    gain: i8,
    mute: bool,
    device: Device,
    buffer: HashMap<usize, Vec<f32>>,
    output: Option<Arc<cpal::Stream>>,
    processor: ModuleManager,
}

impl Bus {
    pub fn new( name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, device: Device, buffer_size: u32, proc: Processor) -> Self {
        let manager = ModuleManager::new(proc);
        let buffer = HashMap::new();

        let supported_config = device.supported_input_configs().expect("Error while quering configs").next().expect("No avalible configs");
        let mut rate: u32 = 0;

        if get_sample_rate(sample_rate) < supported_config.max_sample_rate() {
          println!("Requested Sample Rate is higher than the max sample rate of the device ({})", supported_config.max_sample_rate());
          rate = supported_config.max_sample_rate();
        } else if get_sample_rate(sample_rate) < supported_config.min_sample_rate(){
          println!("Requested Sample rate is lower than the minimum sample rate of the device ({})", supported_config.min_sample_rate());
          rate = supported_config.min_sample_rate();
        }

        let config = StreamConfig { 
          channels: 1, 
          sample_rate: rate, 
          buffer_size: cpal::BufferSize::Fixed(buffer_size),
        };

        Self {
            name,
            status: DASPStatus::STARTING,
            config,
            id,
            ch_type,
            level: 0,
            gain: 0,
            mute: true,
            device,
            buffer,
            output: None,
            processor: manager,
        }
    }

    pub fn create_output(&mut self) -> Result<(), BuildStreamError> {
        let buffer = self.output();

        let output = self.device.build_output_stream(
          &self.config, 
          move | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
            let len = data.len().min(buffer.len());
            data[..len].copy_from_slice(&buffer[..len]);
            for sample in &mut data[len..] {
                *sample = 0.0;
            }
          }, 
          move |err| {
            eprintln!("an error occurred on stream: {}", err);
          }, 
          Some(Duration::new(1, 0))
        ).expect("Could not create output stream");

        self.output = Some(Arc::new(output));
        Ok(())
    }

    fn output(&mut self) -> Vec<f32> {
        let mut output: Vec<f32> = Vec::new();

        for id in self.buffer.keys() {
            match self.buffer.get(id) {
                Some(data) => {
                    output = output.iter().zip(data.iter())
                        .map(|(&o, &d)| (o + d).div(*id as f32))
                        .collect();
                },
                None => {

                }
            }
        }

        output
        
    }


    pub async fn run(&self) -> Result<(), BusError> {
        if self.output.is_none() {
            return Err(BusError::NoOutput)
        }

        let output = Arc::clone(&self.output.as_ref().unwrap());
        output.play().unwrap();
        return Ok(())
    }

    pub fn add_input(&mut self, id: usize, source: Vec<f32>) {
        self.buffer.insert(id, source);
    }

    pub fn get_name(&mut self) -> String {
        return self.name.clone();
    }
    pub fn get_level(&mut self) -> i8 {
        return self.level;
    }
    pub fn get_gain(&mut self) -> i8 {
        return self.gain;
    }
    pub fn get_mute(&mut self) -> bool {
        return self.mute;
    }

    pub fn set_name(&mut self, name: String) -> Result<(), BusError> {
        if name.len() == 0 || name.len() > 30 {
            return Err(BusError::InvalidName);
        }
        self.name = name;
        Ok(())
    }

    pub fn set_level(&mut self, level: i8) -> Result<(), BusError> {
        if level < 10 || level > -99 {
            return Err(BusError::InvalidLevel);
        }
        self.level = level;
        Ok(())
    }

    pub fn set_gain(&mut self, gain: i8) -> Result<(), BusError> {
        if gain < 60 || gain > -20 {
            return Err(BusError::InvalidGain);
        }
        self.gain = gain;
        Ok(())
    }

    pub fn set_mute(&mut self, mute: bool) {
        if self.mute != mute {
            self.mute = mute;
        }
    }

    pub fn set_status(&mut self, status: DASPStatus) {
        self.status = status;
    }
}
