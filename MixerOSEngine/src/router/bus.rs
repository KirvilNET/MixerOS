use cpal::{Device, SizedSample, StreamConfig, SupportedInputConfigs};
use num;

use std::sync::Arc;

use crate::dasp::module_manager::ModuleManager;
use crate::router::error::BusError;
use crate::system::util::*;
use crate::system::util::{ChannelType, SampleRate};

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
    input: Option<Arc<cpal::Stream>>,
    output: Option<Arc<cpal::Stream>>,
    processor: ModuleManager,
}

impl Bus {
    pub fn new(
        name: String,
        id: usize,
        ch_type: ChannelType,
        sample_rate: SampleRate,
        device: Device,
        config: StreamConfig,
    ) -> Self {
        let manager = ModuleManager::new(sample_rate);

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
            input: None,
            output: None,
            processor: manager,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
      
      Ok(())
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

    pub fn get_input(&self) -> &Option<Arc<cpal::Stream>> {
        return &self.input;
    }

    pub fn set_input(&mut self, input: cpal::Stream) -> Result<(), BusError> {
        self.input = Some(Arc::new(input));
        Ok(())
    }

    pub fn set_output(&mut self, output: cpal::Stream) -> Result<(), BusError> {
        self.output = Some(Arc::new(output));
        Ok(())
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
}
