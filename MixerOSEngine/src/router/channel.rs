use cpal::traits::DeviceTrait;
use cpal::{Device, SizedSample, StreamConfig, SupportedInputConfigs};
use parking_lot;

use std::sync::Arc;

use crate::system::util::{ ChannelType, SampleRate };
use crate::dasp::module_manager::ModuleManager;
use crate::router::error::ChannelStripError;
use crate::system::util::*;

pub struct ChannelStrip {
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
  processor: ModuleManager
}

impl ChannelStrip {
  pub fn new(name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, device: Device, config: StreamConfig) -> Self {
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
      processor: manager
    }
  }

  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let input_stream: cpal::Stream = self.device.build_input_stream(
    &self.config, 
    move |data: &[f32], _: &cpal::InputCallbackInfo| {
      self.processor.process_chain_buffer_mono(data.as_mut());
    }, 
    |err| eprint!("Stream error: {}", err), 
    None)?;
  }

  pub fn get_name(&mut self) -> String { return self.name.clone(); }
  pub fn get_level(&mut self) -> i8 { return self.level; }
  pub fn get_gain(&mut self) -> i8 { return self.gain; }
  pub fn get_mute(&mut self) -> bool { return self.mute; }

  pub fn get_input(&self) -> &Option<Arc<cpal::Stream>> { return &self.input }

  pub fn set_input(&mut self, input: cpal::Stream) -> Result<(), ChannelStripError> {
    self.input = Some(Arc::new(input));
    Ok(())
  }

  pub fn set_output(&mut self, output: cpal::Stream) -> Result<(), ChannelStripError> {
    self.output = Some(Arc::new(output));
    Ok(())
  } 

  pub fn set_name(&mut self, name: String) -> Result<(), ChannelStripError> { 
    if name.len() == 0 || name.len() > 30 {
      return Err(ChannelStripError::INVALID_NAME)
    }
    self.name = name;
    Ok(())
  }

  pub fn set_level(&mut self, level: i8) -> Result<(), ChannelStripError> { 
    if level < 10 || level > -99 {
      return Err(ChannelStripError::INVALID_LEVEL)
    }
    self.level = level;
    Ok(())
  }

  pub fn set_gain(&mut self, gain: i8) -> Result<(), ChannelStripError> { 
    if gain < 60 || gain > -20 {
      return Err(ChannelStripError::INVALID_GAIN)
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