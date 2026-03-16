use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{BufferSize, BuildStreamError, Device, StreamConfig };
use tokio::sync::{ RwLock };

use std::sync::Arc;
use std::time::Duration;

use crate::system::util::{ ChannelType, SampleRate };
use crate::dasp::module_manager::ModuleManager;
use crate::router::error::ChannelStripError;
use crate::system::util::*;
use crate::dasp::processor::processor::*;

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
  buffer: Arc<std::sync::RwLock<Vec<f32>>>,
  input: Option<Arc<RwLock<cpal::Stream>>>,
  processor: Arc<std::sync::RwLock<ModuleManager>>
}

impl ChannelStrip {
  pub fn new(name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, device: Device, buffer_size: u32, proc: Processor) -> Self {
    let manager = ModuleManager::new(proc);
    let buffer: Arc<std::sync::RwLock<Vec<f32>>> = Arc::new(std::sync::RwLock::new(vec![]));

    let supported_config = device.supported_input_configs().expect("Error while quering configs").next().expect("No avalible configs");
    let rate: u32;

    if get_sample_rate(sample_rate) < supported_config.max_sample_rate() {
      println!("Requested Sample Rate is higher than the max sample rate of the device ({})", supported_config.max_sample_rate());
      rate = supported_config.max_sample_rate();
    } else if get_sample_rate(sample_rate) < supported_config.min_sample_rate(){
      println!("Requested Sample rate is lower than the minimum sample rate of the device ({})", supported_config.min_sample_rate());
      rate = supported_config.min_sample_rate();
    } else {
      rate = get_sample_rate(sample_rate);
    }

    let config = StreamConfig { 
      channels: supported_config.channels(), 
      sample_rate: rate, 
      buffer_size: BufferSize::Fixed(buffer_size),
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
      input: None, 
      processor: Arc::new(std::sync::RwLock::new(manager))
    }
  }

  pub async fn create_input(&mut self) -> Result<(), BuildStreamError> {
    let processor = Arc::clone(&self.processor);
    let mut buffer: Vec<f32> = Arc::clone(&self.buffer).write().unwrap().to_vec();

    let input = self.device.build_input_stream(
      &self.config, 
      move | data: &[f32], _: &cpal::InputCallbackInfo | {
        let output = processor.write().unwrap().process_chain_buffer_mono(Vec::from(data));
      
        buffer.clear();
        buffer = output;
      },  
      move |err| {
        eprintln!("an error occurred on stream: {}", err);
      }, 
      Some(Duration::new(1, 0))
    ).expect("Could not create input stream");

    
    self.input = Some(Arc::new(RwLock::new(input)));
    Ok(())
  }

  pub async fn run(&mut self) -> Result<(), ChannelStripError> {
    match self.create_input().await {
        Ok(_) => {
          let input: Arc<RwLock<cpal::Stream>> = Arc::clone(&self.input.as_ref().unwrap());

          match input.write().await.play() {
              Ok(_) => {
                println!("initilized the channel {}", self.get_name());
                return Ok(())
              },
              Err(_) => {
                println!("failed to initialize channel {}", self.get_name());
                return Err(ChannelStripError::InputError)
              }
          };
        },
        Err(err) => {
          match err {
            BuildStreamError::DeviceNotAvailable => {
              println!("DeviceNotAvalible \n {:#?}", self.device.id());
              return Err(ChannelStripError::StreamError)
            },
            BuildStreamError::StreamConfigNotSupported => {
              println!("StreamConfigNotSupported \n {:#?}", self.config);
              return Err(ChannelStripError::StreamError)
            },
            BuildStreamError::InvalidArgument => {
              println!("InvalidArguement");
              return Err(ChannelStripError::StreamError)
            },
            BuildStreamError::StreamIdOverflow => {
              println!("StreamIdOverflow");
              return Err(ChannelStripError::StreamError)
            },
            BuildStreamError::BackendSpecific { err } => {
              println!("BackendSpecific Error: {}", err);
              return Err(ChannelStripError::StreamError)
            },
          }
        },
    }

  }

  pub fn get_name(&mut self) -> String { return self.name.clone(); }
  pub fn get_level(&mut self) -> i8 { return self.level; }
  pub fn get_gain(&mut self) -> i8 { return self.gain; }
  pub fn get_mute(&mut self) -> bool { return self.mute; }
  pub fn get_type(&mut self) -> ChannelType { return self.ch_type }
  pub fn get_id(&mut self) -> usize { return self.id }

  pub async fn get_input(&mut self) -> Option<Arc<RwLock<cpal::Stream>>> { 
    let input: Arc<RwLock<cpal::Stream>> = Arc::clone(&self.input.as_mut().unwrap());
    return Some(input);
  }

  pub async fn set_input(&mut self, input: cpal::Stream) -> Result<(), ChannelStripError> {
    self.input = Some(Arc::new(RwLock::new(input)));

    let _ = self.run().await;
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

  pub fn set_status(&mut self, status: DASPStatus) {
    self.status = status;
  }

  pub fn set_id(&mut self, id: usize) -> Result<(), ChannelStripError> {
    if self.ch_type == ChannelType::USER {
      self.id = id;
      return Ok(())
    } else {
      return Err(ChannelStripError::SystemChannel)
    }
  }
}