use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{BuildStreamError, Device, StreamConfig };
use parking_lot::{ Mutex };

use std::sync::Arc;
use std::time::Duration;

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
  buffer: Arc<Mutex<Vec<f32>>>,
  input: Option<Arc<cpal::Stream>>,
  processor: Arc<Mutex<ModuleManager>>
}

impl ChannelStrip {
  pub fn new(name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, device: Device, config: StreamConfig) -> Self {
    let manager = ModuleManager::new(sample_rate);
    let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(vec![]));

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
      input: None, 
      processor: Arc::new(Mutex::new(manager))
    }
  }

  pub fn create_input(&mut self) -> Result<(), BuildStreamError> {
    let processor = Arc::clone(&self.processor);
    let buffer = Arc::clone(&self.buffer);

    let input = self.device.build_input_stream(
      &self.config, 
      move | data: &[f32], _: &cpal::InputCallbackInfo | {
        let output = processor.lock().process_chain_buffer_mono(Vec::from(data));
        
        let mut buf = buffer.lock();
        *buf = output;
      },  
      move |err| {
        eprintln!("an error occurred on stream: {}", err);
      }, 
      Some(Duration::new(1, 0))
    ).expect("Could not create input stream");

    
    self.input = Some(Arc::new(input));
    Ok(())
  }

  pub async fn run(&self) {
    let input = Arc::clone(&self.input.as_ref().unwrap());
    input.play();
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

  pub fn set_name(&mut self, name: String) -> Result<(), ChannelStripError> { 
    if name.len() == 0 || name.len() > 30 {
      return Err(ChannelStripError::InvalidName)
    }
    self.name = name;
    Ok(())
  }

  pub fn set_level(&mut self, level: i8) -> Result<(), ChannelStripError> { 
    if level < 10 || level > -99 {
      return Err(ChannelStripError::InvalidLevel)
    }
    self.level = level;
    Ok(())
  }

  pub fn set_gain(&mut self, gain: i8) -> Result<(), ChannelStripError> { 
    if gain < 60 || gain > -20 {
      return Err(ChannelStripError::InvalidGain)
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